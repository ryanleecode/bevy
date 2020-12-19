use crate::Rect;
use bevy_asset::Handle;
use bevy_core::Byteable;
use bevy_math::Vec2;
use bevy_reflect::TypeUuid;
use bevy_render::{
    color::Color,
    renderer::{RenderResource, RenderResources},
    texture::Texture,
};
use bevy_utils::HashMap;

/// An atlas containing multiple textures (like a spritesheet or a tilemap)
#[derive(Debug, RenderResources, TypeUuid)]
#[uuid = "946dacc5-c2b2-4b30-b81d-af77d79d1db7"]
pub struct TextureAtlas {
    /// The handle to the texture in which the sprites are stored
    pub texture: Handle<Texture>,
    // TODO: add support to Uniforms derive to write dimensions and sprites to the same buffer
    pub size: Vec2,
    /// The specific areas of the atlas where each texture can be found
    #[render_resources(buffer)]
    pub textures: Vec<Rect>,
    #[render_resources(ignore)]
    pub texture_handles: Option<HashMap<Handle<Texture>, usize>>,
}

#[derive(Debug, RenderResources, RenderResource)]
#[render_resources(from_self)]
pub struct TextureAtlasSprite {
    pub color: Color,
    pub index: u32,
}

impl Default for TextureAtlasSprite {
    fn default() -> Self {
        Self {
            index: 0,
            color: Color::WHITE,
        }
    }
}

unsafe impl Byteable for TextureAtlasSprite {}

impl TextureAtlasSprite {
    pub fn new(index: u32) -> TextureAtlasSprite {
        Self {
            index,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct PaddingSpecification {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

impl PaddingSpecification {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn uniform<T>(value: T) -> Self
    where
        T: Into<Vec2>,
    {
        let vec2: Vec2 = value.into();

        Self::new(vec2.x, vec2.y, vec2.x, vec2.y)
    }
}

impl From<Vec2> for PaddingSpecification {
    fn from(vec2: Vec2) -> Self {
        PaddingSpecification::uniform(vec2)
    }
}

impl TextureAtlas {
    /// Create a new `TextureAtlas` that has a texture, but does not have
    /// any individual sprites specified
    pub fn new_empty(texture: Handle<Texture>, dimensions: Vec2) -> Self {
        Self {
            texture,
            size: dimensions,
            texture_handles: None,
            textures: Vec::new(),
        }
    }

    /// Generate a `TextureAtlas` by splitting a texture into a grid where each
    /// cell of the grid  of `tile_size` is one of the textures in the atlas
    pub fn from_grid(
        texture: Handle<Texture>,
        tile_size: Vec2,
        texture_dimensions: Vec2,
    ) -> TextureAtlas {
        Self::from_grid_with_padding(
            texture,
            tile_size,
            texture_dimensions,
            PaddingSpecification::default(),
        )
    }

    /// Generate a `TextureAtlas` by splitting a texture into a grid where each
    /// cell of the grid of `tile_size` is one of the textures in the atlas and is separated by
    /// some `padding` in the texture
    pub fn from_grid_with_padding<PS>(
        texture: Handle<Texture>,
        tile_size: Vec2,
        texture_dimensions: Vec2,
        padding: PS,
    ) -> TextureAtlas
    where
        PS: Into<PaddingSpecification>,
    {
        let padding: PaddingSpecification = padding.into();
        let mut sprites = Vec::new();
        let x_padding = padding.left + padding.right;
        let y_padding = padding.top + padding.bottom;
        let rows = (texture_dimensions.x / (tile_size.x + x_padding)) as i32;
        let columns = (texture_dimensions.y / (tile_size.y + y_padding)) as i32;

        for y in 0..rows {
            for x in 0..columns {
                let rect_min = Vec2::new(
                    (tile_size.x + x_padding) * x as f32 + padding.left,
                    (tile_size.y + y_padding) * y as f32 + padding.top,
                );

                sprites.push(Rect {
                    min: rect_min,
                    max: Vec2::new(rect_min.x + tile_size.x, rect_min.y + tile_size.y),
                })
            }
        }

        TextureAtlas {
            size: texture_dimensions,
            textures: sprites,
            texture,
            texture_handles: None,
        }
    }

    /// Add a sprite to the list of textures in the `TextureAtlas`
    ///
    /// # Arguments
    ///
    /// * `rect` - The section of the atlas that contains the texture to be added,
    /// from the top-left corner of the texture to the bottom-right corner
    pub fn add_texture(&mut self, rect: Rect) {
        self.textures.push(rect);
    }

    /// How many textures are in the `TextureAtlas`
    pub fn len(&self) -> usize {
        self.textures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.textures.is_empty()
    }

    pub fn get_texture_index(&self, texture: &Handle<Texture>) -> Option<usize> {
        self.texture_handles
            .as_ref()
            .and_then(|texture_handles| texture_handles.get(texture).cloned())
    }
}

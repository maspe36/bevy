use crate::{Font, FontAtlasSet};
use bevy_asset::{Assets, Handle};
use bevy_render::{
    draw::{DrawError, Drawable},
    Color,
};
use bevy_sprite::TextureAtlas;
use glam::Vec2;

pub struct TextStyle {
    pub font_size: f32,
    pub color: Color,
}

#[allow(dead_code)]
pub struct DrawableText<'a> {
    font_handle: Handle<Font>,
    fonts: &'a Assets<Font>,
    font_atlas_sets: &'a Assets<FontAtlasSet>,
    texture_atlases: &'a Assets<TextureAtlas>,
    position: Vec2,
    style: &'a TextStyle,
    text: &'a str,
}

impl<'a> DrawableText<'a> {
    pub fn new(
        font_handle: Handle<Font>,
        fonts: &'a Assets<Font>,
        font_atlas_sets: &'a Assets<FontAtlasSet>,
        texture_atlases: &'a Assets<TextureAtlas>,
        position: Vec2,
        style: &'a TextStyle,
        text: &'a str,
    ) -> Self {
        Self {
            font_handle,
            fonts,
            font_atlas_sets,
            texture_atlases,
            position,
            style,
            text,
        }
    }
}

impl<'a> Drawable for DrawableText<'a> {
    fn draw(&mut self, _draw: &mut bevy_render::draw::DrawContext) -> Result<(), DrawError> {
        // draw.set_pipeline(bevy_sprite::SPRITE_SHEET_PIPELINE_HANDLE)?;
        // let render_resource_context = draw.render_resource_context;
        // // TODO: add draw.set_mesh(slot)
        // let quad_vertex_buffer = render_resource_context
        //     .get_asset_resource(bevy_sprite::QUAD_HANDLE, mesh::VERTEX_BUFFER_ASSET_INDEX)
        //     .unwrap();
        // let quad_index_buffer = render_resource_context
        //     .get_asset_resource(bevy_sprite::QUAD_HANDLE, mesh::INDEX_BUFFER_ASSET_INDEX)
        //     .unwrap();
        // draw.set_vertex_buffer(0, quad_vertex_buffer, 0);
        // draw.set_index_buffer(quad_index_buffer, 0);
        // draw.set_global_bind_groups()?;

        // // TODO: ideally the TexureAtlas bind group is automatically generated by AssetRenderResourcesNode and is retrievable
        // // here using render_resource_context.get_asset_render_resource_set(texture_atlas)
        // let mut atlas_set = RenderResourceSet::build()
        //     .add_assignment(0, draw.get_uniform_buffer(&10)?)
        //     .finish();
        Ok(())
    }
}
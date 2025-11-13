//! Texture atlas generation for voxel materials
//!
//! Creates a simple solid-color texture atlas with one color per voxel type.
//! Atlas layout: horizontal strip with 4 materials (Air, Grass, Dirt, Stone)

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// Atlas dimensions
const ATLAS_WIDTH: u32 = 256;   // 4 materials × 64 pixels each
const ATLAS_HEIGHT: u32 = 64;   // Single row
const MATERIAL_WIDTH: u32 = 64; // Width per material

/// Generate a simple solid-color texture atlas
pub fn generate_atlas() -> Image {
    let mut data = vec![0u8; (ATLAS_WIDTH * ATLAS_HEIGHT * 4) as usize]; // RGBA

    // Fill each material region with solid color
    fill_material_region(&mut data, 0, Color::srgba(0.0, 0.0, 0.0, 0.0)); // Air (transparent)
    fill_material_region(&mut data, 1, Color::srgb(0.3, 0.7, 0.2));        // Grass (green)
    fill_material_region(&mut data, 2, Color::srgb(0.5, 0.3, 0.1));        // Dirt (brown)
    fill_material_region(&mut data, 3, Color::srgb(0.5, 0.5, 0.5));        // Stone (gray)

    Image::new(
        Extent3d {
            width: ATLAS_WIDTH,
            height: ATLAS_HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    )
}

/// Fill a material region in the atlas with a solid color
fn fill_material_region(data: &mut [u8], material_index: u32, color: Color) {
    let start_x = material_index * MATERIAL_WIDTH;
    let end_x = start_x + MATERIAL_WIDTH;

    let [r, g, b, a] = color.to_srgba().to_u8_array();

    for y in 0..ATLAS_HEIGHT {
        for x in start_x..end_x {
            let pixel_index = ((y * ATLAS_WIDTH + x) * 4) as usize;
            data[pixel_index] = r;
            data[pixel_index + 1] = g;
            data[pixel_index + 2] = b;
            data[pixel_index + 3] = a;
        }
    }
}

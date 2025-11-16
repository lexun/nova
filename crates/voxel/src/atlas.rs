//! Texture atlas generation for voxel materials
//!
//! Creates procedurally generated textures for each voxel type.
//! Atlas layout: horizontal strip with 4 materials (Air, Grass, Dirt, Stone)

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// Atlas dimensions
const ATLAS_WIDTH: u32 = 256;   // 4 materials × 64 pixels each
const ATLAS_HEIGHT: u32 = 64;   // Single row
const MATERIAL_WIDTH: u32 = 64; // Width per material

/// Generate procedural texture atlas
pub fn generate_atlas() -> Image {
    let mut data = vec![0u8; (ATLAS_WIDTH * ATLAS_HEIGHT * 4) as usize]; // RGBA

    // Fill each material region with procedural texture
    fill_material_region(&mut data, 0); // Air (transparent)
    fill_material_region(&mut data, 1); // Grass (green)
    fill_material_region(&mut data, 2); // Dirt (brown)
    fill_material_region(&mut data, 3); // Stone (gray)

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

/// Fill a material region in the atlas with procedural texture
fn fill_material_region(data: &mut [u8], material_index: u32) {
    let start_x = material_index * MATERIAL_WIDTH;
    let end_x = start_x + MATERIAL_WIDTH;

    for y in 0..ATLAS_HEIGHT {
        for x in start_x..end_x {
            // Get local coordinates within this material region
            let local_x = x - start_x;
            let local_y = y;

            // Generate color based on material type
            let color = match material_index {
                0 => [0, 0, 0, 0], // Air (transparent)
                1 => generate_grass_pixel(local_x, local_y),
                2 => generate_dirt_pixel(local_x, local_y),
                3 => generate_stone_pixel(local_x, local_y),
                _ => [0, 0, 0, 255],
            };

            let pixel_index = ((y * ATLAS_WIDTH + x) * 4) as usize;
            data[pixel_index] = color[0];
            data[pixel_index + 1] = color[1];
            data[pixel_index + 2] = color[2];
            data[pixel_index + 3] = color[3];
        }
    }
}

/// Simple value noise function (deterministic)
fn noise(x: f32, y: f32, seed: u32) -> f32 {
    let n = (x * 12.9898 + y * 78.233 + seed as f32).sin() * 43758.5453;
    n.fract().abs()
}

/// Generate grass texture pixel
fn generate_grass_pixel(x: u32, y: u32) -> [u8; 4] {
    let fx = x as f32;
    let fy = y as f32;

    // Base green color
    let base_r = 0.3;
    let base_g = 0.6;
    let base_b = 0.2;

    // Add noise for variation
    let n1 = noise(fx * 0.2, fy * 0.2, 100);
    let n2 = noise(fx * 0.5, fy * 0.5, 200);

    // Create darker vertical streaks for grass blades
    let blade_pattern = (fx * 0.3).sin().abs();
    let blade = if blade_pattern < 0.2 { -0.1 } else { 0.0 };

    let r = (base_r + (n1 - 0.5) * 0.1 + blade).clamp(0.0, 1.0);
    let g = (base_g + (n1 - 0.5) * 0.1 + n2 * 0.05 + blade).clamp(0.0, 1.0);
    let b = (base_b + (n1 - 0.5) * 0.1 + blade).clamp(0.0, 1.0);

    [
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        255,
    ]
}

/// Generate dirt texture pixel
fn generate_dirt_pixel(x: u32, y: u32) -> [u8; 4] {
    let fx = x as f32;
    let fy = y as f32;

    // Base brown color
    let base_r = 0.5;
    let base_g = 0.35;
    let base_b = 0.2;

    // Multiple octaves of noise for grainy texture
    let n1 = noise(fx * 0.3, fy * 0.3, 300);
    let n2 = noise(fx * 0.8, fy * 0.8, 400);
    let n3 = noise(fx * 1.5, fy * 1.5, 500);

    // Combine noise for particle appearance
    let variation = (n1 * 0.5 + n2 * 0.3 + n3 * 0.2 - 0.5) * 0.2;

    let r = (base_r + variation).clamp(0.0, 1.0);
    let g = (base_g + variation).clamp(0.0, 1.0);
    let b = (base_b + variation).clamp(0.0, 1.0);

    [
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
        255,
    ]
}

/// Generate stone texture pixel
fn generate_stone_pixel(x: u32, y: u32) -> [u8; 4] {
    let fx = x as f32;
    let fy = y as f32;

    // Base gray color
    let base = 0.5;

    // Noise for general variation
    let n1 = noise(fx * 0.2, fy * 0.2, 600);
    let n2 = noise(fx * 0.6, fy * 0.6, 700);

    // Create crack patterns - rare dark lines
    let crack1 = noise(fx * 0.15, fy * 0.8, 800);
    let crack2 = noise(fx * 0.8, fy * 0.15, 900);
    let is_crack = crack1 > 0.92 || crack2 > 0.92;

    let variation = if is_crack {
        -0.2 // Dark cracks
    } else {
        (n1 * 0.4 + n2 * 0.2 - 0.3) * 0.15 // Subtle variation
    };

    let value = (base + variation).clamp(0.0, 1.0);

    [
        (value * 255.0) as u8,
        (value * 255.0) as u8,
        (value * 255.0) as u8,
        255,
    ]
}

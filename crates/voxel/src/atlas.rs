//! Texture atlas generation for voxel materials
//!
//! Creates procedurally generated textures for each voxel type.
//! Atlas layout: horizontal strip with 5 texture regions:
//! [Air | Grass Top | Dirt | Stone | Grass Side]

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use crate::VoxelType;

/// Atlas texture regions
/// Each region occupies 64px width in a 320px horizontal atlas
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtlasRegion {
    Air = 0,
    GrassTop = 1,
    Dirt = 2,
    Stone = 3,
    GrassSide = 4,
}

impl AtlasRegion {
    /// Get U coordinate offset for this atlas region (0.0 to 1.0 range)
    pub fn u_offset(self) -> f32 {
        const REGION_WIDTH: f32 = 1.0 / 5.0; // 5 regions in atlas
        (self as u32) as f32 * REGION_WIDTH
    }

    /// Get atlas region width in UV space (0.0 to 1.0 range)
    pub fn region_width() -> f32 {
        1.0 / 5.0
    }
}

/// Face direction for texture selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FaceDir {
    Top,    // Y+
    Bottom, // Y-
    Side,   // X+, X-, Z+, Z-
}

/// Determine which atlas region to use for a voxel face
pub fn get_atlas_region(voxel_type: VoxelType, face_dir: FaceDir) -> AtlasRegion {
    match voxel_type {
        VoxelType::Air => AtlasRegion::Air,
        VoxelType::Stone => AtlasRegion::Stone,
        VoxelType::Dirt => AtlasRegion::Dirt,
        VoxelType::Grass => match face_dir {
            FaceDir::Top => AtlasRegion::GrassTop,
            FaceDir::Bottom => AtlasRegion::Dirt,
            FaceDir::Side => AtlasRegion::GrassSide,
        },
    }
}

/// Atlas dimensions
const ATLAS_WIDTH: u32 = 320;   // 5 materials × 64 pixels each
const ATLAS_HEIGHT: u32 = 64;   // Single row
const MATERIAL_WIDTH: u32 = 64; // Width per material

/// Generate procedural texture atlas
pub fn generate_atlas() -> Image {
    let mut data = vec![0u8; (ATLAS_WIDTH * ATLAS_HEIGHT * 4) as usize]; // RGBA

    // Fill each material region with procedural texture
    fill_material_region(&mut data, 0); // Air (transparent)
    fill_material_region(&mut data, 1); // Grass top (green)
    fill_material_region(&mut data, 2); // Dirt (brown)
    fill_material_region(&mut data, 3); // Stone (gray)
    fill_material_region(&mut data, 4); // Grass side (dirt + grass transition)

    let mut image = Image::new(
        Extent3d {
            width: ATLAS_WIDTH,
            height: ATLAS_HEIGHT,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    // Use Repeat mode to enable texture tiling
    // WARNING: UVs must stay within atlas regions or will wrap to wrong materials!
    use bevy::image::{ImageSamplerDescriptor, ImageAddressMode, ImageSampler};
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        ..default()
    });

    image
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
                4 => generate_grass_side_pixel(local_x, local_y),
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
/// Designed for 64×64 texture representing 0.25m (smallest voxel)
/// Noisy green texture without individual blades for natural look at distance
fn generate_grass_pixel(x: u32, y: u32) -> [u8; 4] {
    let fx = x as f32;
    let fy = y as f32;

    // Base green color
    let base_r = 0.3;
    let base_g = 0.6;
    let base_b = 0.2;

    // Multiple octaves of noise for natural variation
    let n1 = noise(fx * 0.3, fy * 0.3, 100);
    let n2 = noise(fx * 0.8, fy * 0.8, 200);
    let n3 = noise(fx * 1.5, fy * 1.5, 300);

    // Combine noise for natural grass appearance
    let variation = (n1 * 0.5 + n2 * 0.3 + n3 * 0.2 - 0.5) * 0.15;

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
/// Designed for 64×64 texture representing 0.25m (smallest voxel)
/// Simple 8×8 checkerboard to verify UV mapping
fn generate_stone_pixel(x: u32, y: u32) -> [u8; 4] {
    // Create 8×8 checkerboard (each square is 8×8 pixels)
    let cell_size = 8;
    let cell_x = x / cell_size;
    let cell_y = y / cell_size;

    let is_white = (cell_x + cell_y) % 2 == 0;

    let value = if is_white { 200 } else { 80 };

    [value, value, value, 255]
}

/// Generate grass side texture pixel
/// Minecraft-style: dirt texture with grass transition in top quarter
fn generate_grass_side_pixel(x: u32, y: u32) -> [u8; 4] {
    let fx = x as f32;
    let fy = y as f32;

    // Transition happens in top 16 pixels (top quarter of 64px texture)
    let grass_transition_height = 16.0;

    if fy < grass_transition_height {
        // Top quarter: blend from dirt to grass
        let blend = fy / grass_transition_height;

        // Grass color (top)
        let grass_r = 0.3;
        let grass_g = 0.6;
        let grass_b = 0.2;

        // Dirt color (bottom)
        let dirt_r = 0.5;
        let dirt_g = 0.35;
        let dirt_b = 0.2;

        // Blend between grass (at top) and dirt (at transition line)
        let r = grass_r + (dirt_r - grass_r) * blend;
        let g = grass_g + (dirt_g - grass_g) * blend;
        let b = grass_b + (dirt_b - grass_b) * blend;

        // Add noise for texture variation
        let n1 = noise(fx * 0.3, fy * 0.3, 600);
        let variation = (n1 - 0.5) * 0.1;

        [
            ((r + variation).clamp(0.0, 1.0) * 255.0) as u8,
            ((g + variation).clamp(0.0, 1.0) * 255.0) as u8,
            ((b + variation).clamp(0.0, 1.0) * 255.0) as u8,
            255,
        ]
    } else {
        // Bottom 3/4: pure dirt texture
        generate_dirt_pixel(x, y)
    }
}

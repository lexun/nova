//! Individual texture generation for voxel materials
//!
//! Generates separate 64×64 textures for each material type.
//! This approach uses GPU Repeat mode without atlas limitations,
//! allowing proper texture tiling on greedy-meshed quads.
//!
//! This is an interim solution until texture arrays are available (blocked by upstream Bevy bug).

use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

/// Texture dimensions - each texture is 64×64 pixels
const TEXTURE_SIZE: u32 = 64;

/// Generate all voxel material textures
/// Returns handles for: (air, grass_top, dirt, stone, grass_side)
pub fn generate_material_textures(images: &mut Assets<Image>) -> MaterialTextures {
    MaterialTextures {
        air: images.add(generate_air_texture()),
        grass_top: images.add(generate_grass_top_texture()),
        dirt: images.add(generate_dirt_texture()),
        stone: images.add(generate_stone_texture()),
        grass_side: images.add(generate_grass_side_texture()),
    }
}

/// Material texture handles
#[derive(Clone)]
pub struct MaterialTextures {
    pub air: Handle<Image>,
    pub grass_top: Handle<Image>,
    pub dirt: Handle<Image>,
    pub stone: Handle<Image>,
    pub grass_side: Handle<Image>,
}

impl MaterialTextures {
    /// Get texture handle for a voxel type and face direction
    pub fn get_texture(&self, voxel_type: crate::VoxelType, face_dir: crate::atlas::FaceDir) -> Handle<Image> {
        use crate::VoxelType::*;
        use crate::atlas::FaceDir;
        match voxel_type {
            Air => self.air.clone(),
            Stone => self.stone.clone(),
            Dirt => self.dirt.clone(),
            Grass => match face_dir {
                FaceDir::Top => self.grass_top.clone(),
                FaceDir::Bottom => self.dirt.clone(),
                FaceDir::Side => self.grass_side.clone(),
            },
        }
    }
}

/// Generate transparent air texture
fn generate_air_texture() -> Image {
    let data = vec![0u8; (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize];
    // All pixels transparent
    let mut image = Image::new(
        Extent3d {
            width: TEXTURE_SIZE,
            height: TEXTURE_SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    // Enable Repeat mode for tiling
    use bevy::image::{ImageSamplerDescriptor, ImageAddressMode, ImageSampler};
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        ..default()
    });

    image
}

/// Generate grass top texture - noisy green
fn generate_grass_top_texture() -> Image {
    let mut data = vec![0u8; (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize];

    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let color = generate_grass_pixel(x, y);
            let index = ((y * TEXTURE_SIZE + x) * 4) as usize;
            data[index..index + 4].copy_from_slice(&color);
        }
    }

    let mut image = Image::new(
        Extent3d {
            width: TEXTURE_SIZE,
            height: TEXTURE_SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    // Enable Repeat mode for tiling
    use bevy::image::{ImageSamplerDescriptor, ImageAddressMode, ImageSampler};
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        ..default()
    });

    image
}

/// Generate dirt texture
fn generate_dirt_texture() -> Image {
    let mut data = vec![0u8; (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize];

    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let color = generate_dirt_pixel(x, y);
            let index = ((y * TEXTURE_SIZE + x) * 4) as usize;
            data[index..index + 4].copy_from_slice(&color);
        }
    }

    let mut image = Image::new(
        Extent3d {
            width: TEXTURE_SIZE,
            height: TEXTURE_SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    // Enable Repeat mode for tiling
    use bevy::image::{ImageSamplerDescriptor, ImageAddressMode, ImageSampler};
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        ..default()
    });

    image
}

/// Generate stone texture
fn generate_stone_texture() -> Image {
    let mut data = vec![0u8; (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize];

    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let color = generate_stone_pixel(x, y);
            let index = ((y * TEXTURE_SIZE + x) * 4) as usize;
            data[index..index + 4].copy_from_slice(&color);
        }
    }

    let mut image = Image::new(
        Extent3d {
            width: TEXTURE_SIZE,
            height: TEXTURE_SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    // Enable Repeat mode for tiling
    use bevy::image::{ImageSamplerDescriptor, ImageAddressMode, ImageSampler};
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        ..default()
    });

    image
}

/// Generate grass side texture - dirt with grass transition
fn generate_grass_side_texture() -> Image {
    let mut data = vec![0u8; (TEXTURE_SIZE * TEXTURE_SIZE * 4) as usize];

    for y in 0..TEXTURE_SIZE {
        for x in 0..TEXTURE_SIZE {
            let color = generate_grass_side_pixel(x, y);
            let index = ((y * TEXTURE_SIZE + x) * 4) as usize;
            data[index..index + 4].copy_from_slice(&color);
        }
    }

    let mut image = Image::new(
        Extent3d {
            width: TEXTURE_SIZE,
            height: TEXTURE_SIZE,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );

    // Enable Repeat mode for tiling
    use bevy::image::{ImageSamplerDescriptor, ImageAddressMode, ImageSampler};
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        ..default()
    });

    image
}

// Pixel generation functions (copy from atlas.rs)

fn noise(x: f32, y: f32, seed: u32) -> f32 {
    let n = (x * 12.9898 + y * 78.233 + seed as f32).sin() * 43758.5453;
    n.fract().abs()
}

fn generate_grass_pixel(x: u32, y: u32) -> [u8; 4] {
    let fx = x as f32;
    let fy = y as f32;

    let base_r = 0.3;
    let base_g = 0.6;
    let base_b = 0.2;

    let n1 = noise(fx * 0.3, fy * 0.3, 100);
    let n2 = noise(fx * 0.8, fy * 0.8, 200);
    let n3 = noise(fx * 1.5, fy * 1.5, 300);

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

fn generate_dirt_pixel(x: u32, y: u32) -> [u8; 4] {
    let fx = x as f32;
    let fy = y as f32;

    let base_r = 0.5;
    let base_g = 0.35;
    let base_b = 0.2;

    let n1 = noise(fx * 0.2, fy * 0.2, 200);
    let n2 = noise(fx * 0.6, fy * 0.6, 300);

    let variation = (n1 * 0.6 + n2 * 0.4 - 0.5) * 0.15;

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

fn generate_stone_pixel(x: u32, y: u32) -> [u8; 4] {
    let cell_size = 8;
    let cell_x = x / cell_size;
    let cell_y = y / cell_size;

    let is_white = (cell_x + cell_y) % 2 == 0;
    let value = if is_white { 200 } else { 80 };

    [value, value, value, 255]
}

fn generate_grass_side_pixel(x: u32, y: u32) -> [u8; 4] {
    let fx = x as f32;
    let fy = y as f32;

    let grass_transition_height = 16.0;

    if fy < grass_transition_height {
        let blend = fy / grass_transition_height;

        let grass_r = 0.3;
        let grass_g = 0.6;
        let grass_b = 0.2;

        let dirt_r = 0.5;
        let dirt_g = 0.35;
        let dirt_b = 0.2;

        let r = grass_r + (dirt_r - grass_r) * blend;
        let g = grass_g + (dirt_g - grass_g) * blend;
        let b = grass_b + (dirt_b - grass_b) * blend;

        let n1 = noise(fx * 0.3, fy * 0.3, 600);
        let variation = (n1 - 0.5) * 0.1;

        [
            ((r + variation).clamp(0.0, 1.0) * 255.0) as u8,
            ((g + variation).clamp(0.0, 1.0) * 255.0) as u8,
            ((b + variation).clamp(0.0, 1.0) * 255.0) as u8,
            255,
        ]
    } else {
        generate_dirt_pixel(x, y)
    }
}

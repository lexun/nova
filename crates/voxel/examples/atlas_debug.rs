//! Debug: Save the generated atlas to a file to inspect it
//!
//! This creates the atlas texture and immediately saves it, then exits.

use bevy::prelude::*;
use voxel::atlas::generate_atlas;

fn main() {
    let atlas = generate_atlas();

    // Save the atlas to /tmp for inspection
    atlas.clone().try_into_dynamic()
        .expect("Failed to convert to dynamic")
        .save("/tmp/atlas_debug.png")
        .expect("Failed to save atlas");

    println!("Atlas saved to /tmp/atlas_debug.png");
    println!("Atlas dimensions: {}x{}", atlas.texture_descriptor.size.width, atlas.texture_descriptor.size.height);
    println!("Format: {:?}", atlas.texture_descriptor.format);
}

//! Dump the texture atlas to a file to verify layout

fn main() {
    let atlas_image = voxel::atlas::generate_atlas();

    println!("Atlas dimensions: {}x{}", atlas_image.width(), atlas_image.height());
    println!("Expected layout: [Air | GrassTop | Dirt | Stone | GrassSide]");
    println!("Each region is 64px wide in a 320px atlas");

    // Save to PNG
    atlas_image.try_into_dynamic().unwrap()
        .save("/tmp/atlas_dump.png").unwrap();

    println!("Atlas saved to /tmp/atlas_dump.png");
}

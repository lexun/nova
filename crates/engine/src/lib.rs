use bevy::prelude::*;

pub mod camera;

pub use camera::{FlyCameraController, FlyCameraPlugin};

pub fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app
}

pub fn run_app() {
    create_app().run();
}

pub mod window;
pub mod render;
pub mod widget;
pub mod theme;
pub mod input;
pub mod compositor;

pub use window::WindowManager;
pub use render::Renderer;
pub use compositor::Compositor;

pub fn init() {
    println!("Initializing {}", "tjorn-gui");
}

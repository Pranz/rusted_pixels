
use sdl2::render::Renderer;
use sdl2_ttf::Font;

use state::State;

pub mod drawing_window;
pub mod preview_window;
pub mod palette_window;
pub mod status_window;

pub use self::drawing_window::DrawingWindow;
pub use self::preview_window::PreviewWindow;
pub use self::palette_window::PaletteWindow;
pub use self::status_window::StatusWindow;

/*
 * Any sort of window that displays, or handles mouse input.
 * Keyboard input should be handled separately.
 *
 * Examples: Color select, tool select, image windows.
 */
pub trait Window {
    fn draw<'a>(&self, renderer: &mut Renderer<'a>, font: &mut Font, state: &State);
    fn handle_mouse_down(&self, state: &mut State, mouse_x:
                         i32, mouse_y: i32);
    fn increment_scale(&mut self) { }
    fn decrement_scale(&mut self) { }
}

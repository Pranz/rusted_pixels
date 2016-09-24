use sdl2::render::Renderer;

use state::State;
use windows::Window;
use windows::DrawingWindow;

/*
 * Just a wrapper around drawing window, that discard it's 
 * `handle_mouse_down` method
 */
pub struct PreviewWindow(pub DrawingWindow);

impl Window for PreviewWindow {
    
    
    fn draw<'a>(&self, renderer: &mut Renderer<'a>, state: &State) {
        match self {
            &PreviewWindow(ref window) => { window.draw(renderer, state); }
        }
    }

    fn handle_mouse_down(&self, _: &mut State,
                         _: i32, _: i32) {
        // intentionally left blank
    }

}

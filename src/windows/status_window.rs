use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::*;
use sdl2_ttf::Font;

use state::State;
use windows::Window; 

pub struct StatusWindow {
    height: u32,
    background_color: Color,
    font_color: Color,
}

impl StatusWindow {
    pub fn new() -> StatusWindow {
        StatusWindow {
            height: 40,
            background_color: Color::RGB(240, 210, 120),
            font_color: Color::RGB(40, 40, 40),
        }
    }
}

impl Window for StatusWindow {
    fn handle_mouse_down(&self, _: &mut State, _: i32, _: i32) {

    }

    fn draw<'a>(&self, renderer: &mut Renderer<'a>, font: &mut Font, state: &State) {
        let (window_width, window_height) = renderer.window().unwrap().size();
        let rect = Rect::new(
            0,
            (window_height - self.height) as i32,
            window_width,
            self.height);
        
        renderer.set_draw_color(self.background_color);
        renderer.fill_rect(rect).ok();

        let text = state.show_input_stack();
        let surface = font.render(&text)
            .blended(self.font_color).unwrap();
        let mut texture = renderer.create_texture_from_surface(&surface).unwrap();

        let (font_width, font_height) = font.size_of(&text).unwrap();
        let font_rect = Rect::new(
            0,
            (window_height - self.height) as i32,
            font_width / 8, 
            font_height / 8);

        renderer.copy(&mut texture, None, Some(font_rect));
        renderer.present();
    }
}

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
        let surface = font.render("Hello world!")
            .blended(self.font_color).unwrap();
        let mut texture = renderer.create_texture_from_surface(&surface).unwrap();

        renderer.copy(&mut texture, None, Some(rect));
        renderer.present();
    }
}

use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::*;

use state::State;
use windows::Window; 

pub struct PaletteWindow{
    pub x: usize,
    pub y: usize,
    pub palette_id: usize,
}

const PALETTEWINDOW_COLOR_SQUARE_PX: usize = 24;
const PALETTEWINDOW_COLORS_PER_ROW: usize = 3;
const PALETTEWINDOW_PADDING_PX: usize = 3;

impl Window for PaletteWindow{
    fn handle_mouse_down(&self, state: &mut State,
                             mouse_x: i32, mouse_y: i32) {
        let palette_x1 = self.x+PALETTEWINDOW_PADDING_PX;
        let palette_x2 = self.x+PALETTEWINDOW_PADDING_PX + PALETTEWINDOW_COLORS_PER_ROW*PALETTEWINDOW_COLOR_SQUARE_PX;
        let palette_y1 = self.y+PALETTEWINDOW_PADDING_PX;
        let palette_y2 = self.y+PALETTEWINDOW_PADDING_PX + (state.current_palette().len() as f32/PALETTEWINDOW_COLORS_PER_ROW as f32).ceil() as usize*PALETTEWINDOW_COLOR_SQUARE_PX;

        if mouse_x as usize>=palette_x1 && mouse_x as usize<=palette_x2
        && mouse_y as usize>=palette_y1 && mouse_y as usize<=palette_y2{
            let palette_x = (mouse_x as usize-palette_x1)/PALETTEWINDOW_COLOR_SQUARE_PX;
            let palette_y = (mouse_y as usize-palette_y1)/PALETTEWINDOW_COLOR_SQUARE_PX;
            let palette_id = palette_x%PALETTEWINDOW_COLORS_PER_ROW + palette_y*PALETTEWINDOW_COLORS_PER_ROW;

            state.current_color = if let Some(color) = state.current_palette().get(palette_id) {
                *color
            } else { state.current_color }
        }
    }

    fn draw<'a>(&self, renderer: &mut Renderer<'a>, state: &State) {
        renderer.set_draw_color(Color::RGB(48,48,48));
        renderer.fill_rect(Rect::new(
            self.x as i32,
            self.y as i32,
            (PALETTEWINDOW_COLORS_PER_ROW * PALETTEWINDOW_COLOR_SQUARE_PX + 2*PALETTEWINDOW_PADDING_PX) as u32,
            ((state.current_palette().len() as f32 / PALETTEWINDOW_COLORS_PER_ROW as f32).ceil() as usize * PALETTEWINDOW_COLOR_SQUARE_PX + 2*PALETTEWINDOW_PADDING_PX) as u32,
        )).ok();

        for (i,color) in state.current_palette().iter().enumerate(){
            renderer.set_draw_color(*color);
            renderer.fill_rect(Rect::new((PALETTEWINDOW_PADDING_PX as usize + self.x + (i%PALETTEWINDOW_COLORS_PER_ROW as usize)*PALETTEWINDOW_COLOR_SQUARE_PX as usize) as i32,
                                         (PALETTEWINDOW_PADDING_PX as usize + self.y + (i/PALETTEWINDOW_COLORS_PER_ROW as usize)*PALETTEWINDOW_COLOR_SQUARE_PX as usize) as i32,
                                         PALETTEWINDOW_COLOR_SQUARE_PX as u32,
                                         PALETTEWINDOW_COLOR_SQUARE_PX as u32)).ok();
        }
    }
}

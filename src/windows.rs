
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::*;

use image_buffer::ImageBuffer;
use state::State;

/*
 * Any sort of window that displays, or handles mouse input.
 * Keyboard input should be handled separately.
 *
 * Examples: Color select, tool select, image windows.
 */
pub trait Window {
    fn draw<'a>(&self, renderer: &mut Renderer<'a>, state: &State);
    fn handle_mouse_down(&self, state: &mut State, mouse_x:
                         i32, mouse_y: i32); 
}

/*
 * A window that draws out an image and lets the user edit 
 * said image.
 */
pub struct DrawingWindow {
    pub x: usize,
    pub y: usize,
    pub scale: usize,
    pub background: Color,
    pub image_id: usize,
}

impl DrawingWindow {

    #[inline(always)]
    pub fn new(x: usize, y: usize, scale: usize, background: Color,
               image_id: usize)
               -> DrawingWindow
    {
        DrawingWindow {
            x: x,
            y: y,
            scale: scale,
            background: background,
            image_id: image_id,
        }
    }
    
    

    pub fn in_range(&self, image: &ImageBuffer, x: i32, y: i32) -> bool {
        /*
         * Test whether a window rendering `image` contains the points
         * `x` and `y`
         */
 
        let (win_x, win_y, scale) =
            (self.x as i32, self.y as i32, self.scale as i32);
        (win_x <= x
         && x < (win_x + image.width as i32 * scale)
         && win_y <= y
         && y < (win_y + image.height as i32 * scale))
    }

    pub fn get_index(&self, image: &ImageBuffer, x: i32, y: i32) ->
        Option<(i32, i32)>
    {
        /*
         * A window rendering `image`, check what index the absolute 
         * points `x` and `y` corresponds to. For example, if a mouse clicks
         * occurs at point `(x,y)`, which image pixel was targeted?
         */
        if self.in_range(image, x, y) {
            let (win_x, win_y, scale) =
                (self.x as i32, self.y as i32, self.scale as i32);
            Some(((x - win_x) / scale, (y - win_y) / scale))
        }
        else { None }
    }

    
}

impl Window for DrawingWindow {
    fn handle_mouse_down(&self, state: &mut State,
                             mouse_x: i32, mouse_y: i32) {
        let image = &mut state.images[self.image_id];
        let coordinates = self.get_index(&image, mouse_x, mouse_y);
        if let Some((x,y)) = coordinates {
            *image.get_mut_ref(x as usize,y as usize) = state.current_color; 
        }
    }

    fn draw<'a>(&self, renderer: &mut Renderer<'a>, state: &State) {
        let image = &state.images[self.image_id];
        
        renderer.set_draw_color(self.background);
        renderer.fill_rect(Rect::new(
            self.x as i32,
            self.y as i32,
            (image.width * self.scale) as u32,
            (image.height * self.scale) as u32)).ok();
        
        for x in 0..image.width {
            for y in 0..image.height {
                renderer.set_draw_color(image.get_point(x,y));
                let (x,y) = (x * self.scale, y * self.scale);
                renderer.fill_rect(Rect::new(self.x as i32 + x as i32,
                                             self.y as i32 + y as i32,
                                             self.scale as u32,
                                             self.scale as u32)).ok();
            }
        }
    }
}

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
        let palette_y2 = self.y+PALETTEWINDOW_PADDING_PX + (state.palettes.len() as f32/PALETTEWINDOW_COLORS_PER_ROW as f32).ceil() as usize*PALETTEWINDOW_COLOR_SQUARE_PX;

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

use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::*;
use sdl2_ttf::Font;

use image_buffer::ImageBuffer;
use state::{State, DrawUndo};
use windows::Window; 

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
        Option<(usize, usize)>
    {
        /*
         * A window rendering `image`, check what index the absolute 
         * points `x` and `y` corresponds to. For example, if a mouse clicks
         * occurs at point `(x,y)`, which image pixel was targeted?
         */
        if self.in_range(image, x, y) && x >= 0 && y >= 0 {
            let (win_x, win_y, scale) =
                (self.x as i32, self.y as i32, self.scale as i32);
            let index_x = ((x - win_x) / scale) as usize;
            let index_y = ((y - win_y) / scale) as usize;
            Some((index_x, index_y))
        }
        else { None }
    }

    pub fn save_undo(&self, state: &mut State, target_x: usize, target_y: usize, color: Color) {
        if let Some(undo) = state.undo_stack.last_mut() {
            let has_previous_undo = undo
                .draw_undo
                .iter()
                .any(|&DrawUndo {image_id, x, y, color}| {
                    x == target_x && y == target_y
                });
            if !has_previous_undo {
                undo.draw_undo.push(DrawUndo::new(
                    self.image_id,
                    target_x,
                    target_y,
                    state.images[self.image_id].get_point(target_x, target_y)));
            }
        }
    }
}

impl Window for DrawingWindow {
    fn handle_mouse_down(&self, state: &mut State,
                             mouse_x: i32, mouse_y: i32) {
        let coordinates = self.get_index(&state.images[self.image_id], mouse_x, mouse_y);
        if let Some((x,y)) = coordinates {
            let color = state.images[self.image_id].get_point(x,y);
            self.save_undo(state, x, y, color);
            *state.images[self.image_id].get_mut_ref(x, y) = state.current_color; 
        }
    }

    fn draw<'a>(&self, renderer: &mut Renderer<'a>, _: &mut Font, state: &State) {
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

    fn increment_scale(&mut self) {
        self.scale = (self.scale + 1) % 32;
    }

    fn decrement_scale(&mut self) {
        self.scale = ((self.scale as isize + 31) % 32) as usize;
    }
}

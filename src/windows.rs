
use sdl2::render::Renderer;
use sdl2::pixels::Color;
use sdl2::rect::*;

use image_buffer::ImageBuffer;

pub struct DrawingWindow {
    pub x: usize,
    pub y: usize,
    pub scale: usize,
    pub background: Color,
}

impl DrawingWindow {

    #[inline(always)]
    pub fn new(x: usize, y: usize, scale: usize, background: Color)
               -> DrawingWindow
    {
        DrawingWindow {
            x: x,
            y: y,
            scale: scale,
            background: background,
        }
    }
    
    pub fn draw<'a>(&self, renderer: &mut Renderer<'a>, image: &ImageBuffer) {
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

    pub fn in_range(&self, image: &ImageBuffer, x: i32, y: i32) -> bool {
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
        if self.in_range(image, x, y) {
            let (win_x, win_y, scale) =
                (self.x as i32, self.y as i32, self.scale as i32);
            Some(((x - win_x) / scale, (y - win_y) / scale))
        }
        else { None }
    }
}

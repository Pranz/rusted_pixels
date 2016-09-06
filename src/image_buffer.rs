
use sdl2::pixels::Color;

pub struct ImageBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize) -> ImageBuffer {
        let mut buffer = Vec::with_capacity(width * height);
        
        for _ in 0..(width * height) {
            buffer.push(Color::RGBA(0,0,0,0));
        }
        
        ImageBuffer {
            width: width,
            height: height,
            buffer: buffer,
        }
    }

    #[inline(always)]
    pub fn get_point(&self, x: usize, y: usize) -> Color {
        self.buffer[y * self.width + x]
    }

    #[inline(always)]
    pub fn get_mut_ref<'a>(&'a mut self, x: usize, y: usize) -> &'a mut Color {
        &mut self.buffer[y* self.width + x]
    }
}

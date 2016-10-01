
use sdl2::pixels::Color;
use sdl2::keyboard::{Keycode,Mod,LALTMOD,LCTRLMOD};
use image_buffer::ImageBuffer;
use input::{Input, Arg, keycode_to_char};

/*
 * Holds the main state, pretty self explanatory.
 */
pub struct State {
    pub current_color: Color,
    pub left_mouse_down: bool,
    pub right_mouse_down: bool,
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub images: Vec<ImageBuffer>,
    pub current_palette_index: usize,
    pub palettes: Vec<Vec<Color>>,//TODO: Multiple palettes
    pub input: Vec<Input>,
    pub args: Vec<Arg>,
    pub input_buffer: String,
    pub window_index: usize,
    pub undo_stack: Vec<Undo>,
}

/*
 * Should contain every information needed for an undo.
 */
pub struct Undo {
    pub draw_undo: Vec<DrawUndo>,
}

impl Undo {
    pub fn new() -> Undo {
        Undo {
            draw_undo: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.draw_undo.is_empty()
    }
}

pub struct DrawUndo {
    pub image_id: usize,
    pub x: usize,
    pub y: usize,
    pub color: Color,
}

impl DrawUndo {
    pub fn new(image_id: usize, x: usize, y: usize, color: Color) -> Self {
        DrawUndo {
            image_id: image_id,
            x: x,
            y: y,
            color: color,
        }
    }
}

impl State {
    pub fn new() -> Self {
        State {
            current_color: Color::RGB(255,255,255),
            left_mouse_down: false,
            right_mouse_down: false,
            mouse_x: 0,
            mouse_y: 0,
            images: vec![],
            current_palette_index: 0,
            palettes: vec![vec![
                Color::RGB(0,0,0),
                Color::RGB(128,128,128),
                Color::RGB(255,255,255),
                Color::RGB(192,128,112),
            ]],
            input: Vec::new(),
            args: Vec::new(),
            input_buffer: String::new(),
            window_index: 0,
            undo_stack: vec![],
        }
    }

    #[inline(always)]
    pub fn current_palette<'a>(&'a self) -> &'a [Color] {
        &self.palettes[self.current_palette_index]
    }

    pub fn undo(&mut self) {
        if let Some(undo) = self.undo_stack.pop() {
            for DrawUndo {image_id, x, y, color} in undo.draw_undo.into_iter() {
                let image = &mut self.images[image_id];
                *image.get_mut_ref(x, y) = color;
            }
        }
    }

    pub fn show_input_stack(&self) -> String {
        fn mod_to_string(modifier: Mod) -> &'static str {
            match modifier {
                LALTMOD => "M-",
                LCTRLMOD => "C-",
                _ => "",
            }
        }
        
        let mut string = String::new();
        let mut count = 0;
        for input in &self.input {
            match *input {
                Input::Char(keycode, modifier) => {
                    string = string + mod_to_string(modifier);
                    string.push(keycode_to_char(keycode).unwrap_or(' '));
                },
                Input::Integer => {
                    string = string + &self.args[count].coerce_integer().to_string();
                    count += 1;
                },
                Input::Exact(ref exact_phrase) => {
                    string = string + exact_phrase;
                },
                Input::String => {
                    string = string + &self.args[count].coerce_string();
                    count += 1;
                },
                Input::Color => {
                    let color = &self.args[count].coerce_color();
                    if let Color::RGB(r,g,b) = *color {
                        string = string + "rgb(" + &r.to_string() + ","
                            + &g.to_string() + ","
                            + &b.to_string() + ")"
                    } else if let Color::RGBA(r,g,b,a) = *color {
                        string = string + "rgb(" + &r.to_string()
                            + "," + &g.to_string() + ","
                            + &b.to_string() + "," + &a.to_string() + ")"
                    }
                    count += 1;
                }
            }
            string = string + " ";
        }
        string = string + " " + &self.input_buffer;
        return string;
    }
}

use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use state::State;

#[derive(PartialEq)]
pub enum ExtendedChar {
    NonModified(Keycode),
    CtrlModified(Keycode),
    AltModified(Keycode),
}

/*
 * Veeery emacs inspired. Basically a emacs-like commando like
 * `M-x swap-color RET color1 color2`
 * would be translated in code as
 * &[AltModified(char), Exact("swap-color"), Color, Color]
 */

#[derive(PartialEq)]
pub enum Input {
    Char(ExtendedChar),
    Integer,
    Color,
    String,
    Exact(String)
}

pub enum Arg {
    String(String),
    Integer(isize),
    Color(Color),
}

impl Arg {

}

#[derive(Copy, Clone)]
pub enum Command {
    ExportPng,
    Quit,
}

pub fn get_commands() -> Vec<(Vec<Input>, Command)> {
    vec![(vec![Input::Char(ExtendedChar::CtrlModified(Keycode::S))],
          Command::ExportPng),
         (vec![Input::Char(ExtendedChar::AltModified(Keycode::X)),
               Input::Exact(String::from("export-png"))],
          Command::ExportPng),
         (vec![Input::Char(ExtendedChar::CtrlModified(Keycode::Q))],
          Command::Quit),
         (vec![Input::Char(ExtendedChar::AltModified(Keycode::X)),
               Input::Exact(String::from("quit"))],
          Command::Quit)
    ]
}


pub enum InterpretErr {
    NoValidCommand,
    RequiresMoreInput
}


pub fn interpret_input(input: &[Input],
                       commands: &[(Vec<Input>, Command)])
                                   -> Result<Command, InterpretErr> {
    let mut has_match = false;
    for &(ref inputstack, command) in commands {
        if input.len() <= inputstack.len() &&
            input == &inputstack[0..input.len()] {
            has_match = true;
            if input == inputstack.as_slice() {
                return Ok(command);
            }
        }
    }
    match has_match {
        true => Err(InterpretErr::RequiresMoreInput),
        false => Err(InterpretErr::NoValidCommand)
    }
}

pub enum CommandResult {
    Quit,
    RequiresMoreInput,
    NoValidCommand,
    Success,
}

pub fn execute_command(state: &mut State,
                       commands: &[(Vec<Input>, Command)])
-> CommandResult {
    pub fn clean_input_and_args(state: &mut State) {
        state.args = Vec::new();
        state.input = Vec::new();
    }
    
    match interpret_input(&state.input, commands) {
        Ok(command) => match command {
            Command::ExportPng => {
                state.images[0].save_png_image("test_out.png").unwrap();
                println!("exported png");
                clean_input_and_args(state);
                CommandResult::Success
            },
            Command::Quit => {
                println!("quit succesfully");
                CommandResult::Quit
            },
        },
        Err(InterpretErr::NoValidCommand) => {
            clean_input_and_args(state);
            CommandResult::NoValidCommand
        },
        Err(InterpretErr::RequiresMoreInput) => {
            CommandResult::RequiresMoreInput
        }
    }
}

pub fn keycode_to_char(keycode: Keycode) -> Option<char> {
    match keycode {
        Keycode::A => Some('a'),
        Keycode::B => Some('b'),
        Keycode::C => Some('c'),
        Keycode::D => Some('d'),
        Keycode::E => Some('e'),
        Keycode::F => Some('f'),
        Keycode::G => Some('g'),
        Keycode::H => Some('h'),
        Keycode::I => Some('i'),
        Keycode::J => Some('j'),
        Keycode::K => Some('k'),
        Keycode::L => Some('l'),
        Keycode::M => Some('m'),
        Keycode::N => Some('n'),
        Keycode::O => Some('o'),
        Keycode::P => Some('p'),
        Keycode::Q => Some('q'),
        Keycode::R => Some('r'),
        Keycode::S => Some('s'),
        Keycode::T => Some('t'),
        Keycode::U => Some('u'),
        Keycode::V => Some('v'),
        Keycode::W => Some('w'),
        Keycode::X => Some('x'),
        Keycode::Y => Some('y'),
        Keycode::Z => Some('z'),
        Keycode::Quote => Some('\''),
        Keycode::Minus => Some('-'),
        _ => None,
    }
}

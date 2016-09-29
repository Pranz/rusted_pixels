use sdl2::pixels::Color;
use sdl2::keyboard::{Keycode,Mod,LALTMOD,LCTRLMOD, NOMOD};
use state::State;
use util;
use windows::Window;

/*
 * Veeery emacs inspired. Basically a emacs-like commando like
 * `M-x swap-color RET color1 color2`
 * would be translated in code as
 * &[AltModified(char), Exact("swap-color"), Color, Color]
 */

#[derive(PartialEq)]
pub enum Input {
    Char(Keycode,Mod),
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
    pub fn coerce_string(&self) -> String {
        if let Arg::String(ref string) = *self {
            return string.clone();
        }
        panic!("Commands misconfigured. Expected `String` on stack.");
    }

    pub fn coerce_color(&self) -> Color {
        if let Arg::Color(color) = *self {
            return color;
        }
        panic!("Commands misconfigured. Expected `Color` on stack.");
    }

    pub fn coerce_integer(&self) -> isize {
        if let Arg::Integer(integer) = *self {
            return integer;
        }
        panic!("Commands misconfigured. Expected `Integer` on stack.");
    }
}

#[derive(Copy, Clone)]
pub enum Command {
    ExportPng,
    Print,
    Quit,
    SetColor,
    ZoomIn,
    ZoomOut,
}

const META_X: Input = Input::Char(Keycode::X,LALTMOD);

pub fn get_commands() -> Vec<(Vec<Input>, Command)> {
    vec![(vec![Input::Char(Keycode::S,LCTRLMOD)],
          Command::ExportPng),
         (vec![META_X,
               Input::Exact(String::from("export-png")),
               Input::String],
          Command::ExportPng),
         (vec![Input::Char(Keycode::Q,LCTRLMOD)],
          Command::Quit),
         (vec![META_X,
               Input::Exact(String::from("quit"))],
          Command::Quit),
         (vec![META_X,
               Input::Exact(String::from("print")),
               Input::String],
          Command::Print),
         (vec![META_X,
               Input::Exact(String::from("set-color")),
               Input::Color],
          Command::SetColor),
         (vec![Input::Char(Keycode::W, NOMOD)],
          Command::ZoomOut),
         (vec![Input::Char(Keycode::F, NOMOD)],
          Command::ZoomIn),
    ]
}

pub enum InterpretErr {
    NoValidCommand,
    RequiresMoreInput
}

/*
 * Interpret the given input to see if there's a matching command
 * Returns matching command or an err if more input is required,
 * or wether there's no possible command for the input so far.
 */
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

/*
 * Check wether the current states input has a valid
 * command, and if so, executes the given command.
 * If no command is possible from the given input,
 * clear the input buffer. Otherwise, do nothing
 * and await more user input
 */
pub fn execute_command(state: &mut State, windows: &mut [Box<Window>],
                       commands: &[(Vec<Input>, Command)]) -> CommandResult {
    pub fn clean_input_and_args(state: &mut State) {
        state.args = Vec::new();
        state.input = Vec::new();
    }
        
    match interpret_input(&state.input, commands) {
        Ok(command) =>  {
            match select_command(state, windows, command) {
                CommandResult::Success => {
                    clean_input_and_args(state);
                    CommandResult::Success
                },
                anything_else => anything_else
            }
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

fn select_command(state: &mut State, windows: &mut [Box<Window>], command: Command) -> CommandResult {
    match command {
        Command::ExportPng => {
            commands::export_png(state)
        },
        Command::Quit => {
            println!("quit succesfully");
            CommandResult::Quit
            },
        Command::Print => {
            commands::print(state)
        },
        Command::SetColor => {
            commands::set_color(state)
        },
        Command::ZoomIn => {
            windows[state.window_index].increment_scale();
            CommandResult::Success
        },
        Command::ZoomOut => {
            windows[state.window_index].decrement_scale();
            CommandResult::Success
        },
    }
}


pub mod commands {
    use super::CommandResult;
    use super::Arg;
    use ::state::State;
    pub fn set_color(state: &mut State) -> CommandResult {
        let color = state.args.pop().unwrap().coerce_color();
        state.current_color = color;
        println!("set color");
        CommandResult::Success
    }

    pub fn export_png(state: &mut State) -> CommandResult {
       let out = state.args.pop()
            .unwrap_or(Arg::String(String::from("tmp/test_out.png")))
            .coerce_string();
        state.images[0].save_png_image(out).unwrap();
        println!("exported png");
        CommandResult::Success 
    }

    pub fn print(state: &mut State) -> CommandResult {
        println!("{}", state.args.pop().unwrap().coerce_string());
        CommandResult::Success
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
        Keycode::Space => Some(' '),
        Keycode::Comma => Some(','),
        Keycode::Slash => Some('/'),
        Keycode::Period => Some('.'),
        Keycode::Num0  => Some('0'),
        Keycode::Num1  => Some('1'),
        Keycode::Num2  => Some('2'),
        Keycode::Num3  => Some('3'),
        Keycode::Num4  => Some('4'),
        Keycode::Num5  => Some('5'),
        Keycode::Num6  => Some('6'),
        Keycode::Num7  => Some('7'),
        Keycode::Num8  => Some('8'),
        Keycode::Num9  => Some('9'),
        _ => None,
    }
}

/*
 * Parses the input, returning.
 * If the input is an argument, also return it.
 */
pub fn parse_input(input: &str) -> (Input, Option<Arg>) {
    if let Ok(integer) = input.parse::<isize>() {
        (Input::Integer, Some(Arg::Integer(integer)))
    }
    else if let Some(string) = parse_string(input) {
        (Input::String, Some(Arg::String(string)))
    }
    else if let Some(color) = util::parse_color(input) {
        (Input::Color, Some(Arg::Color(color)))
    }
    else {
        (Input::Exact(input.to_string()), None)
    }
}

pub fn parse_string(input: &str) -> Option<String> {
    if input.len() > 1
        && input.starts_with('\'')
        && input.as_bytes()[input.len() - 1] == b'\''
    {
        // remove quotation marks
        Some(input[1..(input.len() - 1)].to_string())
    }
    else { None }
}

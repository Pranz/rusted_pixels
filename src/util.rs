use sdl2::pixels::Color;

pub fn parse_color(string: &str) -> Option<Color> {
    if string.starts_with("rgb ") {
        
        let splitted = string[4..]
            .split(',')
            .into_iter()
            .collect::<Vec<&str>>();
        
        if splitted.len() == 3 {
            if let (Ok(r), Ok(g), Ok(b)) = (splitted[0].parse::<u8>(),
                                            splitted[1].parse::<u8>(),
                                            splitted[2].parse::<u8>())
            { return Some(Color::RGB(r,g,b)); }
        }
    } else if string.starts_with("rgba ") {
        
        let splitted = string[5..]
            .split(',')
            .into_iter()
            .collect::<Vec<&str>>();
        
        if splitted.len() == 4 {
            if let (Ok(r), Ok(g), Ok(b), Ok(a))
                = (splitted[0].parse::<u8>(),
                   splitted[1].parse::<u8>(),
                   splitted[2].parse::<u8>(),
                   splitted[3].parse::<u8>())
            { return Some(Color::RGBA(r,g,b,a)); }
        }
    } else if string.starts_with('#') {
        //TODO: hexadecimal conversion
        return None;
    }
    return None;
}

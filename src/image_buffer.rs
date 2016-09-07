use png;
use sdl2::pixels::Color;
use std::{fmt,fs,io,path};
use std::error::Error;
use std::io::Write;

/*
 * Holds image data
 */
pub struct ImageBuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize) -> Self {
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

    pub fn load_png_image<P: AsRef<path::Path>>(path: P) -> io::Result<Self> {
        use png::ColorType::*;

        let decoder = png::Decoder::new(try!(fs::File::open(path)));
        let (info,mut reader) = try!(decoder.read_info());
        let mut img_data = vec![0; info.buffer_size()];
        try!(reader.next_frame(&mut img_data));

        Ok(ImageBuffer{
            width: info.width as usize,
            height: info.height as usize,
            buffer: match info.color_type {
                RGB => {
                    //Check data length
                    if img_data.len()%3!=0{
                        return Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            LoadImageError::DataAndColorTypeMismatch(info.color_type)
                        ));
                    }

                    //Map every three elements of the data to a Color struct
                    img_data.chunks(3).map(|color_data|{
                        if let &[r,g,b] = color_data{
                            Color::RGB(r,g,b)
                        }else{
                            unreachable!()
                        }
                    }).collect()
                },
                RGBA => {
                    //Check data length
                    if img_data.len()%4!=0{
                        return Err(io::Error::new(
                            io::ErrorKind::UnexpectedEof,
                            LoadImageError::DataAndColorTypeMismatch(info.color_type)
                        ));
                    }

                    //Map every four elements of the data to a Color struct
                    img_data.chunks(4).map(|color_data|{
                        if let &[r,g,b,a] = color_data{
                            Color::RGBA(r,g,b,a)
                        }else{
                            unreachable!();
                        }
                    }).collect()
                },
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        LoadImageError::UnsupportedColorType(info.color_type)
                    ));
                }
            }
        })
    }

    pub fn save_png_image<P: AsRef<path::Path>>(&self,path: P) -> io::Result<()>{
        let mut out = Vec::new();
        {
            let mut encoder = png::Encoder::new(
                &mut out,
                png::Info{
                    width : self.width as u32,
                    height: self.height as u32,
                    color_type: png::ColorType::RGB,//TODO: RGB output is hardcoded because there's no way to choose at the moment. It will fail when the color data is RGBA
                    .. png::Info::default()
                },
            ).write_header().unwrap();

            //Read from the local data, convert and then write to the file data
            let buffer: Vec<_> = self.buffer.iter().flat_map(ColorIter::new).collect();
            encoder.write_image_data(buffer.as_ref()).unwrap();
    }
        //Open and write to file
        let mut file = try!(fs::File::create(path));
        try!(file.write(out.as_ref()));
        Ok(())
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

/*
 * Error output type of load_image
 */
#[derive(Debug,Clone, PartialEq)]
pub enum LoadImageError {
    DataAndColorTypeMismatch(png::ColorType),
    UnsupportedColorType(png::ColorType)
}
impl Error for LoadImageError {
    fn description(&self) -> &str{match self{
        &LoadImageError::DataAndColorTypeMismatch(_) =>
            "The given PNG data does not align correctly to the color type",
        &LoadImageError::UnsupportedColorType(_) =>
            "The color type in the PNG is not supported",
    }}
}
impl fmt::Display for LoadImageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

/*
 * Iterates over the individual color components of a Color
 */
pub struct ColorIter<'a>(&'a Color,u8);
impl<'a> ColorIter<'a>{
    pub fn new(c: &'a Color) -> Self{
        ColorIter(c,0)
    }
}
impl<'a> Iterator for ColorIter<'a>{
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item>{
        match self{
            &mut ColorIter(&Color::RGB (r,_,_)  ,0) |
            &mut ColorIter(&Color::RGBA(r,_,_,_),0) => {self.1=1; Some(r)}
            &mut ColorIter(&Color::RGB (_,g,_)  ,1) |
            &mut ColorIter(&Color::RGBA(_,g,_,_),1) => {self.1=2; Some(g)}
            &mut ColorIter(&Color::RGB (_,_,b)  ,2) |
            &mut ColorIter(&Color::RGBA(_,_,b,_),2) => {self.1=3; Some(b)}
            &mut ColorIter(&Color::RGBA(_,_,_,a),3) => {self.1=4; Some(a)}
            _ => None
        }
    }
}

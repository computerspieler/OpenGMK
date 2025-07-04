use std::{
    collections::HashMap, fmt::{write, Formatter}, fs::File, io::{self, BufReader, BufWriter, Write}, path::Path
};

use gm8exe::reader::ReaderError;
use utf8_chars::BufReadCharsExt;
use bmp::{Pixel, Image};

pub enum ParsingError {
    UnexpectedEof,
    IoErr(io::Error),
    ReaderErr(ReaderError)
}

impl std::fmt::Display for ParsingError {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
        ParsingError::UnexpectedEof => write!(fmt, "Unexpected end of file"),
        ParsingError::IoErr(e) => write!(fmt, "I/O error: {}", e),
        ParsingError::ReaderErr(e) => write!(fmt, "Reader error: {}", e),
        }
    }
}

pub trait ToParsingError<T> {
    fn convert(self) -> Result<T, ParsingError>;
}
impl<T, E> ToParsingError<T> for Result<T, E>
    where E: Into<ParsingError> {

    fn convert(self) -> Result<T, ParsingError> {
        self.map_err(|x| x.into())
    }
}

impl Into<ParsingError> for io::Error {
    fn into(self) -> ParsingError {
        ParsingError::IoErr(self)
    }
}
impl Into<ParsingError> for ReaderError {
    fn into(self) -> ParsingError {
        ParsingError::ReaderErr(self)
    }
}


fn write_with_replacements(
    mut fin: std::iter::Peekable<utf8_chars::Chars<'_, BufReader<File>>>,
    mut fout: BufWriter<File>,
    replacements: HashMap<String, String>
) -> Result<(), ParsingError> {
    let mut braces_encountered_in_a_row = 0;
    let mut variable_encountered: Vec<char> = Vec::new();
    while let Some(c) = fin.next() {
        let c= c.convert()?;
        match c {
        '{' if braces_encountered_in_a_row < 2
            => braces_encountered_in_a_row += 1,
        
        'a' ..= 'z' | 'A' ..= 'Z' | '_'
        if braces_encountered_in_a_row == 2 => {
            variable_encountered.push(c);
        }

        '}'
        if braces_encountered_in_a_row == 2 => {
            let var_name = variable_encountered.iter().collect::<String>();
            match replacements.get(&var_name) {
            None => write!(&mut fout, "{{{{{}}}}}", var_name).convert()?,
            Some(x) => write!(&mut fout, "{}", x).convert()?
            }
            variable_encountered.clear();
            braces_encountered_in_a_row = 1;
            if let Some(Ok('}')) = fin.peek() {
                fin.next();
                braces_encountered_in_a_row = 0;
            }
        }

        c => {
            while braces_encountered_in_a_row > 0 {
                braces_encountered_in_a_row -= 1;
                
                write!(&mut fout, "{{").convert()?
            }
            write!(&mut fout, "{}", c).convert()?
        }
        }
    }

    if braces_encountered_in_a_row == 0 {
        Ok(())
    } else {
        Err(ParsingError::UnexpectedEof)
    }
}

pub fn copy_and_replace_with_hashmap(
    path: String,
    src: String,
    dst: String,
    replacements: HashMap<String, String>
) -> Result<(), ParsingError> {
    let mut fin_buf = BufReader::new(
        File::open(src).convert()?
    );
    let fin = fin_buf.chars().peekable();
    let fout = BufWriter::new(
        File::options()
            .create(true)
            .write(true)
            .read(false)
            .append(false)
            .open(
                path.clone() + &dst
            )
            .convert()?
    );

    write_with_replacements(fin, fout, replacements)
}

#[macro_export]
macro_rules! copy_and_replace {
    ($path: ident,
        $src: literal => $dst: literal,
        $($to_replace: ident : $t: ident $replace: expr),*
    ) => {{
        use std::collections::HashMap;
        use crate::utils::copy_and_replace_with_hashmap;

        let mut map = HashMap::new();

        $(
            map.insert(
                String::from(stringify!($to_replace)),
                match stringify!($t) {
                "val" => format!("{}", $replace),
                "str" => format!("\"{}\"", $replace),
                _ => todo!()
                }
            );
        )*

        copy_and_replace_with_hashmap(
            String::from($path),
            String::from($src),
            String::from($dst),
            map
        )?;
    }};
}

pub fn replace_and_amend_with_hashmap(
    path: String,
    src: String,
    dst: String,
    replacements: HashMap<String, String>
) -> Result<(), ParsingError> {
    let mut fin_buf = BufReader::new(
        File::open(src).convert()?
    );
    let fin = fin_buf.chars().peekable();
    let fout = BufWriter::new(
        File::options()
            .append(true)
            .write(true)
            .read(false)
            .open(
                path.clone() + &dst
            )
            .convert()?
    );

    write_with_replacements(fin, fout, replacements)
}

#[macro_export]
macro_rules! replace_and_amend {
    ($path: ident,
        $src: literal => $dst: literal,
        $($to_replace: ident : $t: ident $replace: expr),*
    ) => {{
        use std::collections::HashMap;
        use crate::utils::replace_and_amend_with_hashmap;

        let mut map = HashMap::new();

        $(
            map.insert(
                String::from(stringify!($to_replace)),
                match stringify!($t) {
                "val" => format!("{}", $replace),
                "str" => format!("\"{}\"", $replace),
                _ => todo!()
                }
            );
        )*

        replace_and_amend_with_hashmap(
            String::from($path),
            String::from($src),
            String::from($dst),
            map
        )?;
    }};
}

pub fn save_image<P: AsRef<std::path::Path>>(path: P, pixels: &Box<[u8]>, width: u32, height: u32)
    -> std::io::Result<()>
{
    let mut img = Image::new(width, height);

    for x in 0 .. width {
        for y in 0 .. height {
            let idx = (y * width + x) as usize;
            let b = pixels[4*idx + 0];
            let g = pixels[4*idx + 1];
            let r = pixels[4*idx + 2];
            //let a = pixels[4*idx + 3];
            img.set_pixel(x, y, Pixel {
                r: r, g: g, b: b
            });
        }
    }

    img.save(path)
}

pub fn write_str_to_file<P>(path: P, s: &str)
    -> Result<usize, ParsingError>
where P: AsRef<Path> {
    Ok(BufWriter::new(
        File::options()
            .create(true)
            .write(true)
            .read(false)
            .append(true)
            .open(path)
            .convert()?
    ).write(s.as_bytes()).convert()?)
}
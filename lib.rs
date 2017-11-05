use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use std::convert::AsRef;
use std::option::Option;
use std::fmt::Arguments;

pub fn read<P: AsRef<Path>>(src:P) -> Option<Vec<u8>> {
        
    return match File::open(src) {

        Ok(mut file) => {

            let mut data = Vec::new();

            if let Ok(m) = file.metadata() {
                data.reserve(m.len() as usize);
            }
                
            file.read_to_end(&mut data);

            Option::Some(data)

       }
       
       Err(_) => Option::None,

    };

}

pub fn read_text<P: AsRef<Path>>(src:P) -> Option<String> {

    return match File::open(src) {

        Ok(mut file) => {
                
            let mut content = String::new();

            file.read_to_string(&mut content);

            Option::Some(content)

        }

        Err(_) => Option::None,

    };

}

pub fn read_delim<P: AsRef<Path>>(src:P, d:char) -> Option<Vec<String>> {

    return match File::open(src) {

        Ok(mut file) => {

            let mut content = String::new();

            file.read_to_string(&mut content);

            Option::Some(content.split(d).map(|s| s.to_string()).collect())

        }

        Err(_) => Option::None,

    };

}

pub fn read_lines<P: AsRef<Path>>(src:P) -> Option<Vec<String>> {
        
    self::read_delim(src, '\n')

}

pub fn read_words<P: AsRef<Path>>(src:P) -> Option<Vec<String>> {
        
    return match File::open(src) {

        Ok(mut file) => {

            let mut content = String::new();

            file.read_to_string(&mut content);

            Option::Some(content.split(&['\n', ' '][..]).map(|s| s.to_string()).collect())

        }

        Err(_) => Option::None,

    };

}

pub fn write<P: AsRef<Path>, B: AsRef<[u8]>>(src:P, data:B, trunc:bool) -> Option<usize> {

    return match OpenOptions::new().write(true)
                                   .truncate(trunc)
                                   .append(!trunc)
                                   .create(true)
                                   .open(src) {
        
        Ok(mut file) => match file.write(data.as_ref()) {

            Ok(n) => Option::Some(n),

            Err(_) => Option::Some(0),

        }

        Err(_) => Option::None,

    };

}

pub fn write_text<P: AsRef<Path>, S: AsRef<str>>(src:P, text:S, trunc:bool) -> Option<usize> {

    self::write(src, text.as_ref().as_bytes(), trunc)

}

pub fn write_lines<P: AsRef<Path>>(src:P, lines:Vec<String>, trunc:bool) -> Option<usize> {

    self::write_text(src, lines.join("\r\n"), trunc)

}

pub fn write_fmt<P: AsRef<Path>>(src:P, fmt:Arguments, trunc:bool) -> bool {

    return match OpenOptions::new().write(true)
                                   .truncate(trunc)
                                   .append(!trunc)
                                   .create(true)
                                   .open(src) {
        
        Ok(mut file) => match file.write_fmt(fmt) {

            _ => true,

        }
        
        Err(_) => false,

    };

}

pub fn write_newline<P: AsRef<Path>>(src:P) -> bool {

    return match self::write_text(src, "\r\n", false) {

        Some(_) => true,

        None => false,

    }

}
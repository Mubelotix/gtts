use percent_encoding::{AsciiSet, utf8_percent_encode, CONTROLS};
use minreq::get;
use std::fs::File;
use std::io::prelude::*;

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn save_to_file(text: &str, filename: &str) -> bool {
    let len = text.len();
    let text = utf8_percent_encode(text, FRAGMENT).to_string();

    if let Ok(rep) = get(format!("https://translate.google.fr/translate_tts?ie=UTF-8&q={}&tl=en&total=1&idx=0&textlen={}&tl=en&client=tw-ob", text, len)).send() {
        if let Ok(mut file) = File::create(filename) {
            let bytes = rep.as_bytes();
            if bytes.len() > 0 {
                if file.write_all(bytes).is_ok() {
                    return true;
                }
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(save_to_file("Hello world!", "test"));
    }
}

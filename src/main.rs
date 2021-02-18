use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Lexer<R: Read> {
    reader: R,
    last_char: u8,
}


impl<R> Lexer<R>
where
    R: Read,
{
    const EMPTY: u8 = 0;

    fn new(file: R) -> Lexer<R> {
        Lexer {
            reader: file,
            last_char: Lexer::<R>::EMPTY,
        }
    }
    fn get_char(&mut self) -> Option<u8> {
        if self.last_char == Lexer::<R>::EMPTY {
            let mut byte: [u8; 1] = [0];
            if let Ok(size) = self.reader.read(&mut byte) {
                if size > 0 {
                    return Some(byte[0]);
                }
            }
            None
        } else {
            let c = self.last_char;
            self.last_char = Lexer::<R>::EMPTY;
            Some(c)
        }
    }
    fn under_char(&mut self, c: u8) {
        self.last_char = c;
    }
    fn read(&mut self) -> Option<String> {
        let mut sb = String::new();
        let mut c;
        loop {
            c = self.get_char()?;
            if !Self::is_space(c) { break; }
        }
        if Self::is_digit(c) {
            loop {
                sb.push(c as char);
                c = self.get_char()?;
                if !Self::is_digit(c) { break; }
            }
        } else if Self::is_letter(c) {
            loop {
                sb.push(c as char);
                c = self.get_char()?;
                if !Self::is_letter(c) { break; }
            }
        } else if c == '=' as u8 {
            println!("=");
            c = self.get_char()?;
            if c == '=' as u8 {
            println!("==");
                return Some("==".to_string());
            } else {
                self.under_char(c);
                return Some("=".to_string());
            }
        } else {
            panic!();
        }

        self.under_char(c);

        Some(sb)
    }

    fn is_space(c: u8) -> bool {
        if 0 == c || 1 <= c && c <= ' ' as u8 {
            true
        } else {
            false
        }
    }
    fn is_digit(c: u8) -> bool {
        if '0' as u8 <= c && c <= '9' as u8 {
            true
        } else {
            false
        }
    }
    fn is_letter(c: u8) -> bool {
        if 'A' as u8 <= c && c <= 'Z' as u8 || 'a' as u8 <= c && c <= 'z' as u8 {
            true
        } else {
            false
        }
    }

}

fn main() {
    let path = "./sample.txt";
    let file = File::open(path).unwrap();
    let mut lexer = Lexer::new(file);
    while let Some(token) = lexer.read() {
        dbg!(token);
    }

}

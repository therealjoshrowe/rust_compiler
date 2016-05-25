use std::char;
use std::io::Read;
use std::error::Error;
use std::io;
use std::fs::File;

const LEFT_PAREN: i32 = 101;
const RIGHT_PAREN: i32 = 102;
const PLUS_OP: i32 = 103;
const MINUS_OP: i32 = 104;
const MULT_OP: i32 = 105;
const DIV_OP: i32 = 106;
const ASSIGN_OP: i32 = 107;
const EQUALS_OP: i32 = 108;
const ARROW_OP: i32 = 109;
const AMBER_OP: i32 = 110;
const LEFT_MOUST: i32 = 111;
const RIGHT_MOUST: i32 = 112;
const PERIOD: i32 = 113;
const NOT_EQUALS_OP: i32 = 114;
const DOUBLE_COLON_OP: i32 = 115;
const COMMA: i32 = 116;
const SEMICOLON: i32 = 117;
const LEFT_BRACE: i32 = 118;
const RIGHT_BRACE: i32 = 119;
const STRING_LIT: i32 = 120;
const INT_LIT: i32 = 121;

fn main() {
    let mut lexeme = String::new();

    let mut f = match File::open("foo.txt") {
        Ok(file) => file,
        Err(why) => panic!("could not read {}", Error::description(&why)),
    };

    let mut c = read_char(&f);

    while c != 0 {

        let mut ch = c as char;

        while ch.is_whitespace() {
            c = read_char(&f);
            ch = c as char;
        }
       
        if ch.is_alphabetic() {
            while (ch.is_alphabetic()) {
                lexeme.push(ch);
                c = read_char(&f);
                ch = c as char;
            }
        }
       else  if ch.is_numeric() {
            while ch.is_numeric() {
                lexeme.push(ch);
                c = read_char(&f);
                ch = c as char;
            }
        }
        else  {
            let token = lookup(ch, &f);
            lexeme.push(ch);
        }


        println!("{}", lexeme);
        lexeme = String::new();
        c = read_char(&f);
    }

}
fn read_char(mut f: &File) -> u8 {
   let mut buffer: [u8; 1]  = [0];
   let c = match f.read(&mut buffer) {
       Ok(ch) => ch,
       Err(why) => panic!("Could not read {}", Error::description(&why)),
   }; 
   buffer[0] 
}
fn lookup(ch: char, mut f: &File) -> (i32, String) {
    let mut token = 0;
    let mut lexeme = String::new();
    if ch == '(' {
        token = LEFT_PAREN;
        lexeme.push(ch);
   }
    else if ch == ')' {
        token = RIGHT_PAREN;
        lexeme.push(ch);
    }
    else if ch == '+' {
        token = PLUS_OP;
        lexeme.push(ch);
    }
    else if ch == '-' {
        token = MINUS_OP;
        lexeme.push(ch);
    }
    else if ch == '/' {
        token = DIV_OP;
        lexeme.push(ch);
    }
    else if ch == '*' {
        token = MULT_OP;
        lexeme.push(ch);
    }
    (token, lexeme)
}

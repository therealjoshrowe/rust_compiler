use std::env;
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
const PATH_OP: i32 = 115;
const COMMA: i32 = 116;
const SEMICOLON: i32 = 117;
const LEFT_BRACE: i32 = 118;
const RIGHT_BRACE: i32 = 119;
const STRING_LIT: i32 = 120;
const INT_LIT: i32 = 121;
const STAR_OP: i32 = 122;
const MATCH_ARROW: i32 = 123;
const COLON: i32 = 124;
const NO_RETURN: i32 = 125;
const IDENT: i32 = 126;
const KEYWORD: i32 = 127;
const CHAR_LIT: i32 = 128;
const LINE_COMMENT: i32 = 129;
const LINE_CONTINUATION: i32 = 130;

fn main() {
    let mut next_char;
    let mut token = 0;
    let mut lexeme = String::new();
    //need to do bounds checking on args vec!

    let args: Vec<_> = env::args().collect();
    let f = match File::open(&args[1]) {
        Ok(file) => file,
        Err(why) => panic!("could not read {}", Error::description(&why)),
    };

    next_char = read_char(&f);
    //need to add support to handle comments!!!!!
    while next_char   != 0 as char {
        
        while next_char.is_whitespace() {
            next_char = read_char(&f);
        }
       
        if next_char.is_alphabetic() {
            lexeme.push(next_char);
            next_char = read_char(&f);
            while next_char.is_alphabetic() || next_char == '_' || next_char.is_numeric() ||
                  next_char == '!' {
                lexeme.push(next_char);
                next_char = read_char(&f);
            }
            token = IDENT; // only handle identifiers currently
        }
       else  if next_char.is_numeric() {
            while next_char.is_numeric() {
                lexeme.push(next_char);
                next_char = read_char(&f);
            }
            token = INT_LIT; //only handle integers currently
        }
        else  {
            let (tok, lex) = lookup(&mut next_char, &f);
            lexeme.push_str(&lex);
            token = tok;
            next_char = read_char(&f);
        }


        println!("({}, {})", token, lexeme);
        lexeme = String::new();
    }

}
fn read_char(mut f: &File) -> char {
   let mut buffer: [u8; 1]  = [0];
   let _ = match f.read(&mut buffer) {
       Ok(ch) => ch,
       Err(why) => panic!("Could not read {}", Error::description(&why)),
   };
   buffer[0] as char
}

fn lookup(next_char: &mut char, f: &File) -> (i32, String) {
    let mut token = 0;
    let mut lexeme = String::new();
    let ch = *next_char;

    if ch == '(' {
        token = LEFT_PAREN;
        lexeme.push('(');
    }
    else if ch == ')' {
        token - RIGHT_PAREN;
        lexeme.push(')');
    }
    else if ch == '+' {
        token = PLUS_OP;
        lexeme.push('+');
    }
    else if ch == '-' {
        lexeme.push('-');
        *next_char = read_char(&f);
        if *next_char == '>' {
            token = ARROW_OP;
            lexeme.push('>');
        }
        else {
            token = MINUS_OP;
        }
    } 
    else if ch == '*' {
        token = STAR_OP;
        lexeme.push('*');
    }
    else if ch == '/' {
        token = DIV_OP;
        lexeme.push('/');
    }
    else if ch == '=' {
        lexeme.push(ch);
        *next_char = read_char(&f);
        if *next_char == '=' {
            token = EQUALS_OP;
            lexeme.push(*next_char);
        }
        else if *next_char == '>' {
            token = MATCH_ARROW;
            lexeme.push(*next_char);
        }
        else {
            token = ASSIGN_OP;
        }
    }
    else if ch == '&' {
        lexeme.push(ch);
        token = AMBER_OP;
    }
    else if ch == '{' {
        token = LEFT_MOUST;
        lexeme.push(ch);
    }
    else if ch == '}' {
        token = RIGHT_MOUST;
        lexeme.push(ch);
    }
    else if ch == '.' {
        token = PERIOD;
        lexeme.push(ch);
    }
    else if ch == '!' {
        lexeme.push(ch);
        *next_char = read_char(&f);
        if *next_char == '=' {
            token = NOT_EQUALS_OP;
            lexeme.push(*next_char);
        }
        else {
            token = NO_RETURN;
        }
    }
    else if ch == ':' {
        lexeme.push(ch);
        *next_char = read_char(&f);
        if *next_char == ':' {
            token = PATH_OP;
            lexeme.push(*next_char);
        }
        else {
            token = COLON;
        }
    }
    else if ch == '[' {
        lexeme.push(ch);
        token = LEFT_BRACE;
    }
    else if ch == ']' {
        lexeme.push(ch);
        token = RIGHT_BRACE;
    }
    else if ch == '\'' {
        lexeme.push(ch);
        *next_char = read_char(&f);
        if *next_char == '\\' {
            lexeme.push(*next_char);
            *next_char = read_char(&f);
            if *next_char == 'n' || *next_char == 'r' || *next_char == 't' || *next_char == '\\' ||
               *next_char == '0' || *next_char == '\'' || *next_char == '\"' {
                   lexeme.push(*next_char);
                   *next_char = read_char(&f);
                   if *next_char == '\'' {
                       lexeme.push(*next_char);
                       token = CHAR_LIT;
                   }
                   else {
                       lex_error();
                   }
            }
            else {
                lex_error();
            }
        }
        else {
            lexeme.push(*next_char);
            *next_char = read_char(&f);
            if *next_char == '\'' {
                lexeme.push(*next_char);
                token = CHAR_LIT;
            }
            else {
                lex_error();
            }
        }
    }
    else if ch == '"' {
        lexeme.push(ch);
        *next_char = read_char(&f);
        while *next_char != '"' {
            lexeme.push(*next_char);
            *next_char = read_char(&f);
        }
        lexeme.push(*next_char);
        token = STRING_LIT; // does not handle escape sequences or malformed strings
    }
    else if ch == ';' {
        lexeme.push(ch);
        token = SEMICOLON;

    }
    else if ch == '/' {
        lexeme.push(ch);
        *next_char = read_char(&f);
        if *next_char == '/' {
            lexeme.push(*next_char);
            while *next_char != '\n' {
                *next_char = read_char(&f);
                lexeme.push(*next_char);
            }
            token = LINE_COMMENT;
        }
        else {
            token = LINE_CONTINUATION;
        }
    }
    (token, lexeme)
}
fn lex_error() -> ! {
    panic!("There was a lexical error");
}

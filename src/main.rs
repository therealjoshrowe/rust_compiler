use std::collections::HashSet;
use std::env;
use std::io::Read;
use std::error::Error;
use std::fs::File;

const LEFT_PAREN: i32 = 101;
const RIGHT_PAREN: i32 = 102;
const PLUS_OP: i32 = 103;
const MINUS_OP: i32 = 104;
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
const HASH: i32 = 131;
const LEFT_ANGLE: i32 = 132;
const RIGHT_ANGLE: i32 = 133;
const UNDERSCORE: i32 = 134;

fn main() {
    let mut next_char;
    let mut token = 0;
    let mut lexeme = String::new();
    let mut keywords: HashSet<String> = HashSet::new();
    load_keywords(&mut keywords);
    //need to do bounds checking on args vec!

    let args: Vec<_> = env::args().collect();
    let f = match File::open(&args[1]) {
        Ok(file) => file,
        Err(why) => panic!("could not read {}", Error::description(&why)),
    };

    next_char = read_char(&f);
    //need to add support to handle block comments
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
            if keywords.contains(&lexeme) {
                token = KEYWORD;
            }
            else {
                token = IDENT;
            }
        }
       else  if next_char.is_numeric() {
            while next_char.is_numeric() {
                lexeme.push(next_char);
                next_char = read_char(&f);
            }
            token = INT_LIT; //only handle integers currently
        }        
        else if next_char == '/' {
            lexeme.push(next_char);
            next_char = read_char(&f);
            if next_char == '/' {
                lexeme.push(next_char);
                while next_char != '\n' {
                    if next_char != '\n' {
                        lexeme.push(next_char);
                        next_char = read_char(&f);
                    }
                }
                token = LINE_COMMENT;
            }
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
    else if ch == '#' {
        lexeme.push(ch);
        token = HASH;
    }
    else if ch == ',' {
        lexeme.push(ch);
        token = COMMA;
    }
    else if ch == '<' {
        lexeme.push(ch);
        token = LEFT_ANGLE;
    }
    else if ch == '>' {
        lexeme.push(ch);
        token = RIGHT_ANGLE;
    }
    else if ch == '_' {
        lexeme.push(ch);
        token = UNDERSCORE;
    }
    (token, lexeme)
}
fn lex_error() -> ! {
    panic!("There was a lexical error");
}
fn load_keywords(s: &mut HashSet<String>) {
    s.insert(String::from("abstract"));
    s.insert(String::from("alignof"));
    s.insert(String::from("as"));
    s.insert(String::from("become"));
    s.insert(String::from("box"));
    s.insert(String::from("const"));
    s.insert(String::from("continue"));
    s.insert(String::from("crate"));
    s.insert(String::from("do"));
    s.insert(String::from("else"));
    s.insert(String::from("enum"));
    s.insert(String::from("extern"));
    s.insert(String::from("false"));
    s.insert(String::from("final"));
    s.insert(String::from("fn"));
    s.insert(String::from("for"));
    s.insert(String::from("if"));
    s.insert(String::from("impl"));
    s.insert(String::from("in"));
    s.insert(String::from("let"));
    s.insert(String::from("loop"));
    s.insert(String::from("macro"));
    s.insert(String::from("match"));
    s.insert(String::from("mod"));
    s.insert(String::from("move"));
    s.insert(String::from("offsetof"));
    s.insert(String::from("override"));
    s.insert(String::from("priv"));
    s.insert(String::from("proc"));
    s.insert(String::from("pub"));
    s.insert(String::from("pure"));
    s.insert(String::from("ref"));
    s.insert(String::from("return"));
    s.insert(String::from("Self"));
    s.insert(String::from("Self"));
    s.insert(String::from("sizeof"));
    s.insert(String::from("struct"));
    s.insert(String::from("super"));
    s.insert(String::from("trait"));
    s.insert(String::from("true"));
    s.insert(String::from("type"));
    s.insert(String::from("typeof"));
    s.insert(String::from("unsafe"));
    s.insert(String::from("unsized"));
    s.insert(String::from("virtual"));
    s.insert(String::from("where"));
    s.insert(String::from("while"));
    s.insert(String::from("yeild"));
}

use std::fmt::{Display, Formatter, Result};

pub enum Token {
    INT(i64),
    FLOAT(f64),
    STRING(String),
    UNKNOWN,
    SEP, // separador
    EOL, // end of line
    EOF, // end of line
}

pub type CSVLine = Vec<Token>;

impl Clone for Token {
    fn clone(&self) -> Self {
        match self {
            Token::INT(i) => Token::INT(*i),
            Token::FLOAT(f) => Token::FLOAT(*f),
            Token::STRING(s) => Token::STRING(s.clone()),
            Token::UNKNOWN => Token::UNKNOWN,
            Token::SEP => Token::SEP, // separador
            Token::EOL => Token::EOL, // end of line
            Token::EOF => Token::EOF, // end of line
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{}",
            match *self {
                Token::INT(_) => &"Token Integer",
                Token::FLOAT(_) => &"Token Float",
                Token::STRING(_) => &"Token String",
                Token::UNKNOWN => &"Token Unknown",
                Token::SEP => &"Token field Separator",
                Token::EOL => &"Token End Of Line",
                Token::EOF => &"Token End Of File",
                _ => "Error",
            }
        )
    }
}

#[allow(non_snake_case)]
pub fn printTknVec(v: &Vec<Token>) {
    print!("{{\n");
    for tkn in v {
        print!("\t{}, ", &tkn);
    }
    println!("}}");
}

#[allow(non_snake_case)]
pub fn dbgTknVec(v: &Vec<Token>) {
    let mut i: usize = 0;
    let len = v.len();
    print!("Token vector ({}) ", len);
    print!("{{\n");
    for tkn in v {
        print!("\t{} ", &tkn);
        match &tkn {
            Token::INT(i) => print!("{}, ", &i),
            Token::FLOAT(f) => print!("{}, ", &f),
            Token::STRING(s) => print!("{}, ", &s),
            _ => print!(","),
        }
        println!();
        i += 1;
    }
    println!("}}");
    assert_eq!(i, len);
    println!("Vector length {}", i);
}

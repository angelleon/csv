use std::fs::File;
use std::io::{BufRead, BufReader};

extern crate unicode_segmentation;
use unicode_segmentation::UnicodeSegmentation as UniSeg;

use super::{CSVLine, Token};

pub struct Automata {
    /**
     * Objeto que parsea el contenido de un archivo CSV
     */
    f_buff: BufReader<File>,
    buff: String,
    tkn_offset: usize,
    tkn_length: usize,
}

impl Automata {
    pub fn new(file_name: &str) -> Automata {
        Automata {
            f_buff: BufReader::new(File::open(file_name).expect("Error opening file")),
            buff: String::new(),
            tkn_offset: 0,
            tkn_length: 0,
        }
    }

    pub fn parse_line(&mut self) -> CSVLine {
        // print!("===========================================");
        //let mut buff = String::new();
        self.buff.clear();
        let buff_size = self.f_buff.read_line(&mut self.buff).expect("");
        //print!("Read line size: {}", &buff_size);
        //print!("Read line: {}", &self.buff);
        let mut curr_tkn: Token;
        if buff_size == 0 {
            return vec![Token::EOF];
        }
        // print!("buffer length: {}", &self.buff.len());
        // print!("raw buffer: {:?}", &self.buff);
        let mut v = Vec::<Token>::new();
        let mut i: usize = 0;
        self.tkn_offset = 0;
        self.tkn_length = 0;
        loop {
            curr_tkn = self.next_token();
            match curr_tkn {
                Token::EOL => {
                    v.push(Token::EOL);
                    break;
                }
                Token::EOF => {
                    v.push(Token::EOF);
                    break;
                }
                _ => v.push(curr_tkn),
            }
            // print!("last vec element: {}", &v.last().expect("no hay elementos en el vector"));
            if i > buff_size {
                break;
            }
            i += 1;
        }
        // print!("");
        v
    }

    fn next_token(&mut self) -> Token {
        const F: i16 = -1; // float
        const S: i16 = -2; // string
        const I: i16 = -3; // integer
        const SEP: i16 = -4; // value separator
        const EOF: i16 = -5; // End Of File
        const EOL: i16 = -6; // End Of File

        let mut state: i16 = 0;
        let mut raw_tkn = String::new();
        let buff = UniSeg::graphemes(&*self.buff, true).collect::<Vec<&str>>();
        let mut c: &str;
        self.tkn_length = 0;
        while state >= 0 {
            // println!("=====================================");
            c = buff[self.tkn_offset + self.tkn_length];
            // println!("character: {:?}", &c);
            // println!("state: {}", &state);
            state = Automata::transition(state, &c);
            // println!("state: {}", &state);
            if state >= 0 {
                if state > 0 {
                    raw_tkn.push_str(&c);
                } else {
                    //self.tkn_length += 1;
                    self.tkn_offset += 1;
                }
            } else {
                self.tkn_length += 1;
                self.tkn_offset += self.tkn_length
                    - match state {
                        SEP | EOL | EOF => 0,
                        _ => 1,
                    };
                if state == S {
                    match c {
                        "\"" | "\'" => self.tkn_offset += 1,
                        _ => {}
                    }
                }
                match state {
                    F => return Token::FLOAT(raw_tkn.parse().expect("Can not parse float value")),
                    S => return Token::STRING(raw_tkn),
                    I => return Token::INT(raw_tkn.parse().expect("Can not parse integer value")),
                    EOL => return Token::EOL,
                    EOF => return Token::EOF,
                    SEP => return Token::SEP,
                    _ => return Token::UNKNOWN,
                };
            }
            // println!("{:?}", &raw_tkn);
            self.tkn_length += 1;
            // println!("offset: {}", &self.tkn_offset);
            // println!("length: {}", &self.tkn_length);
            if self.tkn_offset + self.tkn_length == buff.len() {
                return Token::EOL;
            }
            // println!("=====================================");
        }
        Token::UNKNOWN
    }

    fn transition(state: i16, c: &str) -> i16 {
        const F: i16 = -1; // float
        const S: i16 = -2; // string
        const I: i16 = -3; // integer
        const SEP: i16 = -4; // value separator
        const EOF: i16 = -5; // End Of File
        const EOL: i16 = -6; // End Of File
        const MATR: [[i16; 11]; 14] = [
        // TODO: quitar 1
        //     0    1    2    3    4    5    6    7    8    9   10
        //   SEP    D    e    .    -    "    '  EOL  EOF  LAM,  WS
            [SEP,   2,  12,  12,   2,   8,  10, EOL, EOF,  12,   0], //  0
            [SEP, SEP, SEP, SEP, SEP, SEP, SEP, SEP, SEP, SEP, SEP], //  1
            [  I,   2,  13,   3,  13,  13,  13,   I,   I,  13,   I], //  2
            [  S,   4,   S,   S,   S,   S,   S,   S,   S,   S,   S], //  3
            [  F,   4,   5,   S,   S,   S,   S,   F,   F,   S,   F], //  4
            [  S,   7,   S,   S,   6,   S,   S,   S,   S,   S,   S], //  5
            [  S,   7,   S,   S,   S,   S,   S,   S,   S,   S,   S], //  6
            [  F,   7,   S,   S,   S,   S,   S,   F,   F,   S,   F], //  7
            [  9,   9,   9,   9,   9,   S,   S,   S,   S,   9,   9], //  8
            [  9,   9,   9,   9,   9,  13,   9,   S,   S,   9,   9], //  9
            [ 11,  11,  11,  11,  11,  11,  11,   S,   S,  11,  11], // 10
            [ 11,  11,  11,  11,  11,  11,  13,   S,   S,  11,  11], // 11
            [  S,  12,  12,  12,  12,  12,  12,   S,   S,  12,  12], // 12
            [  S,  13,  13,  13,  13,  13,  13,   S,   S,  13,  13], // 13
        //     0    1    2    3    4    5    6    7    8    9
        ];
        let i = state as usize;
        match c {
            "\u{09}" | "\u{20}" | "\u{a0}" => MATR[i][10 as usize],
            "," => MATR[i][0usize],
            "e" => MATR[i][2usize],
            "." => MATR[i][3usize],
            "-" => MATR[i][4usize],
            "\"" => MATR[i][5usize],
            "'" => MATR[i][6usize],
            "\n" => MATR[i][7usize],
            "\r" => MATR[i][7usize],
            "\r\n" => MATR[i][7usize],
            "\0" => MATR[i][8usize],
            _ => {
                if c.parse::<u8>().is_ok() {
                    MATR[i][1 as usize]
                } else {
                    MATR[i][9 as usize]
                }
            }
        }
    }
}

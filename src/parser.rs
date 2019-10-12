use super::automata::Automata;
use super::token::{dbgTknVec, printTknVec, CSVLine, Token};

pub struct CSVParser {
    automata: Automata,
}

impl CSVParser {
    pub fn new(file_name: &str) -> CSVParser {
        CSVParser {
            automata: Automata::new(file_name),
        }
    }

    pub fn chk_integrity(&self) -> f32 {
        0.0
    }

    pub fn parse_header(&mut self) -> (CSVLine, usize) {
        let header = self.automata.parse_line();
        let mut length: usize = 0;
        for tkn in &header {
            match &tkn {
                Token::SEP => length += 1,
                Token::EOL | Token::EOF => {
                    length += 1;
                    break;
                }
                _ => {}
            }
        }
        (header, length)
    }

    pub fn parse_line(&mut self) -> Vec<Token> {
        let line = self.automata.parse_line();
        let mut v = Vec::<Token>::new();
        for tkn in &line {
            match &tkn {
                Token::STRING(s) => {
                    let first = s.chars().next().unwrap();
                    let last = s.chars().last().unwrap();
                    if s.len() >= 2 && first == last && (first == '"' || first == '\'') {
                        let val = String::from(&s[1..s.len() - 1]);
                        v.push(Token::STRING(val));
                    } else {
                        v.push((*tkn).clone());
                    }
                }
                Token::INT(_) | Token::FLOAT(_) => v.push((*tkn).clone()),
                _ => {}
            }
        }
        print!("");
        v
    }
}

use std::iter::Peekable;
use std::os::raw::c_long;
use std::ptr::addr_of_mut;

#[derive(Debug)]
pub struct TextSpan {
    start:usize,
    end:usize,
    literal:String
}
#[derive(Debug)]
pub struct Token {
    pub(crate) kind:TokenKind,
    pub(crate) span:TextSpan
}
#[derive(Debug)]
pub enum TokenKind {
    Number(i64), // 343
    Plus,       // +
    Minus,     // -
    Asterisk,  // *
    Slash,    // /
    LeftParen, // (
    RightParen, // )
    Whitespace,
    Bad,
    Eof
}


impl TextSpan {
    pub fn new(start:usize,end:usize,literal:String) -> Self {
        Self { start, end, literal}
    }

    pub fn length(&self) -> usize {
        self.end - self.start
    }


}

impl Token {
    pub fn new(kind:TokenKind,span:TextSpan) -> Self {
        Self{ kind , span }
    }
}


pub struct Lexer <'a>{
    input: &'a str,
    current_pos:usize
}

impl <'a> Lexer <'a>  {
    pub fn new(input: &'a str) -> Self {
        Self {input,current_pos:0}
    }

    fn consume(&mut self) ->Option<char> {
        let  c = self.current_char();
        self.current_pos += 1;
        if self.current_pos > self.input.len() {
            return None;
        }
        c
    }



    fn  consumer_number(&mut self) -> i64 {
        let mut number:i64 = 0;
        while let Some(c) = self.current_char() {

            if c.is_digit(10) {
                self.consume().unwrap();
                number = number * 10 + c.to_digit(10).unwrap() as i64;
            }
            else {
                 break;
            }
        }
        number
    }


    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.current_pos)
    }
    fn is_whitespace(c:&char) ->bool {
       c.is_whitespace()
    }

    fn consume_punctuation(&mut self) -> TokenKind {
        let c = self.consume().unwrap();
        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Asterisk,
            '/' => TokenKind::Slash,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            _ => TokenKind::Bad
        }
    }



    fn is_number_start(c: &char) -> bool {
        c.is_digit(10)
    }
    pub fn next_token(&mut self) -> Option<Token>{
        if self.current_pos > self.input.len() {
            return None;
        }
        if self.current_pos == self.input.len() {
            let eof_charc = '\0';
            self.current_pos += 1;
            return Some(Token::new(
                TokenKind::Eof,
                TextSpan::new(0,0,eof_charc.to_string())
            ));
        }
       let c = self.current_char();
        return  c.map(|c|{
            let start:usize  = self.current_pos;
            let mut kind = TokenKind::Bad;

            if Self::is_number_start(&c) {
                let number = self.consumer_number();
                kind = TokenKind::Number(number)
            }
            else if Self::is_whitespace(&c)  {
                self.consume();
                kind = TokenKind::Whitespace;
            }
            else {
                kind = self.consume_punctuation();
            }
            let end = self.current_pos;
            let literal = self.input[start..end].to_string();
            let span = TextSpan::new(start,end,literal);
            Token::new(kind,span)
        })

    }
}
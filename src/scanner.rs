use crate::token::Token;
use std::str::Chars;

const RESERVED_COUNT: usize = 8;
const KEYWORDS: [&str; RESERVED_COUNT] = [
    "if", "then", "else", "end", "repeat", "until", "read", "write",
];

const KEY_TYPES: [Token; RESERVED_COUNT] = [
    Token::If,
    Token::Then,
    Token::Else,
    Token::End,
    Token::Repeat,
    Token::Until,
    Token::Read,
    Token::Write,
];

fn reserved_lookup(str: String) -> Token {
    for (wit, tit) in KEYWORDS.iter().zip(KEY_TYPES.iter()) {
        if *wit == str.as_str() {
            return tit.clone();
        }
    }
    Token::Id(str)
}

#[derive(PartialEq, Debug)]
enum StateType {
    Start,
    InAssign,
    InComment,
    InNum,
    InId,
    Done,
}

pub struct Scanner<'a> {
    //input: &'a str,
    line_position: usize,
    line_num: usize,
    it: Chars<'a>,
    next_char: Option<char>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            // input,
            line_position: 0,
            line_num: 0,
            it: input.chars(),
            next_char: None,
        }
    }
    #[allow(unused_assignments)]
    pub fn get_token(&mut self) -> Token {
        let mut token_string = String::from("");
        let mut token = Token::EndFile;
        let mut state = StateType::Start;
        let mut save = false;
        while state != StateType::Done {
            let c = self.get_peek_char();
            if c.is_none() {
                self.get_next_char();
                break;
            }
            save = true;
            let c = c.unwrap();
            if c == '\n' {
                self.line_num += 1;
                self.line_position = 0;
            } else {
                self.line_position += 1;
            }
            match state {
                StateType::Start => {
                    if c.is_ascii_digit() {
                        state = StateType::InNum;
                    } else if c.is_ascii_alphabetic() {
                        state = StateType::InId;
                    } else if c == ':' {
                        state = StateType::InAssign;
                    } else if c.is_ascii_whitespace() {
                        save = false;
                    } else if c == '{' {
                        save = false;
                        state = StateType::InComment;
                    } else {
                        state = StateType::Done;
                        match c {
                            '=' => token = Token::Eq,
                            '<' => token = Token::Lt,
                            '+' => token = Token::Plus,
                            '-' => token = Token::Minus,
                            '*' => token = Token::Times,
                            '/' => token = Token::Over,
                            '(' => token = Token::Lparen,
                            ')' => token = Token::Rparen,
                            ';' => token = Token::Semi,
                            _ => token = Token::Error(token_string.clone()),
                        }
                    }
                    self.get_next_char();
                }
                StateType::InComment => {
                    save = false;
                    if c == '}' {
                        state = StateType::Start;
                    }
                    self.get_next_char();
                }
                StateType::InAssign => {
                    state = StateType::Done;
                    if c == '=' {
                        token = Token::Assign;
                        self.get_next_char();
                    } else {
                        save = false;
                        token = Token::Error(token_string.clone());
                    }
                }
                StateType::InNum => {
                    if !c.is_ascii_digit() {
                        save = false;
                        state = StateType::Done;
                        token = Token::Num(token_string.clone());
                    } else {
                        self.get_next_char();
                    }
                }
                StateType::InId => {
                    if !c.is_ascii_alphabetic() {
                        save = false;
                        state = StateType::Done;
                        token = reserved_lookup(token_string.clone());
                    } else {
                        self.get_next_char();
                    }
                }
                StateType::Done => {}
            }
            if save {
                token_string.push(c);
            }
        }
        if state == StateType::InNum {
            token = Token::Num(token_string.clone());
        } else if state == StateType::InId {
            token = reserved_lookup(token_string.clone());
        }
        token
    }

    pub fn get_token2(&mut self) -> Token {
        let mut token = Token::EndFile;
        loop {
            let c = self.get_next_char();
            if c.is_none() {
                break;
            }
            let c = c.unwrap();
            if c.is_ascii_whitespace() {
                continue;
            }
            match c {
                '=' => {
                    token = Token::Eq;
                    break;
                }
                '<' => {
                    token = Token::Lt;
                    break;
                }
                '+' => {
                    token = Token::Plus;
                    break;
                }
                '-' => {
                    token = Token::Minus;
                    break;
                }
                '*' => {
                    token = Token::Times;
                    break;
                }
                '/' => {
                    token = Token::Over;
                    break;
                }
                '(' => {
                    token = Token::Lparen;
                    break;
                }
                ')' => {
                    token = Token::Rparen;
                    break;
                }
                ';' => {
                    token = Token::Semi;
                    break;
                }
                '{' => {
                    let mut ch = self.get_next_char();
                    while ch != Some('}') {
                        ch = self.get_next_char();
                    }
                }
                ':' => {
                    let ch = self.get_peek_char();
                    if ch == Some('=') {
                        token = Token::Assign;
                    } else {
                        // let str = ":" + ch.unwrap_or(')
                        token = Token::Error(String::from(":"));
                    }
                    self.get_next_char();
                    break;
                }
                _ => {
                    if c.is_ascii_digit() {
                        let mut num = String::from("");
                        num.push(c);
                        loop {
                            let ch = self.get_peek_char();
                            if ch.is_none() || !ch.unwrap().is_ascii_digit() {
                                break;
                            }
                            let ch = ch.unwrap();
                            num.push(ch);
                            self.get_next_char();
                        }
                        token = Token::Num(num);
                        break;
                    }
                    if c.is_ascii_alphabetic() {
                        let mut id = String::from("");
                        id.push(c);
                        loop {
                            let ch = self.get_peek_char();
                            if ch.is_none() || !ch.unwrap().is_ascii_alphanumeric() {
                                break;
                            }
                            let ch = ch.unwrap();
                            id.push(ch);
                            self.get_next_char();
                        }
                        token = reserved_lookup(id);
                        break;
                    }
                }
            }
        }
        token
    }

    fn get_next_char(&mut self) -> Option<char> {
        match self.next_char {
            Some(_) => {
                let ch = self.next_char;
                self.next_char = None;
                ch
            }
            None => self.it.next(),
        }
    }

    fn get_peek_char(&mut self) -> Option<char> {
        if self.next_char.is_none() {
            self.next_char = self.it.next();
        }
        self.next_char
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::Scanner;
    use crate::token::Token;

    #[test]
    fn test_get_token2() {
        let input = "{ Sample program
  in TINY language -
  computes factorial
}
read x; { input an integer }
if 0 < x then { don't compute if x <= 0 }
  fact := 1;
  repeat
    fact := fact * x;
    x := x - 1
  until x = 0;
  write fact  { output factorial of x }
end";

        let mut scanner = Scanner::new(input);
        let mut tokens = vec![];
        let rets = vec![
            Token::Read,
            Token::Id("x".into()),
            Token::Semi,
            Token::If,
            Token::Num("0".into()),
            Token::Lt,
            Token::Id("x".into()),
            Token::Then,
            Token::Id("fact".into()),
            Token::Assign,
            Token::Num("1".into()),
            Token::Semi,
            Token::Repeat,
            Token::Id("fact".into()),
            Token::Assign,
            Token::Id("fact".into()),
            Token::Times,
            Token::Id("x".into()),
            Token::Semi,
            Token::Id("x".into()),
            Token::Assign,
            Token::Id("x".into()),
            Token::Minus,
            Token::Num("1".into()),
            Token::Until,
            Token::Id("x".into()),
            Token::Eq,
            Token::Num("0".into()),
            Token::Semi,
            Token::Write,
            Token::Id("fact".into()),
            Token::End,
            Token::EndFile,
        ];
        loop {
            let token = scanner.get_token2();
            tokens.push(token.clone());
            if token == Token::EndFile {
                break;
            }
        }
        assert_eq!(rets, tokens);
    }

    #[test]
    fn test_get_token2_1() {
        let input = "{
  Factorial Program in TINY language
  ----------------------------------
}

read x;         { input an integer x }
if 0 < x then   { don't compute if x <= 0 }
  fact := 1;
  repeat
    fact := fact * x;
    x := x - 1;
  until x = 0;
  write fact;   { output factorial of x }
end";

        let mut scanner = Scanner::new(input);
        let mut tokens = vec![];
        let rets = vec![
            Token::Read,
            Token::Id("x".into()),
            Token::Semi,
            Token::If,
            Token::Num("0".into()),
            Token::Lt,
            Token::Id("x".into()),
            Token::Then,
            Token::Id("fact".into()),
            Token::Assign,
            Token::Num("1".into()),
            Token::Semi,
            Token::Repeat,
            Token::Id("fact".into()),
            Token::Assign,
            Token::Id("fact".into()),
            Token::Times,
            Token::Id("x".into()),
            Token::Semi,
            Token::Id("x".into()),
            Token::Assign,
            Token::Id("x".into()),
            Token::Minus,
            Token::Num("1".into()),
            Token::Semi,
            Token::Until,
            Token::Id("x".into()),
            Token::Eq,
            Token::Num("0".into()),
            Token::Semi,
            Token::Write,
            Token::Id("fact".into()),
            Token::Semi,
            Token::End,
            Token::EndFile,
        ];
        loop {
            let token = scanner.get_token2();
            tokens.push(token.clone());
            if token == Token::EndFile {
                break;
            }
        }
        assert_eq!(rets, tokens);
    }
}

use std::{error::Error, str::Chars, thread::current};

use crate::tokens::Token;

pub struct Lexer<'a> {
    code: &'a str,
    code_chars: Vec<char>,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn tokenize(&mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut tokens: Vec<Token> = vec![];

        let mut current_opcode = String::new();

        while let Some(ch) = self.skip_to_next() {
            let ch = *ch;

            let token_char = match ch {
                '.' => {
                    let mut t = Token::Dot;

                    if let (Some(next_c1), Some(next_c2)) = (self.peek_next(), self.peek_next_to(2))
                    {
                        match (next_c1, next_c2) {
                            ('.', '.') => {
                                t = Token::Spread;
                                self.skip_to_next();
                            }
                            ('?', _) => {
                                t = Token::OptionalChaining;
                                self.skip_to_next();
                            }

                            _ => {}
                        }
                    }

                    t
                }

                '!' => {
                    let mut t = Token::LogicalNot;

                    if let (Some(next_c), Some(next_c2)) = (self.peek_next(), self.peek_next_to(2))
                    {
                        match (next_c, next_c2) {
                            ('=', '=') => {
                                t = Token::StrictNotEqual;
                                self.skip_to(2);
                            }
                            ('=', _) => {
                                t = Token::NotEqual;
                                self.skip_to_next();
                            }
                            _ => {}
                        }
                    }

                    t
                }
                '&' => {
                    let mut t = Token::BitwiseAnd;

                    if let Some(next_c) = self.peek_next() {
                        match next_c {
                            '&' => {
                                t = Token::LogicalAnd;
                                self.skip_to_next();
                            }
                            _ => {}
                        }
                    }

                    t
                }
                '|' => {
                    let mut t = Token::BitwiseOr;

                    if let Some(next_c) = self.peek_next() {
                        match next_c {
                            '|' => {
                                t = Token::LogicalOr;
                                self.skip_to_next();
                            }
                            _ => {}
                        }
                    }

                    t
                }

                '(' => Token::LeftParen,
                ')' => Token::RightParen,
                '{' => Token::LeftBrace,
                '}' => Token::RightBrace,
                '[' => Token::LeftBracket,
                ']' => Token::RightBracket,

                ',' => Token::Comma,
                ':' => Token::Colon,

                '=' => Token::Assign,

                ' ' => Token::Whitespace,
                '\n' => Token::Newline,

                _ => Token::NoToken,
            };

            if token_char != Token::NoToken {
                if !current_opcode.is_empty() {
                    tokens.push(Token::Identifier(current_opcode));
                    current_opcode = String::new();
                } else {
                    tokens.push(token_char);
                    current_opcode = String::new();
                }
                continue;
            }

            current_opcode.push(ch);

            let token = match current_opcode.as_str() {
                "const" => Token::Const,
                "let" => Token::Let,
                "var" => Token::Var,
                "function" => Token::Function,
                "return" => Token::Return,
                "yield" => Token::Yield,
                "if" => Token::If,
                "else" => Token::Else,
                "switch" => Token::Switch,
                "case" => Token::Case,
                "break" => Token::Break,
                "continue" => Token::Continue,
                "default" => Token::Default,
                "for" => Token::For,
                "while" => Token::While,
                "do" => Token::Do,
                "try" => Token::Try,
                "catch" => Token::Catch,
                "finally" => Token::Finally,
                "throw" => Token::Throw,
                "class" => Token::Class,
                "extends" => Token::Extends,
                "super" => Token::Super,
                "this" => Token::This,
                "new" => Token::New,
                "import" => Token::Import,
                "export" => Token::Export,
                "from" => Token::From,
                "as" => Token::As,
                "async" => Token::Async,
                "await" => Token::Await,
                "with" => Token::With,
                "in" => Token::In,
                "of" => Token::Of,
                "instanceof" => Token::InstanceOf,
                "typeof" => Token::Typeof,
                "delete" => Token::Delete,
                "void" => Token::Void,

                "true" => Token::True,
                "false" => Token::False,
                "null" => Token::Null,
                "undefined" => Token::Undefined,

                "\"" => {
                    let mut string = String::new();

                    while let Some(c) = self.skip_to_next() {
                        match c {
                            '\"' => break,
                            _ => string.push(*c),
                        }
                    }

                    Token::String(string)
                }

                _ => Token::NoToken,
            };

            if token != Token::NoToken {
                tokens.push(token);
                current_opcode = String::new();
            }
        }

        Ok(tokens)
    }

    pub fn skip_to(&mut self, skip_count: usize) -> Option<&char> {
        self.position += skip_count;
        self.at(self.position)
    }

    pub fn skip_to_next(&mut self) -> Option<&char> {
        self.position += 1;
        self.at(self.position - 1)
    }

    pub fn peek_next(&self) -> Option<&char> {
        self.at(self.position + 1)
    }

    pub fn peek_next_to(&self, offset: usize) -> Option<&char> {
        self.at(self.position + offset)
    }

    pub fn at(&self, pos: usize) -> Option<&char> {
        self.code_chars.get(pos)
    }

    pub fn new(code: &'a str) -> Self {
        let code_chars = code.chars().collect();

        Self {
            position: 0,
            code: code,
            code_chars,
        }
    }
}

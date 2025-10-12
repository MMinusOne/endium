use crate::tokens::Token;
use std::error::Error;

pub struct Lexer {
    code_chars: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn tokenize(&mut self) -> Result<Vec<Token>, Box<dyn Error>> {
        let mut tokens: Vec<Token> = vec![];

        let mut current_opcode = String::new();

        while self.position < self.code_chars.len() {
            let ch = self.code_chars[self.position];

            if ch.is_whitespace() && ch != '\n' && ch != '\r' {
                if !current_opcode.is_empty() {
                    let token = self.match_token(&current_opcode)?;
                    if token != Token::NoToken {
                        tokens.push(token);
                    }
                    current_opcode.clear();
                }
                self.position += 1;
                continue;
            }

            let token_result = self.match_token_char(ch);

            match token_result {
                Some(token) => {
                    if !current_opcode.is_empty() {
                        let prev_token = self.match_token(&current_opcode)?;
                        if prev_token != Token::NoToken {
                            tokens.push(prev_token);
                        }
                        current_opcode.clear();
                    }
                    tokens.push(token);
                }

                None => {
                    current_opcode.push(ch);
                    self.position += 1;
                }
            }
        }

        if !current_opcode.is_empty() {
            let token = self.match_token(&current_opcode)?;
            if token != Token::NoToken {
                tokens.push(token);
            }
        }

        // tokens.push(Token::Eof);

        Ok(tokens)
    }

    fn peek_ahead(&self, offset: usize) -> Option<char> {
        self.code_chars.get(self.position + offset).copied()
    }

    pub fn match_token_char(&mut self, ch: char) -> Option<Token> {
        match ch {
            '.' => {
                if self.peek_ahead(1) == Some('.') && self.peek_ahead(2) == Some('.') {
                    self.position += 3;
                    Some(Token::Spread)
                } else if self.peek_ahead(1) == Some('?') {
                    self.position += 2;
                    Some(Token::OptionalChaining)
                } else {
                    self.position += 1;
                    Some(Token::Dot)
                }
            }

            '!' => {
                if self.peek_ahead(1) == Some('=') && self.peek_ahead(2) == Some('=') {
                    self.position += 3;
                    Some(Token::StrictNotEqual)
                } else if self.peek_ahead(1) == Some('=') {
                    self.position += 2;
                    Some(Token::NotEqual)
                } else {
                    self.position += 1;
                    Some(Token::LogicalNot)
                }
            }
            '&' => {
                if self.peek_ahead(1) == Some('&') {
                    self.position += 2;
                    Some(Token::LogicalAnd)
                } else {
                    self.position += 1;
                    Some(Token::BitwiseAnd)
                }
            }
            '|' => {
                if self.peek_ahead(1) == Some('|') {
                    self.position += 2;
                    Some(Token::LogicalOr)
                } else {
                    self.position += 1;
                    Some(Token::BitwiseOr)
                }
            }

            '(' => {
                self.position += 1;
                Some(Token::LeftParen)
            }
            ')' => {
                self.position += 1;
                Some(Token::RightParen)
            }
            '{' => {
                self.position += 1;
                Some(Token::LeftBrace)
            }
            '}' => {
                self.position += 1;
                Some(Token::RightBrace)
            }
            '[' => {
                self.position += 1;
                Some(Token::LeftBracket)
            }
            ']' => {
                self.position += 1;
                Some(Token::RightBracket)
            }

            ',' => {
                self.position += 1;
                Some(Token::Comma)
            }
            ':' => {
                self.position += 1;
                Some(Token::Colon)
            }
            ';' => {
                self.position += 1;
                Some(Token::Semicolon)
            }

            '=' => {
                if self.peek_ahead(1) == Some('=') && self.peek_ahead(2) == Some('=') {
                    self.position += 3;
                    Some(Token::StrictEqual)
                } else if self.peek_ahead(1) == Some('=') {
                    self.position += 2;
                    Some(Token::Equal)
                } else if self.peek_ahead(1) == Some('>') {
                    self.position += 2;
                    Some(Token::ArrowFunction)
                } else {
                    self.position += 1;
                    Some(Token::Assign)
                }
            }

            '<' => {
                if self.peek_ahead(1) == Some('=') {
                    self.position += 2;
                    Some(Token::LessThanOrEqual)
                } else {
                    self.position += 1;
                    Some(Token::LessThan)
                }
            }
            '>' => {
                if self.peek_ahead(1) == Some('=') {
                    self.position += 2;
                    Some(Token::GreaterThanOrEqual)
                } else {
                    self.position += 1;
                    Some(Token::GreaterThan)
                }
            }

            '+' => {
                if self.peek_ahead(1) == Some('=') {
                    self.position += 2;
                    Some(Token::PlusAssign)
                } else if self.peek_ahead(1) == Some('+') {
                    self.position += 2;
                    Some(Token::Increment)
                } else {
                    self.position += 1;
                    Some(Token::Plus)
                }
            }
            '-' => {
                if self.peek_ahead(1) == Some('=') {
                    self.position += 2;
                    Some(Token::MinusAssign)
                } else if self.peek_ahead(1) == Some('-') {
                    self.position += 2;
                    Some(Token::Decrement)
                } else {
                    self.position += 1;
                    Some(Token::Minus)
                }
            }
            '*' => {
                if self.peek_ahead(1) == Some('*') && self.peek_ahead(1) == Some('=') {
                    self.position += 2;
                    Some(Token::ExponentAssign)
                } else if self.peek_ahead(1) == Some('*') {
                    self.position += 2;
                    Some(Token::Exponent)
                } else if self.peek_ahead(1) == Some('=') {
                    self.position += 2;
                    Some(Token::MultiplyAssign)
                } else {
                    self.position += 1;
                    Some(Token::Multiply)
                }
            }
            '/' => {
                if self.peek_ahead(1) == Some('=') {
                    self.position += 2;
                    Some(Token::DivideAssign)
                } else {
                    self.position += 1;
                    Some(Token::Divide)
                }
            }
            '%' => {
                if self.peek_ahead(1) == Some('=') {
                    self.position += 2;
                    Some(Token::ModuloAssign)
                } else {
                    self.position += 1;
                    Some(Token::Modulo)
                }
            }

            '\r' | '\n' => {
                self.position += 1;
                Some(Token::Newline)
            }

            '\'' | '"' | '`' => Some(self.parse_string_or_template(ch)),

            '?' => {
                if self.peek_ahead(1) == Some('?') {
                    self.position += 2;
                    Some(Token::NullishCoalescing)
                } else if self.peek_ahead(1) == Some('.') {
                    self.position += 2;
                    Some(Token::OptionalChaining)
                } else {
                    Some(Token::Ternary)
                }
            }

            _ if ch.is_ascii_digit() => None,

            _ => None,
        }
    }

    pub fn match_token(&mut self, current_opcode: &String) -> Result<Token, Box<dyn Error>> {
        if current_opcode
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_digit())
        {
            return Ok(self.parse_number(current_opcode));
        }

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

            _ => {
                if !current_opcode.is_empty() {
                    Token::Identifier(current_opcode.to_string())
                } else {
                    Token::NoToken
                }
            }
        };

        Ok(token)
    }

    fn parse_number(&self, start: &str) -> Token {
        let mut num_str = start.to_string();
        let mut pos = self.position;

        while pos < self.code_chars.len() {
            let ch = self.code_chars[pos];
            if ch.is_ascii_digit() || ch == '.' {
                num_str.push(ch);
                pos += 1;
            } else if ch == 'n' {
                pos += 1;
                return Token::BigNumber(num_str);
            } else {
                break;
            }
        }

        Token::Number(num_str)
    }

    fn parse_string_or_template(&mut self, quote: char) -> Token {
        self.position += 1;

        if quote == '`' {
            return self.parse_template_string();
        }

        let mut string = String::new();

        while self.position < self.code_chars.len() {
            let ch = self.code_chars[self.position];
            self.position += 1;

            if ch == quote {
                break;
            } else if ch == '\\' && self.position < self.code_chars.len() {
                let next = self.code_chars[self.position];
                self.position += 1;
                string.push(match next {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '\'' => '\'',
                    '\"' => '"',
                    _ => next,
                });
            } else {
                string.push(ch);
            }
        }

        Token::String(string)
    }

    fn parse_template_string(&mut self) -> Token {
        let mut tokens = Vec::new();
        let mut current_string = String::new();

        while self.position < self.code_chars.len() {
            let ch = self.code_chars[self.position];

            if ch == '$' && self.peek_ahead(1) == Some('{') {
                if !current_string.is_empty() || tokens.is_empty() {
                    tokens.push(Token::String(current_string.clone()));
                    current_string.clear();
                }

                self.position += 2;

                let mut depth = 1;
                let mut expr = String::new();

                while self.position < self.code_chars.len() && depth > 0 {
                    let ch = self.code_chars[self.position];
                    self.position += 1;

                    if ch == '{' {
                        depth += 1;
                        expr.push(ch);
                    } else if ch == '}' {
                        depth -= 1;
                        if depth > 0 {
                            expr.push(ch)
                        }
                    } else {
                        expr.push(ch);
                    }
                }

                if let Ok(expr_tokens) = Lexer::new(&expr).tokenize() {
                    tokens.push(Token::TemplateExpr(expr_tokens));
                }
            } else if ch == '`' {
                self.position += 1;
                break;
            } else {
                current_string.push(ch);
                self.position += 1;
            }
        }

        if !current_string.is_empty() {
            tokens.push(Token::String(current_string));
        }

        Token::TemplateString(tokens)
    }

    pub fn new(code: &String) -> Self {
        let code_chars = code.chars().collect();

        Self {
            position: 0,
            code_chars,
        }
    }
}

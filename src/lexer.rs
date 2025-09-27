use std::fmt;

use crate::tokens::Token;

#[derive(Debug, Clone)]
pub struct TokenWithPosition {
    pub token: Token,
    pub line: usize,
    pub column: usize,
    pub start_pos: usize,
    pub end_pos: usize,
}

#[derive(Debug)]
pub struct LexError {
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub position: usize,
}

impl fmt::Display for LexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Lexical error at line {}, column {}: {}", 
               self.line, self.column, self.message)
    }
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
    current_char: Option<char>,
    position: usize,
    line: usize,
    column: usize,
    start_pos: usize,
    start_line: usize,
    start_column: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();
        
        Self {
            input,
            chars,
            current_char,
            position: 0,
            line: 1,
            column: 1,
            start_pos: 0,
            start_line: 1,
            start_column: 1,
        }
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<TokenWithPosition>, LexError> {
        let mut tokens = Vec::new();
        
        loop {
            let token = self.next_token()?;
            let is_eof = matches!(token.token, Token::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        
        Ok(tokens)
    }
    
    pub fn next_token(&mut self) -> Result<TokenWithPosition, LexError> {
        self.skip_whitespace();
        self.mark_start();
        
        match self.current_char {
            None => Ok(self.create_token(Token::Eof)),
            Some(ch) => self.tokenize_char(ch),
        }
    }
    
    fn tokenize_char(&mut self, ch: char) -> Result<TokenWithPosition, LexError> {
        match ch {
            // Whitespace and newlines
            '\n' | '\r' => {
                self.consume_newline();
                Ok(self.create_token(Token::Newline))
            }
            
            // Single character tokens
            ';' => {
                self.advance();
                Ok(self.create_token(Token::Semicolon))
            }
            ',' => {
                self.advance();
                Ok(self.create_token(Token::Comma))
            }
            ':' => {
                self.advance();
                Ok(self.create_token(Token::Colon))
            }
            '(' => {
                self.advance();
                Ok(self.create_token(Token::LeftParen))
            }
            ')' => {
                self.advance();
                Ok(self.create_token(Token::RightParen))
            }
            '{' => {
                self.advance();
                Ok(self.create_token(Token::LeftBrace))
            }
            '}' => {
                self.advance();
                Ok(self.create_token(Token::RightBrace))
            }
            '[' => {
                self.advance();
                Ok(self.create_token(Token::LeftBracket))
            }
            ']' => {
                self.advance();
                Ok(self.create_token(Token::RightBracket))
            }
            '~' => {
                self.advance();
                Ok(self.create_token(Token::BitwiseNot))
            }
            
            // Operators that can be multi-character
            '+' => self.tokenize_plus(),
            '-' => self.tokenize_minus(),
            '*' => self.tokenize_multiply(),
            '/' => self.tokenize_divide(),
            '%' => self.tokenize_modulo(),
            '=' => self.tokenize_equals(),
            '!' => self.tokenize_exclamation(),
            '<' => self.tokenize_less_than(),
            '>' => self.tokenize_greater_than(),
            '&' => self.tokenize_ampersand(),
            '|' => self.tokenize_pipe(),
            '^' => self.tokenize_caret(),
            '?' => self.tokenize_question(),
            '.' => self.tokenize_dot(),
            
            // String literals
            '"' | '\'' => self.tokenize_string(ch),
            '`' => self.tokenize_template_string(),
            
            // Numbers
            '0'..='9' => self.tokenize_number(),
            
            // Identifiers and keywords
            'a'..='z' | 'A'..='Z' | '_' | '$' => self.tokenize_identifier(),
            
            _ => Err(LexError {
                message: format!("Unexpected character: '{}'", ch),
                line: self.line,
                column: self.column,
                position: self.position,
            }),
        }
    }
    
    fn tokenize_plus(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance();
        match self.current_char {
            Some('+') => {
                self.advance();
                Ok(self.create_token(Token::Increment))
            }
            Some('=') => {
                self.advance();
                Ok(self.create_token(Token::PlusAssign))
            }
            _ => Ok(self.create_token(Token::Plus)),
        }
    }
    
    fn tokenize_minus(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance();
        match self.current_char {
            Some('-') => {
                self.advance();
                Ok(self.create_token(Token::Decrement))
            }
            Some('=') => {
                self.advance();
                Ok(self.create_token(Token::MinusAssign))
            }
            _ => Ok(self.create_token(Token::Minus)),
        }
    }
    
    fn tokenize_multiply(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance();
        match self.current_char {
            Some('*') => {
                self.advance();
                match self.current_char {
                    Some('=') => {
                        self.advance();
                        Ok(self.create_token(Token::ExponentAssign))
                    }
                    _ => Ok(self.create_token(Token::Exponent)),
                }
            }
            Some('=') => {
                self.advance();
                Ok(self.create_token(Token::MultiplyAssign))
            }
            _ => Ok(self.create_token(Token::Multiply)),
        }
    }
    
    fn tokenize_divide(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance();
        match self.current_char {
            Some('/') => self.tokenize_line_comment(),
            Some('*') => self.tokenize_block_comment(),
            Some('=') => {
                self.advance();
                Ok(self.create_token(Token::DivideAssign))
            }
            _ => Ok(self.create_token(Token::Divide)),
        }
    }
    
    fn tokenize_modulo(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance();
        match self.current_char {
            Some('=') => {
                self.advance();
                Ok(self.create_token(Token::ModuloAssign))
            }
            _ => Ok(self.create_token(Token::Modulo)),
        }
    }
    
    fn tokenize_equals(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance();
        match self.current_char {
            Some('=') => {
                self.advance();
                match self.current_char {
                    Some('=') => {
                        self.advance();
                        Ok(self.create_token(Token::StrictEqual))
                    }
                    _ => Ok(self.create_token(Token::Equal)),
                }
            }
            Some('>') => {
                self.advance();
                Ok(self.create_token(Token::ArrowFunction))
            }
            _ => Ok(self.create_token(Token::Assign)),
        }
    }
    
    fn tokenize_exclamation(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance();
        match self.current_char {
            Some('=') => {
                self.advance();
                match self.current_char {
                    Some('=') => {
                        self.advance();
                        Ok(self.create_token(Token::StrictNotEqual))
                    }
                    _ => Ok(self.create_token(Token::NotEqual)),
                }
            }
            _ => Ok(self.create_token(Token::LogicalNot)),
        }
    }
    
    fn tokenize_less_than(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance();
        match self.current_char {
            Some('=') => {
                self.advance();
                Ok(self.create_token(Token::LessThanOrEqual))
            }
            Some('<') => {
                self.advance();
                Ok(self.create_token(Token::LeftShift))
            }
            _ => Ok(self.create_token(Token::LessThan)),
        }
    }
    
    fn tokenize_greater_than(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance();
        match self.current_char {
            Some('=') => {
                self.advance();
                Ok(self.create_token(Token::GreaterThanOrEqual))
            }
            Some('>') => {
                self.advance();
                match self.current_char {
                    Some('>') => {
                        self.advance();
                        Ok(self.create_token(Token::UnsignedRightShift))
                    }
                    _ => Ok(self.create_token(Token::RightShift)),
                }
            }
            _ => Ok(self.create_token(Token::GreaterThan)),
        }
    }
    
    fn tokenize_ampersand(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance();
        match self.current_char {
            Some('&') => {
                self.advance();
                Ok(self.create_token(Token::LogicalAnd))
            }
            _ => Ok(self.create_token(Token::BitwiseAnd)),
        }
    }
    
    fn tokenize_pipe(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance();
        match self.current_char {
            Some('|') => {
                self.advance();
                Ok(self.create_token(Token::LogicalOr))
            }
            _ => Ok(self.create_token(Token::BitwiseOr)),
        }
    }
    
    fn tokenize_caret(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance();
        Ok(self.create_token(Token::BitwiseXor))
    }
    
    fn tokenize_question(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance();
        match self.current_char {
            Some('?') => {
                self.advance();
                Ok(self.create_token(Token::NullishCoalescing))
            }
            Some('.') => {
                self.advance();
                Ok(self.create_token(Token::OptionalChaining))
            }
            _ => Ok(self.create_token(Token::Ternary)),
        }
    }
    
    fn tokenize_dot(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance();
        match self.current_char {
            Some('.') => {
                self.advance();
                match self.current_char {
                    Some('.') => {
                        self.advance();
                        Ok(self.create_token(Token::Spread))
                    }
                    _ => Err(LexError {
                        message: "Invalid token '..'".to_string(),
                        line: self.line,
                        column: self.column,
                        position: self.position,
                    }),
                }
            }
            Some('0'..='9') => {
                // Handle numbers starting with .
                self.position = self.start_pos;
                self.column = self.start_column;
                self.line = self.start_line;
                self.current_char = Some('.');
                self.tokenize_number()
            }
            _ => Ok(self.create_token(Token::Dot)),
        }
    }
    
    fn tokenize_line_comment(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance(); // consume second '/'
        let mut comment = String::new();
        
        while let Some(ch) = self.current_char {
            if ch == '\n' || ch == '\r' {
                break;
            }
            comment.push(ch);
            self.advance();
        }
        
        Ok(self.create_token(Token::Comment(comment)))
    }
    
    fn tokenize_block_comment(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance(); // consume '*'
        let mut comment = String::new();
        
        while let Some(ch) = self.current_char {
            if ch == '*' {
                self.advance();
                if let Some('/') = self.current_char {
                    self.advance();
                    break;
                }
                comment.push('*');
            } else {
                comment.push(ch);
                self.advance();
            }
        }
        
        Ok(self.create_token(Token::BlockComment(comment)))
    }
    
    fn tokenize_string(&mut self, quote: char) -> Result<TokenWithPosition, LexError> {
        self.advance(); // consume opening quote
        let mut string = String::new();
        let mut escaped = false;
        
        while let Some(ch) = self.current_char {
            if escaped {
                match ch {
                    'n' => string.push('\n'),
                    't' => string.push('\t'),
                    'r' => string.push('\r'),
                    '\\' => string.push('\\'),
                    '\'' => string.push('\''),
                    '"' => string.push('"'),
                    '0' => string.push('\0'),
                    _ => {
                        string.push('\\');
                        string.push(ch);
                    }
                }
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote {
                self.advance(); // consume closing quote
                return Ok(self.create_token(Token::String(string)));
            } else {
                string.push(ch);
            }
            self.advance();
        }
        
        Err(LexError {
            message: "Unterminated string literal".to_string(),
            line: self.start_line,
            column: self.start_column,
            position: self.start_pos,
        })
    }
    
    fn tokenize_template_string(&mut self) -> Result<TokenWithPosition, LexError> {
        self.advance(); // consume opening backtick
        let mut string = String::new();
        
        while let Some(ch) = self.current_char {
            if ch == '`' {
                self.advance(); // consume closing backtick
                return Ok(self.create_token(Token::TemplateString(string)));
            } else if ch == '$' {
                if let Some('{') = self.peek() {
                    // Handle template interpolation - for now just include it in the string
                    string.push(ch);
                    self.advance();
                    string.push('{');
                    self.advance();
                    
                    let mut brace_count = 1;
                    while let Some(ch) = self.current_char {
                        string.push(ch);
                        match ch {
                            '{' => brace_count += 1,
                            '}' => {
                                brace_count -= 1;
                                if brace_count == 0 {
                                    self.advance();
                                    break;
                                }
                            }
                            _ => {}
                        }
                        self.advance();
                    }
                } else {
                    string.push(ch);
                    self.advance();
                }
            } else {
                string.push(ch);
                self.advance();
            }
        }
        
        Err(LexError {
            message: "Unterminated template literal".to_string(),
            line: self.start_line,
            column: self.start_column,
            position: self.start_pos,
        })
    }
    
    fn tokenize_number(&mut self) -> Result<TokenWithPosition, LexError> {
        let mut number = String::new();
        let mut has_dot = false;
        
        // Handle leading dot
        if let Some('.') = self.current_char {
            has_dot = true;
            number.push('.');
            self.advance();
        }
        
        while let Some(ch) = self.current_char {
            match ch {
                '0'..='9' => {
                    number.push(ch);
                    self.advance();
                }
                '.' if !has_dot => {
                    has_dot = true;
                    number.push(ch);
                    self.advance();
                }
                'e' | 'E' => {
                    number.push(ch);
                    self.advance();
                    if let Some('+' | '-') = self.current_char {
                        number.push(self.current_char.unwrap());
                        self.advance();
                    }
                    while let Some('0'..='9') = self.current_char {
                        number.push(self.current_char.unwrap());
                        self.advance();
                    }
                    break;
                }
                _ => break,
            }
        }
        
        Ok(self.create_token(Token::Number(number)))
    }
    
    fn tokenize_identifier(&mut self) -> Result<TokenWithPosition, LexError> {
        let mut identifier = String::new();
        
        while let Some(ch) = self.current_char {
            match ch {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '$' => {
                    identifier.push(ch);
                    self.advance();
                }
                _ => break,
            }
        }
        
        let token = match identifier.as_str() {
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
            _ => Token::Identifier(identifier),
        };
        
        Ok(self.create_token(token))
    }
    
    fn advance(&mut self) {
        if let Some(ch) = self.current_char {
            self.position += ch.len_utf8();
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
        }
        self.current_char = self.chars.next();
    }
    
    fn peek(&self) -> Option<char> {
        self.chars.as_str().chars().next()
    }
    
    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            match ch {
                ' ' | '\t' => self.advance(),
                _ => break,
            }
        }
    }
    
    fn consume_newline(&mut self) {
        if let Some('\r') = self.current_char {
            self.advance();
            if let Some('\n') = self.current_char {
                self.advance();
            }
        } else if let Some('\n') = self.current_char {
            self.advance();
        }
    }
    
    fn mark_start(&mut self) {
        self.start_pos = self.position;
        self.start_line = self.line;
        self.start_column = self.column;
    }
    
    fn create_token(&self, token: Token) -> TokenWithPosition {
        TokenWithPosition {
            token,
            line: self.start_line,
            column: self.start_column,
            start_pos: self.start_pos,
            end_pos: self.position,
        }
    }
}



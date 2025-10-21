use super::token::{Token, TokenType};
use colored::*;

pub struct Lexer {
    source: String,
    file: String,
    index: usize,
    line: usize,
    column: usize,
    err: bool,
}

impl Lexer {
    pub fn new(source: String, file: String) -> Self {
        Self {
            source,
            file,
            index: 0,
            line: 1,
            column: 1,
            err: true,
        }
    }

    pub fn peek(&mut self) -> Token {

        // temporarily save the current position value
        let index = self.index;
        let line = self.line;
        let column = self.column;

        // get current token
        let token = self.peek_next();

        // return the previous position value
        self.index = index;
        self.line = line;
        self.column = column;

        token
    }

    pub fn peek_next(&mut self) -> Token {
        self.skip_whitespace();
       
        if self.peek_char() == '/' {
            self.advance();
            if self.peek_char() == '/' {
                self.skip_inline_comment();
            } else if self.peek_char() == '*' {
                self.skip_multiline_comment();
            } else {
                return Token::new(TokenType::Slash, "/".to_owned(), self.file.clone(), self.line, self.column, self.index - 1, self.index);
            }
        }

        self.skip_whitespace();

        let mut token = Token::new(TokenType::Eof, "\0".to_owned(), self.file.clone(), self.line, self.column, self.index, self.index + 1);

        if !self.is_at_end() {
            if self.peek_char().is_ascii_digit() {
                token = self.collect_number();
            } else if self.peek_char() == '"' || self.peek_char() == '\'' {
                token = self.collect_string();
            } else if self.peek_char().is_alphabetic() || self.peek_char() == '_' {
                token = self.collect_identifier();
            } else if self.peek_char().is_ascii_punctuation() {
                token = self.collect_punctuation();
            } else { 
                println!("{} {}:{}", "error:".red(), self.file, self.line);
                println!("└───unknown token.");
                self.err = true;
                std::process::exit(1);
            }
        }

        token
    }

    fn collect_number(&mut self) -> Token {
        let start = self.index;
        let mut value = String::new();

        while self.peek_char().is_ascii_digit() {
            value.push(self.peek_char());
            self.advance();
        }

        if self.peek_char() == '.' {
            value.push(self.peek_char());
            self.advance();

            if !self.peek_char().is_ascii_digit() {
                println!("{} {}:{}", "error:".red(), self.file, self.line);
                println!("└───expected ascii digit.");
                self.err = true;
            }

            while self.peek_char().is_ascii_digit() {
                value.push(self.peek_char());
                self.advance();
            }
        }

        if self.peek_char() == 'e' || self.peek_char() == 'E' {
            value.push(self.peek_char());
            self.advance();

            if self.peek_char() == '-' || self.peek_char() == '+' {
                value.push(self.peek_char());
                self.advance();
            }

            if !self.peek_char().is_ascii_digit() {
                println!("{} {}:{}", "error:".red(), self.file, self.line);
                println!("└───expected ascii digit.");
                self.err = true;
            }

            while self.peek_char().is_ascii_digit() {
                value.push(self.peek_char());
                self.advance();
            }
        }

        Token::new(TokenType::Number, value, self.file.clone(), self.line, self.column, start, self.index)
    }

    fn collect_string(&mut self) -> Token {
        let start = self.index;
        let line = self.line;
        let column = self.column;

        let markup = self.peek_char();
        self.advance();

        let mut value = String::new();

        while !self.is_at_end() && self.peek_char() != markup {
            if self.peek_char() == '\\' {
                self.advance();
                if self.peek_char() == '\\' {
                    value.push(self.peek_char());
                    self.advance();
                } else if self.peek_char() == 'n' {
                    value.push('\n');
                    self.advance();
                } else if self.peek_char() == 't' {
                    value.push('\t');
                    self.advance();
                } else if self.peek_char() == 'r' {
                    value.push('\r');
                    self.advance();
                } else if self.peek_char() == '0' {
                    value.push('\0');
                    self.advance();
                } else if self.peek_char() == markup {
                    value.push(self.peek_char());
                    self.advance();
                } else {
                    println!("{} {}:{}", "error:".red(), self.file, self.line);
                    println!("└───unknown escape detection.");
                    self.err = true;
                }

                continue;
            } else if self.peek_char() == '\n' || self.peek_char() == '\r' || self.peek_char() == '\t' || self.peek_char() == '\0' {
                println!("{} {}:{}", "error:".red(), self.file, self.line);
                println!("└───unknown escape detection.");
                self.err = true;
            }

            value.push(self.peek_char());
            self.advance();
        }

        if self.peek_char() == markup {
            self.advance();
        } else {
            println!("{} {}:{}", "error:".red(), self.file, self.line);
            println!("└───expected close quotes.");
            self.err = true;
        }

        Token::new(TokenType::StringLiteral, value, self.file.clone(), line, column, start, self.index)
    }

    fn collect_identifier(&mut self) -> Token {
        let start = self.index;
        let column = self.column;

        let mut value = String::new();

        while self.peek_char().is_alphanumeric() || self.peek_char() == '_' {
            value.push(self.peek_char());
            self.advance();
        }

        let kind = match value.as_str() {
            "func" => TokenType::Func,
            "if" => TokenType::If,
            "let" => TokenType::Let,
            "return" => TokenType::Return,
            "else" => TokenType::Else,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "struct" => TokenType::Struct,
            "enum" => TokenType::Enum,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "default" => TokenType::Default,
            "match" => TokenType::Match,
            "case" => TokenType::Case,
            "await" => TokenType::Await,
            "async" => TokenType::Async,
            "import" => TokenType::Import,
            "use" => TokenType::Use,
            "is" => TokenType::Is,
            "alias" => TokenType::Alias,
            "typename" => TokenType::TypeName,
            "public" => TokenType::Public,
            _ => TokenType::Identifier,
        };

        Token::new(kind, value, self.file.clone(), self.line, column, start, self.index)
    }

    fn collect_punctuation(&mut self) -> Token {
        let start = self.index;
        let column = self.column;

        let ch1 = self.peek_char();
        self.advance();

        let ch2 = self.peek_char();
        let two_chars = format!("{}{}", ch1, ch2);

        let (kind, value, advance_extra) = match two_chars.as_str() {
            "==" => (TokenType::DoubleEqual, two_chars, true),
            "!=" => (TokenType::NotEqual, two_chars, true),
            "+=" => (TokenType::PlusEqual, two_chars, true),
            "-=" => (TokenType::MinusEqual, two_chars, true),
            "*=" => (TokenType::StarEqual, two_chars, true),
            "%=" => (TokenType::PercentEqual, two_chars, true),
            "^=" => (TokenType::PowerEqual, two_chars, true),
            "/=" => (TokenType::SlashEqual, two_chars, true),
            "::" => (TokenType::DoubleColon, two_chars, true),
            "<=" => (TokenType::LessEqual, two_chars, true),
            ">=" => (TokenType::GreaterEqual, two_chars, true),
            "->" => (TokenType::Arrow, two_chars, true),
            _ => {
                let kind = match ch1 {
                    '+' => TokenType::Plus,
                    '-' => TokenType::Minus,
                    '*' => TokenType::Star,
                    '/' => TokenType::Slash,
                    '%' => TokenType::Percent,
                    '=' => TokenType::Equal,
                    '^' => TokenType::Power,
                    '!' => TokenType::Not,
                    '<' => TokenType::Less,
                    '>' => TokenType::Greater,
                    '(' => TokenType::OpenParent,
                    ')' => TokenType::CloseParent,
                    '{' => TokenType::OpenBrace,
                    '}' => TokenType::CloseBrace,
                    '[' => TokenType::OpenBracket,
                    ']' => TokenType::CloseBracket,
                    ';' => TokenType::SemiColon,
                    ':' => TokenType::Colon,
                    ',' => TokenType::Comma,
                    '.' => TokenType::Dot,
                    '&' => TokenType::Ampersand,
                    '$' => TokenType::Dollar,
                    '@' => TokenType::At,
                    '#' => TokenType::Hash,
                    '?' => TokenType::Null,
                    _ => TokenType::Error,
                };
                (kind, ch1.to_string(), false)
            }
        };

        if kind == TokenType::Error {
            println!("{} {}:{}", "error:".red(), self.file, self.line);
            println!("└───unknown token.");
            self.err = true;
        }

        if advance_extra {
            self.advance();
        }

        Token::new(kind, value, self.file.clone(), self.line, column, start, self.index)
    }

    fn peek_char(&self) -> char {
        self.source[self.index..].chars().next().unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        while self.peek_char().is_ascii_whitespace() {
            self.advance();
        }
    }

    fn skip_inline_comment(&mut self) {
        while self.peek_char() != '\n' {
            self.advance();
        }
    }

    fn skip_multiline_comment(&mut self) {
        loop {
            if self.peek_char() == '/' {
                self.advance();
                if self.peek_char() == '*' {
                    self.advance();
                    break;
                }
            }
            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        if self.peek_char() == '\0' { true }
        else { false }
    }

    fn advance(&mut self) {

        if self.peek_char() == '\n' {
            self.line += 1;
            self.column = 0;
        }

        self.column += 1;
        self.index += 1;
    }
}

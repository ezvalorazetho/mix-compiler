use super::lexer::Lexer;
use super::token::{TokenType, Token};
use super::node::{Node, Location};

use colored::*;

pub struct Scanner {
    lexer: Lexer,
    debug: bool,
    err: bool,
}

impl Scanner {
    pub fn new(lexer: Lexer, debug: bool) -> Self {
        Self { lexer, debug, err: true }
    }

    pub fn scan(&mut self) -> Vec<Box<Node>> {
        let mut node = Vec::new();

        if self.debug {
            self.debug("Starting Parsing".to_string());
        }

        while !self.is(TokenType::Eof) {
            if self.is_next(TokenType::Func) {
                node.push(Box::new(self.parse_funcdef(false)));
            } else if self.is_next(TokenType::Public) {
                if self.is_next(TokenType::Func) {
                    node.push(Box::new(self.parse_funcdef(true)));
                } else {
                    let info = self.lexer.peek_next();
                    println!("{} {}:{}", "error:".red(), info.file, info.line);
                    println!("└───unexpected `{}`", info.value);
                    self.err = true;
                }
            } else {
                let info = self.lexer.peek_next();
                println!("{} {}:{}", "error:".red(), info.file, info.line);
                println!("└───unexpected `{}`", info.value);
                self.err = true;
            }
        }

        node
    }

    fn parse_funcdef(&mut self, public: bool) -> Node {
        
        if !self.is(TokenType::Identifier) {
            let info = self.lexer.peek_next();
            println!("{} {}:{}", "error:".red(), info.file, info.line);
            println!("└───expected function identifier");
            self.err = true;
        }

        let token = self.lexer.peek_next();
        let name = token.value.clone();

        let location = self.create_loc(token);

        let mut args = Vec::new();
        if self.is_next(TokenType::OpenParent) {
            while !self.is(TokenType::CloseParent) {
                if self.is(TokenType::Comma) {
                    let info = self.lexer.peek_next();
                    println!("{} {}:{}", "error:".red(), info.file, info.line);
                    println!("└───unexpected comma `,`");
                    self.err = true;
                } else if self.is(TokenType::Identifier) {
                    
                    args.push(Box::new(self.parse_letdef()));
                   
                    if self.is(TokenType::CloseParent) { break; }

                        if !self.is_next(TokenType::Comma) {

                            if !self.is(TokenType::CloseParent) && !self.is(TokenType::Identifier) {
                                break;
                            }
                            
                            let info = self.lexer.peek();
                            println!("{} {}:{}", "error:".red(), info.file, info.line);
                            println!("└───expected comma `,`, but got `{}`", info.value);
                            self.err = true;

                            if !self.is_expr() {
                                break;
                            }
                        } else if !self.is(TokenType::Identifier) {
                            let info = self.lexer.peek();
                            println!("{} {}:{}", "error:".red(), info.file, info.line);
                            println!("└───unexpected comma `,`");
                            self.err = true;
                            break;
                        }
                }
            }

            if !self.is_next(TokenType::CloseParent) {
                let info = self.lexer.peek();
                println!("{} {}:{}", "error:".red(), info.file, info.line);
                println!("└───expected close parenthise `)`, but got `{}`", info.value);
                self.err = true;
            }
        } else {
            let info = self.lexer.peek();
            println!("{} {}:{}", "error:".red(), info.file, info.line);
            println!("└───expected open parenthise `(`, but got `{}`", info.value);
            self.err = true;
        }

        let mut return_type = Box::new(Node::Void);
        if self.is_next(TokenType::Arrow) {
            if self.is(TokenType::Identifier) {
                return_type = Box::new(self.parse_type());
            } else {
                let info = self.lexer.peek();
                println!("{} {}:{}", "error:".red(), info.file, info.line);
                println!("└───expected type identifier");
                self.err = true;
            }
        }

        let mut body = Vec::new();
        if self.is_next(TokenType::OpenBrace) {
            while !self.is(TokenType::CloseBrace) && !self.is(TokenType::Eof) {
                if self.is_next(TokenType::Let) {
                    body.push(Box::new(self.parse_letdef()));
                } else {
                    let info = self.lexer.peek_next();
                    println!("{} {}:{}", "error:".red(), info.file, info.line);
                    println!("└───unexpected `{}`", info.value);
                    self.err = true;
                }
            }

            if !self.is_next(TokenType::CloseBrace) {
                let info = self.lexer.peek();
                println!("{} {}:{}", "error:".red(), info.file, info.line);
                println!("└───expected close brace `}}`, but got `{}`", info.value);
                self.err = true;
            }
        } else {
            let info = self.lexer.peek();
            println!("{} {}:{}", "error:".red(), info.file, info.line);
            println!("└───expected open brace `{{`, but got `{}`", info.value);
            self.err = true;
        }

        Node::FuncDef {
            name: name,
            public: public,
            args: args,
            rtype: return_type,
            body: body,
            loc: location,
        }
    }

    fn parse_type(&mut self) -> Node {

        if !self.is(TokenType::Identifier) {
            let info = self.lexer.peek();
            println!("{} {}:{}", "error:".red(), info.file, info.line);
            println!("└───expected identifier");
            self.err = true;
        }
        
        let mut token = self.lexer.peek_next();
        
        let mut node = Node::Var {
            value: token.value.clone(),
            loc: self.create_loc(token),
        };

        if self.is_next(TokenType::DoubleColon) {
            if self.is(TokenType::Identifier) {
                let token = self.lexer.peek();
                let location = self.create_loc(token);
                node = Node::MemLockup {
                    targ: Box::new(self.parse_type()),
                    obj: Box::new(node),
                    loc: location
                };
            } else {
                let info = self.lexer.peek();
                println!("{} {}:{}", "error:".red(), info.file, info.line);
                println!("└───expected identifier");
                self.err = true;
            }
        } else if self.is_next(TokenType::Less) {
            if !self.is(TokenType::Identifier) {
                let info = self.lexer.peek();
                println!("{} {}:{}", "error:".red(), info.file, info.line);
                println!("└───expected identifier");
                self.err = true;
            }

            token = self.lexer.peek_next();
            let left = Node::Var {
                value: token.value.clone(),
                loc: self.create_loc(token.clone())
            };

            if self.is_next(TokenType::Comma) {
                let right = self.parse_type();

                if !self.is_next(TokenType::Greater) {
                    let info = self.lexer.peek();
                    println!("{} {}:{}", "error:".red(), info.file, info.line);
                    println!("└───expected greater `>`");
                    self.err = true;
                }

                return Node::DictType {
                    dictname: Box::new(node),
                    dictype: Box::new((left, right)),
                    loc: self.create_loc(token)
                };
            } else {
                
                if !self.is_next(TokenType::Greater) {
                    let info = self.lexer.peek();
                    println!("{} {}:{}", "error:".red(), info.file, info.line);
                    println!("└───expected greater `>`");
                    self.err = true;
                }

                return Node::ListType {
                    listname: Box::new(node),
                    listtype: Box::new(left),
                    loc: self.create_loc(token)
                }
            }
        }

        node
    }

    fn parse_letdef(&mut self) -> Node {
        let token = self.lexer.peek_next();

        let name = token.value.clone();
        let location = self.create_loc(token);

        let mut data_type = Node::Null {
            value: "null".to_owned(),
            loc: location.clone(),
        };

        if self.is_next(TokenType::Colon) {
            data_type = self.parse_type();
        }

        let mut value = Node::Null {
            value: "null".to_owned(),
            loc: location.clone(),
        };

        if self.is_next(TokenType::Equal) {
            value = self.parse_expr();
        }

        self.expect_semicolon();

        Node::LetDef {
            name: name,
            dtype: Box::new(data_type),
            value: Box::new(value),
            loc: location,
        }
    }

    fn parse_expr(&mut self) -> Node {
        let mut left = self.parse_logical();

        if self.is(TokenType::Or) {
            let token = self.lexer.peek_next();
 
            left = Node::BinaryOp {
                lhs: Box::new(left),
                rhs: Box::new(self.parse_expr()),
                opr: token.value.clone(),
                loc: self.create_loc(token),
            };
        }

        return left;
    }
 
    fn parse_logical(&mut self) -> Node {
        let mut left = self.parse_equality();

        if self.is(TokenType::And) {
            let token = self.lexer.peek_next();
            
            left = Node::BinaryOp {
                lhs: Box::new(left),
                rhs: Box::new(self.parse_expr()),
                opr: token.value.clone(),
                loc: self.create_loc(token),
            };
        }

        return left;
    }

    fn parse_equality(&mut self) -> Node {
        let mut left = self.parse_overrounded();

        if self.is(TokenType::DoubleEqual) || self.is(TokenType::NotEqual) {
            let token = self.lexer.peek_next();
            
            left = Node::BinaryOp {
                lhs: Box::new(left),
                rhs: Box::new(self.parse_expr()),
                opr: token.value.clone(),
                loc: self.create_loc(token),
            };
        }

        return left;
    }

    fn parse_overrounded(&mut self) -> Node {
        let mut left = self.parse_high();

        if self.is(TokenType::Less) || 
            self.is(TokenType::Greater) || 
            self.is(TokenType::LessEqual) || 
            self.is(TokenType::GreaterEqual) {

            let token = self.lexer.peek_next();

            left = Node::BinaryOp {
                lhs: Box::new(left),
                rhs: Box::new(self.parse_expr()),
                opr: token.value.clone(),
                loc: self.create_loc(token),
            };
        }

        return left;
    }

    fn parse_high(&mut self) -> Node {
        let mut left = self.parse_low();

        if self.is(TokenType::Star) || 
            self.is(TokenType::Slash) ||
            self.is(TokenType::Percent) {

            let token = self.lexer.peek_next();

            left = Node::BinaryOp {
                lhs: Box::new(left),
                rhs: Box::new(self.parse_expr()),
                opr: token.value.clone(),
                loc: self.create_loc(token),
            };
        }

        return left;
    }

    fn parse_low(&mut self) -> Node {
        let mut left = self.parse_unary();

        if self.is(TokenType::Plus) || 
            self.is(TokenType::Minus) {

            let token = self.lexer.peek_next();

            left = Node::BinaryOp {
                lhs: Box::new(left),
                rhs: Box::new(self.parse_expr()),
                opr: token.value.clone(),
                loc: self.create_loc(token),
            };
        }

        return left;
    }

    fn parse_unary(&mut self) -> Node {
        if self.is(TokenType::Plus) || 
            self.is(TokenType::Minus) || 
            self.is(TokenType::Not) {

            let token = self.lexer.peek_next();

            return Node::UnaryOp {
                value: Box::new(self.parse_expr()),
                opr: token.value.clone(),
                loc: self.create_loc(token),
            };
        }

        return self.parse_parenthises();
    }

    fn parse_parenthises(&mut self) -> Node {
        if self.is(TokenType::OpenParent) {
            let node = self.parse_expr();

            if !self.is_next(TokenType::CloseParent) {
                let info = self.lexer.peek();
                    println!("{} {}:{}", "error:".red(), info.file, info.line);
                println!("└───expected close parenthise `}}`");
                self.err = true;
            }

            return node;
        }

        return self.parse_literal();
    }

    fn parse_literal(&mut self) -> Node {
        let token = self.lexer.peek();
        let location = self.create_loc(token.clone());

        if self.is_next(TokenType::StringLiteral) {
            return Node::Str {
                value: token.value,
                loc: location,
            };
        } 
        else if self.is_next(TokenType::Number) {
            
            let value = token.value.clone();

            if let Ok(_) = value.parse::<i32>() {
                return Node::Int {
                    value: token.value,
                    lbit: false,
                    loc: location,
                };
            } else if let Ok(_) = value.parse::<i64>() {
                return Node::Int {
                    value: token.value,
                    lbit: true,
                    loc: location,
                };
            } else if let Ok(_) = value.parse::<f32>() {
                return Node::Float {
                    value: token.value,
                    lbit: false,
                    loc: location,
                };
            } else if let Ok(_) = value.parse::<i64>() {
                return Node::Float {
                    value: token.value,
                    lbit: true,
                    loc: location,
                };
            } else {                 
                let info = self.lexer.peek();
                println!("{} {}:{}", "error:".red(), info.file, info.line);
                println!("└───cannot convert number");
                std::process::exit(1);
            }
        } 
        else if self.is_next(TokenType::True) || self.is_next(TokenType::False) {
            return Node::Bool {
                value: token.value,
                loc: location,
            };
        } 
        else if self.is_next(TokenType::Identifier) {
            return self.parse_id();
        } 
        else if self.is_next(TokenType::Null) {}
        else if self.is_next(TokenType::OpenBracket) {

            let mut element = Vec::new();

            while !self.is(TokenType::CloseBracket) && !self.is(TokenType::Eof) {

                if self.is(TokenType::Comma) {
                    let info = self.lexer.peek_next();
                    println!("{} {}:{}", "error:".red(), info.file, info.line);
                    println!("└───unexpected comma `,`");
                    self.err = true;
                } else if self.is_expr() {
                    element.push(Box::new(self.parse_expr()));

                    if self.is(TokenType::CloseBracket) { break; }

                    if !self.is_next(TokenType::Comma) {

                        if !self.is(TokenType::CloseBracket) && !self.is_expr() {
                            break;
                        }
                        
                        let info = self.lexer.peek();
                        println!("{} {}:{}", "error:".red(), info.file, info.line);
                        println!("└───expected comma `,`");
                        self.err = true;

                        if !self.is_expr() {
                            break;
                        }
                    } else if !self.is_expr() {
                        let info = self.lexer.peek();
                        println!("{} {}:{}", "error:".red(), info.file, info.line);
                        println!("└───expected value expr");
                        self.err = true;
                        break;
                    }
                }
            }

            if !self.is_next(TokenType::CloseBracket) {
                let info = self.lexer.peek();
                println!("{} {}:{}", "error:".red(), info.file, info.line);
                println!("└───expected close bracket `]`");
                self.err = true;
            }

            return Node::List {
                element: element,
                loc: location,
            };
        }
        else if self.is_next(TokenType::OpenBrace) {

            let mut key_value = Vec::new();

            while !self.is(TokenType::CloseBrace) && !self.is(TokenType::Eof) {
                let left = self.parse_expr();

                if !self.is_next(TokenType::Colon) {
                    
                    let info = self.lexer.peek();
                    println!("{} {}:{}", "error:".red(), info.file, info.line);
                    println!("└───expected colon `:`");
                    self.err = true;
                    break;
                }

                let right = self.parse_expr();
                key_value.push(Box::new((left, right)));

                if self.is(TokenType::CloseBrace) { break; }

                if !self.is_next(TokenType::Comma) { 

                    let info = self.lexer.peek_next();
                    println!("{} {}:{}", "error:".red(), info.file, info.line);
                    println!("└───expected comma `,`");
                    self.err = true;

                    break; 
                }
            }

            if !self.is_next(TokenType::CloseBrace) {
                let info = self.lexer.peek();
                println!("{} {}:{}", "error:".red(), info.file, info.line);
                println!("└───expected close brace `}}`");
                self.err = true;
            }

            return Node::Dict {
                key_value: key_value,
                loc: location,
            };
        }
        else {
            let info = self.lexer.peek();
            println!("{} {}:{}", "error:".red(), info.file, info.line);
            println!("└───unknown value");
            self.err = true;
        }

        Node::Null {
            value: token.value,
            loc: location,
        }
    }

    fn is_expr(&mut self) -> bool {
        if self.is(TokenType::Null) || self.is(TokenType::StringLiteral) ||
        self.is(TokenType::Number) || self.is(TokenType::True) || 
        self.is(TokenType::False) || self.is(TokenType::OpenBrace) ||
        self.is(TokenType::OpenBracket) || self.is(TokenType::OpenParent) {
                return true;
        }

        false
    }

    fn parse_id(&mut self) -> Node {
        let mut token = self.lexer.peek_next();

        let mut location = self.create_loc(token.clone());

        let mut node = Node::Var {
            value: token.value,
            loc: location,
        };

        if self.is_next(TokenType::Dot) {
            if !self.is(TokenType::Identifier) {
                let info = self.lexer.peek();
                    println!("{} {}:{}", "error:".red(), info.file, info.line);
                println!("└───expected identifier");
                self.err = true;
            } else {
                token = self.lexer.peek();
                
                location = self.create_loc(token);
                node = Node::MemLockup {
                    targ: Box::new(self.parse_id()),
                    obj: Box::new(node),
                    loc: location,
                };
            }
        } else if self.is_next(TokenType::OpenParent) {

        }
        
        node
    }

    fn expect_semicolon(&mut self) {
        if !self.is_next(TokenType::SemiColon) {
            let info = self.lexer.peek();
                    println!("{} {}:{}", "error:".red(), info.file, info.line);
            println!("└───expected semicolon `;`");
            self.err = true;
        }
    }


    fn is_next(&mut self, t: TokenType) -> bool {
        if self.is(t) { 
            self.lexer.peek_next();
            true
        } else { false }
    }

    fn is(&mut self, t: TokenType) -> bool {
        if self.lexer.peek().kind == t {
            true
        } else { false }
    }

    fn create_loc(&self, token: Token) -> Location {
        Location {
            line: token.line,
            column: token.column,
            start: token.start,
            end: token.end,
            file: token.file,
        }
    }

    fn debug(&self, msg: String) {
        if self.debug {
            println!("[SCANNER] {}", msg);
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum DfaState {
    Start, 
    Identifier, 
    Zero,
    Number,

    Bracket,

    Plus, 
    Minus, 
    Multiply, 
    Div, 

    Assignment, 
    Equals,

    LessThan, 
    LessThanEquals,
    GreaterThan, 
    GreaterThanEquals, 

    Not, 
    NotEquals,

    BitwiseAnd,
    LogicalAnd, 

    BitwiseOr, 
    LogicalOr,

    Dot, 
    Comma,
    Semicolon,
    Colon, 

    Exit,  
    Error,
}

/// Hand coded DFA for mini java programming language. 
impl DfaState {
    pub fn next_state(self, next_char: char) -> DfaState {
        match self {
            DfaState::Start => self.start(next_char),
            DfaState::Identifier => self.identifier(next_char),
            DfaState::Zero => self.exit(),
            DfaState::Number => self.number(next_char),

            // [] {} ()
            DfaState::Bracket => self.exit(),

            // * - + / 
            DfaState::Plus => self.exit(),
            DfaState::Minus => self.exit(),
            DfaState::Multiply => self.exit(),
            DfaState::Div => self.exit(),

            // = == 
            DfaState::Assignment => self.assignment(next_char),
            DfaState::Equals => self.exit(),

            // < <= > >=  
            DfaState::LessThan => self.less(next_char),
            DfaState::LessThanEquals => self.exit(),
            DfaState::GreaterThan => self.greater(next_char),
            DfaState::GreaterThanEquals => self.exit(),

            // ! != 
            DfaState::Not => self.not(next_char),
            DfaState::NotEquals => self.exit(),

            // & && | || 
            DfaState::BitwiseAnd => self.bitwise_and(next_char),
            DfaState::BitwiseOr => self.bitwise_or(next_char),
            DfaState::LogicalAnd => self.exit(),
            DfaState::LogicalOr => self.exit(),

            // . , : ; 
            DfaState::Colon => self.exit(),
            DfaState::Semicolon => self.exit(), 
            DfaState::Comma => self.exit(), 
            DfaState::Dot => self.exit(),

            DfaState::Error => todo!(),
            DfaState::Exit => self.exit(),    
        }
    }

    fn start(self, next_char: char) -> DfaState {
        match next_char {
            'A'..='Z'|'a'..='z' => {
                return DfaState::Identifier;
            },
            '\t'|' '|'\n' => {
                return DfaState::Start;
            },
            '0' => {
                return DfaState::Zero;
            },
            '1'..='9' => {
                return DfaState::Number;
            },
            '{'|'}'|'['|']'|'('|')' => {
                return DfaState::Bracket;
            },
            '-' => {
                return DfaState::Minus
            },
            '*' => {
                return DfaState::Multiply
            },
            '/' => {
                return DfaState::Div
            }
            '+' => {
                return DfaState::Plus
            }
            '=' => {
                return DfaState::Assignment
            }
            '<' => {
                return DfaState::LessThan
            },
            '>' => {
                return DfaState::GreaterThan
            }
            '!' => {
                return DfaState::Not
            }
            '&' => {
                return DfaState::BitwiseAnd
            },
            '|' => {    
                return DfaState::BitwiseOr
            },
            '.' => {
                return DfaState::Dot
            },
            ',' => {
                return DfaState::Comma
            }, 
            ':' => {
                return DfaState::Colon
            },
            ';' => {
                return DfaState::Semicolon
            },
            _ => {
                return DfaState::Exit;
            }
        }  
    }

    fn identifier(self, next_char: char) -> DfaState {
        match next_char {
            'A'..='Z'|'a'..='z'|'_'|'0'..='9' => {
                return DfaState::Identifier;
            },
            _ => {
                return DfaState::Exit;
            }
        }
    }

    fn number(self, next_char: char) -> DfaState {
        match next_char {
            '0'..='9' => {
                return DfaState::Number;
            },
            _ => {
                return DfaState::Exit;
            }
        }
    }

    // Assignment = 
    pub fn assignment(self, next_char: char) -> DfaState {
        match next_char {
            '=' => {
                return DfaState::Equals;
            }, 
            _ => {
                DfaState::Exit
            }
        }
    }
    
    // <
    pub fn less(self, next_char: char) -> DfaState {
        match next_char {
            '=' => {
                return DfaState::LessThanEquals
            }
            _ => {
                return DfaState::Exit
            }
        }
    }

    // > 
    pub fn greater(self, next_char: char) -> DfaState {
        match next_char {
            '=' => {
                return DfaState::GreaterThanEquals
            }
            _ => {
                return DfaState::Exit
            }
        }
    }

    // !
    pub fn not(self, next_char: char) -> DfaState {
        match next_char {
            '=' => {
                return DfaState::NotEquals
            }
            _ => {
                return DfaState::Exit
            }
        }
    }

    // &
    pub fn bitwise_and(self, next_char: char) -> DfaState {
        match next_char {
            '&' => {
                return DfaState::LogicalAnd
            }
            _ => {
                return DfaState::Exit
            }
        }
    }

    //
    pub fn bitwise_or(self, next_char: char) -> DfaState {
        match next_char {
            '|' => {
                return DfaState::LogicalOr
            }
            _ => {
                return DfaState::Exit
            }
        }
    }

    fn exit(self) -> DfaState {
        return DfaState::Exit
    }
}
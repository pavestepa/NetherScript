use crate::lexer::Token;

pub fn precedence(token: &Token) -> u8 {
    match token {
        // member
        Token::Dot => 80,
        Token::LeftParen => 90,
        Token::LeftBracket => 90,

        // unary: led-handled only in nud
        Token::Not => 70,
        Token::BitNot => 70,

        // multiplicative
        Token::Star => 60,
        Token::Slash => 60,
        Token::Percent => 60,

        // additive
        Token::Plus => 50,
        Token::Minus => 50,

        // shifts
        Token::ShiftLeft => 45,
        Token::ShiftRight => 45,

        // comparison
        Token::Greater => 42,
        Token::GreaterEqual => 42,
        Token::Less => 42,
        Token::LessEqual => 42,

        // equality
        Token::Equals => 41,
        Token::NotEquals => 41,

        // bitwise
        Token::BitAnd => 35,
        Token::BitXor => 34,
        Token::BitOr => 33,

        // logical
        Token::And => 30,
        Token::Or => 29,

        // conditional
        Token::Question => 25,

        // assignment operators
        Token::Assign => 10,
        Token::PlusAssign => 10,
        Token::MinusAssign => 10,
        Token::StarAssign => 10,
        Token::SlashAssign => 10,
        Token::PercentAssign => 10,

        _ => 0,
    }
}

use crate::position::Position;

/// The kind of a Token
#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Kind {
    Keyword,
    Type,
    Ident,
    Comment,   // 
    Attribute, // (* attribute *)
    Str,
    Number,
    // Operators
    OpPlus, OpMinus, OpBang, OpTilde, OpAnd, OpNand, OpOr, OpNor, OpXor, OpXnor,
    OpStar, OpDiv, OpMod, OpEq, OpEq2, OpEq3, OpEq2Que, OpDiff, OpDiff2, OpDiffQue, 
    OpLogicAnd, OpLogicOr, OpPow, OpLT, OpLTE, OpGT, OpGTE, OpSL, OpSR, OpSSL, OpSSR, 
    OpImplication, OpEquivalence,   
    OpCompAss, // += -= *= /= %= &= |= ^= <<= >>= <<<= >>>=
    // Parenthesis
    ParenLeft, ParenRight, CurlyLeft, CurlyRight, SquareLeft, SquareRight,
    // Special character
    Comma, Que, Colon, Scope, SemiColon, Tick, At, Hash, Hash2, Dot
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: Kind,
    pub value: String,
    pub pos: Position,
}

#[allow(dead_code)]
impl Token {
    pub fn new(k : Kind, v: String, p : Position) -> Token {
        Token {kind:k, value: v, pos: p}
    }
}
use crate::position::Position;

use std::fmt;

// #[allow(dead_code)]
/// The kind of a Token
#[derive(PartialEq, Clone, Debug)]
pub enum Kind {
    Keyword,    // Reserved SystemVerilog keyword
    BaseType,   // One of the default type
    SystemTask, // $...
    Casting,    // type'
    Macro,      // `my_macro
    Ident,      // Any valid identifier
    Comment,    // // or /* */
    Attribute,  // (* attribute *)
    Str,        // " "
    Integer,    // Any integer, including base
    Real,       // Any real, including base
    // Operators
    OpPlus,     //  +
    OpMinus,    //  -
    OpIncrDecr, //  ++ or --
    OpBang,     //  !
    OpTilde,    //  ~
    OpAnd,      //  &
    OpNand,     //  ~&
    OpOr,       //  |
    OpNor,      //  ~|
    OpXor,      //  ^
    OpXnor,     //  ~^ ^~
    OpStar,     //  *
    OpDiv,      //  /
    OpMod,      //  %
    OpEq,       //  =
    OpEq2,      //  ==
    OpEq3,      //  ===
    OpEq2Que,   //  ==?
    OpDiff,     //  !=
    OpDiff2,    //  !==
    OpDiffQue,  //  !=?
    OpTimingAnd,//  &&&
    OpLogicAnd, //  &&
    OpLogicOr,  //  ||
    OpPow,      //  **
    OpLT,       //  <
    OpLTE,      //  <=
    OpGT,       //  >
    OpGTE,      //  >=
    OpSL,       //  <<
    OpSR,       //  >>
    OpSShift,   //  <<<  or >>>
    OpImpl,     // ->
    OpSeqRel,   // |-> #-# #=#
    OpFatArrL,  // =>
    OpStarLT,   // *>
    OpEquiv,    // <->
    OpCompAss,  // += -= *= /= %= &= |= ^= <<= >>= <<<= >>>=
    OpDist,     // := or :/
    OpRange,    // -: or +:
    // Parenthesis / Curly braces, Square braket
    ParenLeft, ParenRight, CurlyLeft, CurlyRight, SquareLeft, SquareRight, TickCurly,
    // Other Special character
    Comma, Que, Colon, Scope, SemiColon,
    At, At2, Hash, Hash2, Dot, DotStar,
    Dollar,
    None
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: Kind,
    pub value: String,
    pub pos: Position,
}

impl Token {
    pub fn new(k : Kind, v: String, p : Position) -> Token {
        Token {kind:k, value: v, pos: p}
    }
}

#[derive(Debug)]
pub struct TokenError {
    pub kind: Kind,
    pub pos: Position,
    pub txt: String,
}

impl TokenError {
    pub fn new(k : Kind, p : Position, t : String) -> TokenError {
        TokenError {kind:k, pos: p, txt: t}
    }
}

//-----------------------------------------------------------------------------
// Display traits

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Kind::Keyword        => write!(f, "Keyword    ") ,
            Kind::BaseType       => write!(f, "BaseType   ") ,
            Kind::Ident          => write!(f, "Ident      ") ,
            Kind::Casting        => write!(f, "Casting    ") ,
            Kind::Macro          => write!(f, "Macro      ") ,
            Kind::Comment        => write!(f, "Comment    ") ,
            Kind::Attribute      => write!(f, "Attribute  ") ,
            Kind::SystemTask     => write!(f, "SystemTask ") ,
            Kind::Str            => write!(f, "Str        ") ,
            Kind::Integer        => write!(f, "Integer    ") ,
            Kind::Real           => write!(f, "Real       ") ,
            Kind::OpPlus         => write!(f, "OpPlus     ") ,
            Kind::OpMinus        => write!(f, "OpMinus    ") ,
            Kind::OpIncrDecr     => write!(f, "OpIncrDecr ") ,
            Kind::OpBang         => write!(f, "OpBang     ") ,
            Kind::OpTilde        => write!(f, "OpTilde    ") ,
            Kind::OpAnd          => write!(f, "OpAnd      ") ,
            Kind::OpNand         => write!(f, "OpNand     ") ,
            Kind::OpOr           => write!(f, "OpOr       ") ,
            Kind::OpNor          => write!(f, "OpNor      ") ,
            Kind::OpXor          => write!(f, "OpXor      ") ,
            Kind::OpXnor         => write!(f, "OpXnor     ") ,
            Kind::OpStar         => write!(f, "OpStar     ") ,
            Kind::OpDiv          => write!(f, "OpDiv      ") ,
            Kind::OpMod          => write!(f, "OpMod      ") ,
            Kind::OpEq           => write!(f, "OpEq       ") ,
            Kind::OpEq2          => write!(f, "OpEq2      ") ,
            Kind::OpEq3          => write!(f, "OpEq3      ") ,
            Kind::OpEq2Que       => write!(f, "OpEq2Que   ") ,
            Kind::OpDiff         => write!(f, "OpDiff     ") ,
            Kind::OpDiff2        => write!(f, "OpDiff2    ") ,
            Kind::OpDiffQue      => write!(f, "OpDiffQue  ") ,
            Kind::OpLogicAnd     => write!(f, "OpLogicAnd ") ,
            Kind::OpTimingAnd    => write!(f, "OpTimingAnd") ,
            Kind::OpLogicOr      => write!(f, "OpLogicOr  ") ,
            Kind::OpPow          => write!(f, "OpPow      ") ,
            Kind::OpLT           => write!(f, "OpLT       ") ,
            Kind::OpLTE          => write!(f, "OpLTE      ") ,
            Kind::OpGT           => write!(f, "OpGT       ") ,
            Kind::OpGTE          => write!(f, "OpGTE      ") ,
            Kind::OpSL           => write!(f, "OpSL       ") ,
            Kind::OpSR           => write!(f, "OpSR       ") ,
            Kind::OpSShift       => write!(f, "OpSShift   ") ,
            Kind::OpImpl         => write!(f, "OpImpl     ") ,
            Kind::OpSeqRel       => write!(f, "OpSeqRel   ") ,
            Kind::OpFatArrL      => write!(f, "OpFatArrL  ") ,
            Kind::OpStarLT       => write!(f, "OpStarLT   ") ,
            Kind::OpEquiv        => write!(f, "OpEquiv    ") ,
            Kind::OpCompAss      => write!(f, "OpCompAss  ") ,
            Kind::OpDist         => write!(f, "OpDist     ") ,
            Kind::ParenLeft      => write!(f, "ParenLeft  ") ,
            Kind::ParenRight     => write!(f, "ParenRight ") ,
            Kind::CurlyLeft      => write!(f, "CurlyLeft  ") ,
            Kind::CurlyRight     => write!(f, "CurlyRight ") ,
            Kind::SquareLeft     => write!(f, "SquareLeft ") ,
            Kind::SquareRight    => write!(f, "SquareRight") ,
            Kind::TickCurly      => write!(f, "TickCurly  ") ,
            Kind::OpRange        => write!(f, "OpRange    ") ,
            Kind::Comma          => write!(f, "Comma      ") ,
            Kind::Que            => write!(f, "Que        ") ,
            Kind::Colon          => write!(f, "Colon      ") ,
            Kind::Scope          => write!(f, "Scope      ") ,
            Kind::SemiColon      => write!(f, "SemiColon  ") ,
            Kind::At             => write!(f, "At         ") ,
            Kind::At2            => write!(f, "At2        ") ,
            Kind::Hash           => write!(f, "Hash       ") ,
            Kind::Hash2          => write!(f, "Hash2      ") ,
            Kind::Dot            => write!(f, "Dot        ") ,
            Kind::DotStar        => write!(f, "DotStar    ") ,
            Kind::Dollar         => write!(f, "Dollar     ") ,
            Kind::None           => write!(f, "None       ") ,
        }
    }
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:8} = {}",self.kind,self.pos, self.value)
    }
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid token {:?} at line {} : \"{}\"",self.kind,self.pos, self.txt)
    }
}


//-----------------------------------------------------------------------------
// List of SV keywords / Base type

/// SV2018 Keywords
pub const KEYWORDS: [&str;234] = [
    "accept_on", "alias", "always", "always_comb", "always_ff", "always_latch", "and", "assert", "assign", "assume", "automatic",
    "before", "begin", "bind", "bins", "binsof", "break", "buf", "bufif0", "bufif1",
    "case", "casex", "casez", "cell", "checker", "class", "clocking", "cmos", "config", "const", "constraint", "context", "continue",
    "cover", "covergroup", "coverpoint", "cross",
    "deassign", "default", "defparam", "design", "disable", "dist", "do",
    "edge", "else", "end", "endcase", "endchecker", "endclass", "endclocking", "endconfig", "endfunction", "endgenerate", "endgroup",
    "endinterface", "endmodule", "endpackage", "endprimitive", "endprogram", "endproperty", "endspecify", "endsequence", "endtable",
    "endtask", "enum", "event", "eventually", "expect", "export", "extends", "extern",
    "final", "first_match", "for", "force", "foreach", "forever", "fork", "forkjoin", "function",
    "generate", "genvar", "global",
    "highz0", "highz1",
    "if", "iff", "ifnone", "ignore_bins", "illegal_bins", "implements", "implies", "import", "incdir", "include", "initial", "inout",
    "input", "inside", "instance", "interconnect", "interface", "intersect",
    "join", "join_any", "join_none",
    "large", "let", "liblist", "library", "local", "localparam",
    "macromodule", "matches", "modport", "module",
    "nand", "negedge", "nettype", "new", "nexttime", "nmos", "nor", "noshowcancelled", "not", "notif0", "notif1", "null",
    "or", "output",
    "package", "packed", "parameter", "pmos", "posedge", "primitive", "priority", "program", "property", "protected", "pull0", "pull1",
    "pulldown", "pullup", "pulsestyle_ondetect", "pulsestyle_onevent", "pure",
    "rand", "randc", "randcase", "randsequence", "rcmos", "ref", "reg", "reject_on", "release", "repeat", "restrict", "return",
    "rnmos", "rpmos", "rtran", "rtranif0", "rtranif1",
    "s_always", "s_eventually", "s_nexttime", "s_until", "s_until_with", "scalared", "sequence", "showcancelled", "signed",
    "small", "soft", "solve", "specify", "specparam", "static", "string", "strong", "strong0", "strong1", "struct", "super",
    "supply0", "supply1", "sync_accept_on", "sync_reject_on",
    "table", "tagged", "task", "this", "throughout", "timeprecision", "timeunit", "tran", "tranif0", "tranif1",
    "tri", "tri0", "tri1", "triand", "trior", "trireg", "type", "typedef",
    "union", "unique", "unique0", "unsigned", "until", "until_with", "untyped", "use", "uwire",
    "var", "vectored", "virtual",
    "wait", "wait_order", "wand", "weak", "weak0", "weak1", "while", "wildcard", "wire", "with", "within", "wor", "xnor", "xor ",
];

/// SV2018 base types
pub const BASETYPES: [&str;13] = [
    "bit",
    "byte",
    "chandle",
    "int",
    "integer",
    "logic",
    "longint",
    "real",
    "realtime",
    "shortint",
    "shortreal",
    "time",
    "void",
];


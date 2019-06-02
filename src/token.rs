use crate::position::Position;

use std::fmt;

/// The kind of a Token
#[derive(PartialEq, Clone, Debug)]
pub enum TokenKind {
    // Keywords
    KwModule, KwImport, KwParam, KwLParam, KwAssign, KwStatic, KwAutomatic,
    KwInput , KwOutput, KwInout , KwRef, KwVar,
    KwNetType, KwSupply, KwSigning,
    Keyword,    // Reserved SystemVerilog keyword
    // Base Type
    TypeIntAtom, TypeIntVector, TypeReal,
    TypeString, TypeCHandle, TypeVoid, TypeEvent,
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
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub pos: Position,
}

impl Token {
    pub fn new(k : TokenKind, v: String, p : Position) -> Token {
        Token {kind:k, value: v, pos: p}
    }
}

//-----------------------------------------------------------------------------
// Display traits

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TokenKind::Keyword           => write!(f, "Keyword        ") ,
            TokenKind::KwModule          => write!(f, "Kw-Module      ") ,
            TokenKind::KwParam           => write!(f, "Kw-Param       ") ,
            TokenKind::KwLParam          => write!(f, "Kw-LocalParam  ") ,
            TokenKind::KwImport          => write!(f, "Kw-Import      ") ,
            TokenKind::KwAssign          => write!(f, "Kw-Assign      ") ,
            TokenKind::KwStatic          => write!(f, "Kw-Lifetime    ") ,
            TokenKind::KwAutomatic       => write!(f, "Kw-Lifetime    ") ,
            TokenKind::KwInput           => write!(f, "Kw-Input       ") ,
            TokenKind::KwOutput          => write!(f, "Kw-Output      ") ,
            TokenKind::KwInout           => write!(f, "Kw-Inout       ") ,
            TokenKind::KwRef             => write!(f, "Kw-Ref         ") ,
            TokenKind::KwVar             => write!(f, "Kw-Var         ") ,
            TokenKind::KwNetType         => write!(f, "Kw-NetType     ") ,
            TokenKind::KwSupply          => write!(f, "Kw-Supply      ") ,
            TokenKind::KwSigning         => write!(f, "Kw-Signing     ") ,
            TokenKind::TypeIntAtom       => write!(f, "Type-IntAtom   ") ,
            TokenKind::TypeIntVector     => write!(f, "Type-IntVector ") ,
            TokenKind::TypeReal          => write!(f, "Type-Real      ") ,
            TokenKind::TypeString        => write!(f, "Type-String    ") ,
            TokenKind::TypeCHandle       => write!(f, "Type-C Handle  ") ,
            TokenKind::TypeVoid          => write!(f, "Type-Void      ") ,
            TokenKind::TypeEvent         => write!(f, "Type-Event     ") ,
            TokenKind::Ident             => write!(f, "Ident          ") ,
            TokenKind::Casting           => write!(f, "Casting        ") ,
            TokenKind::Macro             => write!(f, "Macro          ") ,
            TokenKind::Comment           => write!(f, "Comment        ") ,
            TokenKind::Attribute         => write!(f, "Attribute      ") ,
            TokenKind::SystemTask        => write!(f, "SystemTask     ") ,
            TokenKind::Str               => write!(f, "Str            ") ,
            TokenKind::Integer           => write!(f, "Integer        ") ,
            TokenKind::Real              => write!(f, "Real           ") ,
            TokenKind::OpPlus            => write!(f, "OpPlus         ") ,
            TokenKind::OpMinus           => write!(f, "OpMinus        ") ,
            TokenKind::OpIncrDecr        => write!(f, "OpIncrDecr     ") ,
            TokenKind::OpBang            => write!(f, "OpBang         ") ,
            TokenKind::OpTilde           => write!(f, "OpTilde        ") ,
            TokenKind::OpAnd             => write!(f, "OpAnd          ") ,
            TokenKind::OpNand            => write!(f, "OpNand         ") ,
            TokenKind::OpOr              => write!(f, "OpOr           ") ,
            TokenKind::OpNor             => write!(f, "OpNor          ") ,
            TokenKind::OpXor             => write!(f, "OpXor          ") ,
            TokenKind::OpXnor            => write!(f, "OpXnor         ") ,
            TokenKind::OpStar            => write!(f, "OpStar         ") ,
            TokenKind::OpDiv             => write!(f, "OpDiv          ") ,
            TokenKind::OpMod             => write!(f, "OpMod          ") ,
            TokenKind::OpEq              => write!(f, "OpEq           ") ,
            TokenKind::OpEq2             => write!(f, "OpEq2          ") ,
            TokenKind::OpEq3             => write!(f, "OpEq3          ") ,
            TokenKind::OpEq2Que          => write!(f, "OpEq2Que       ") ,
            TokenKind::OpDiff            => write!(f, "OpDiff         ") ,
            TokenKind::OpDiff2           => write!(f, "OpDiff2        ") ,
            TokenKind::OpDiffQue         => write!(f, "OpDiffQue      ") ,
            TokenKind::OpLogicAnd        => write!(f, "OpLogicAnd     ") ,
            TokenKind::OpTimingAnd       => write!(f, "OpTimingAnd    ") ,
            TokenKind::OpLogicOr         => write!(f, "OpLogicOr      ") ,
            TokenKind::OpPow             => write!(f, "OpPow          ") ,
            TokenKind::OpLT              => write!(f, "OpLT           ") ,
            TokenKind::OpLTE             => write!(f, "OpLTE          ") ,
            TokenKind::OpGT              => write!(f, "OpGT           ") ,
            TokenKind::OpGTE             => write!(f, "OpGTE          ") ,
            TokenKind::OpSL              => write!(f, "OpSL           ") ,
            TokenKind::OpSR              => write!(f, "OpSR           ") ,
            TokenKind::OpSShift          => write!(f, "OpSShift       ") ,
            TokenKind::OpImpl            => write!(f, "OpImpl         ") ,
            TokenKind::OpSeqRel          => write!(f, "OpSeqRel       ") ,
            TokenKind::OpFatArrL         => write!(f, "OpFatArrL      ") ,
            TokenKind::OpStarLT          => write!(f, "OpStarLT       ") ,
            TokenKind::OpEquiv           => write!(f, "OpEquiv        ") ,
            TokenKind::OpCompAss         => write!(f, "OpCompAss      ") ,
            TokenKind::OpDist            => write!(f, "OpDist         ") ,
            TokenKind::ParenLeft         => write!(f, "ParenLeft      ") ,
            TokenKind::ParenRight        => write!(f, "ParenRight     ") ,
            TokenKind::CurlyLeft         => write!(f, "CurlyLeft      ") ,
            TokenKind::CurlyRight        => write!(f, "CurlyRight     ") ,
            TokenKind::SquareLeft        => write!(f, "SquareLeft     ") ,
            TokenKind::SquareRight       => write!(f, "SquareRight    ") ,
            TokenKind::TickCurly         => write!(f, "TickCurly      ") ,
            TokenKind::OpRange           => write!(f, "OpRange        ") ,
            TokenKind::Comma             => write!(f, "Comma          ") ,
            TokenKind::Que               => write!(f, "Que            ") ,
            TokenKind::Colon             => write!(f, "Colon          ") ,
            TokenKind::Scope             => write!(f, "Scope          ") ,
            TokenKind::SemiColon         => write!(f, "SemiColon      ") ,
            TokenKind::At                => write!(f, "At             ") ,
            TokenKind::At2               => write!(f, "At2            ") ,
            TokenKind::Hash              => write!(f, "Hash           ") ,
            TokenKind::Hash2             => write!(f, "Hash2          ") ,
            TokenKind::Dot               => write!(f, "Dot            ") ,
            TokenKind::DotStar           => write!(f, "DotStar        ") ,
            TokenKind::Dollar            => write!(f, "Dollar         ") ,
        }
    }
}


impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}] {:8} = {}",self.kind,self.pos, self.value)
    }
}


//-----------------------------------------------------------------------------
// Helper function

pub fn keyword_from_str(w: &str) -> Option<TokenKind> {
    match w {
        "accept_on"           => Some(TokenKind::Keyword),
        "alias"               => Some(TokenKind::Keyword),
        "always"              => Some(TokenKind::Keyword),
        "always_comb"         => Some(TokenKind::Keyword),
        "always_ff"           => Some(TokenKind::Keyword),
        "always_latch"        => Some(TokenKind::Keyword),
        "and"                 => Some(TokenKind::Keyword),
        "assert"              => Some(TokenKind::Keyword),
        "assign"              => Some(TokenKind::KwAssign),
        "assume"              => Some(TokenKind::Keyword),
        "automatic"           => Some(TokenKind::KwAutomatic),
        "before"              => Some(TokenKind::Keyword),
        "begin"               => Some(TokenKind::Keyword),
        "bind"                => Some(TokenKind::Keyword),
        "bins"                => Some(TokenKind::Keyword),
        "binsof"              => Some(TokenKind::Keyword),
        "break"               => Some(TokenKind::Keyword),
        "buf"                 => Some(TokenKind::Keyword),
        "bufif0"              => Some(TokenKind::Keyword),
        "bufif1"              => Some(TokenKind::Keyword),
        "case"                => Some(TokenKind::Keyword),
        "casex"               => Some(TokenKind::Keyword),
        "casez"               => Some(TokenKind::Keyword),
        "cell"                => Some(TokenKind::Keyword),
        "checker"             => Some(TokenKind::Keyword),
        "class"               => Some(TokenKind::Keyword),
        "clocking"            => Some(TokenKind::Keyword),
        "cmos"                => Some(TokenKind::Keyword),
        "config"              => Some(TokenKind::Keyword),
        "const"               => Some(TokenKind::Keyword),
        "constraint"          => Some(TokenKind::Keyword),
        "context"             => Some(TokenKind::Keyword),
        "continue"            => Some(TokenKind::Keyword),
        "cover"               => Some(TokenKind::Keyword),
        "covergroup"          => Some(TokenKind::Keyword),
        "coverpoint"          => Some(TokenKind::Keyword),
        "cross"               => Some(TokenKind::Keyword),
        "deassign"            => Some(TokenKind::Keyword),
        "default"             => Some(TokenKind::Keyword),
        "defparam"            => Some(TokenKind::Keyword),
        "design"              => Some(TokenKind::Keyword),
        "disable"             => Some(TokenKind::Keyword),
        "dist"                => Some(TokenKind::Keyword),
        "do"                  => Some(TokenKind::Keyword),
        "edge"                => Some(TokenKind::Keyword),
        "else"                => Some(TokenKind::Keyword),
        "end"                 => Some(TokenKind::Keyword),
        "endcase"             => Some(TokenKind::Keyword),
        "endchecker"          => Some(TokenKind::Keyword),
        "endclass"            => Some(TokenKind::Keyword),
        "endclocking"         => Some(TokenKind::Keyword),
        "endconfig"           => Some(TokenKind::Keyword),
        "endfunction"         => Some(TokenKind::Keyword),
        "endgenerate"         => Some(TokenKind::Keyword),
        "endgroup"            => Some(TokenKind::Keyword),
        "endinterface"        => Some(TokenKind::Keyword),
        "endmodule"           => Some(TokenKind::Keyword),
        "endpackage"          => Some(TokenKind::Keyword),
        "endprimitive"        => Some(TokenKind::Keyword),
        "endprogram"          => Some(TokenKind::Keyword),
        "endproperty"         => Some(TokenKind::Keyword),
        "endspecify"          => Some(TokenKind::Keyword),
        "endsequence"         => Some(TokenKind::Keyword),
        "endtable"            => Some(TokenKind::Keyword),
        "endtask"             => Some(TokenKind::Keyword),
        "enum"                => Some(TokenKind::Keyword),
        "eventually"          => Some(TokenKind::Keyword),
        "expect"              => Some(TokenKind::Keyword),
        "export"              => Some(TokenKind::Keyword),
        "extends"             => Some(TokenKind::Keyword),
        "extern"              => Some(TokenKind::Keyword),
        "final"               => Some(TokenKind::Keyword),
        "first_match"         => Some(TokenKind::Keyword),
        "for"                 => Some(TokenKind::Keyword),
        "force"               => Some(TokenKind::Keyword),
        "foreach"             => Some(TokenKind::Keyword),
        "forever"             => Some(TokenKind::Keyword),
        "fork"                => Some(TokenKind::Keyword),
        "forkjoin"            => Some(TokenKind::Keyword),
        "function"            => Some(TokenKind::Keyword),
        "generate"            => Some(TokenKind::Keyword),
        "genvar"              => Some(TokenKind::Keyword),
        "global"              => Some(TokenKind::Keyword),
        "highz0"              => Some(TokenKind::Keyword),
        "highz1"              => Some(TokenKind::Keyword),
        "if"                  => Some(TokenKind::Keyword),
        "iff"                 => Some(TokenKind::Keyword),
        "ifnone"              => Some(TokenKind::Keyword),
        "ignore_bins"         => Some(TokenKind::Keyword),
        "illegal_bins"        => Some(TokenKind::Keyword),
        "implements"          => Some(TokenKind::Keyword),
        "implies"             => Some(TokenKind::Keyword),
        "import"              => Some(TokenKind::KwImport),
        "incdir"              => Some(TokenKind::Keyword),
        "include"             => Some(TokenKind::Keyword),
        "initial"             => Some(TokenKind::Keyword),
        "inout"               => Some(TokenKind::KwInout),
        "input"               => Some(TokenKind::KwInput),
        "inside"              => Some(TokenKind::Keyword),
        "instance"            => Some(TokenKind::Keyword),
        "interconnect"        => Some(TokenKind::Keyword),
        "interface"           => Some(TokenKind::Keyword),
        "intersect"           => Some(TokenKind::Keyword),
        "join"                => Some(TokenKind::Keyword),
        "join_any"            => Some(TokenKind::Keyword),
        "join_none"           => Some(TokenKind::Keyword),
        "large"               => Some(TokenKind::Keyword),
        "let"                 => Some(TokenKind::Keyword),
        "liblist"             => Some(TokenKind::Keyword),
        "library"             => Some(TokenKind::Keyword),
        "local"               => Some(TokenKind::Keyword),
        "localparam"          => Some(TokenKind::KwLParam),
        "macromodule"         => Some(TokenKind::KwModule),
        "matches"             => Some(TokenKind::Keyword),
        "modport"             => Some(TokenKind::Keyword),
        "module"              => Some(TokenKind::KwModule),
        "nand"                => Some(TokenKind::Keyword),
        "negedge"             => Some(TokenKind::Keyword),
        "nettype"             => Some(TokenKind::Keyword),
        "new"                 => Some(TokenKind::Keyword),
        "nexttime"            => Some(TokenKind::Keyword),
        "nmos"                => Some(TokenKind::Keyword),
        "nor"                 => Some(TokenKind::Keyword),
        "noshowcancelled"     => Some(TokenKind::Keyword),
        "not"                 => Some(TokenKind::Keyword),
        "notif0"              => Some(TokenKind::Keyword),
        "notif1"              => Some(TokenKind::Keyword),
        "null"                => Some(TokenKind::Keyword),
        "or"                  => Some(TokenKind::Keyword),
        "output"              => Some(TokenKind::KwOutput),
        "package"             => Some(TokenKind::Keyword),
        "packed"              => Some(TokenKind::Keyword),
        "parameter"           => Some(TokenKind::KwParam),
        "pmos"                => Some(TokenKind::Keyword),
        "posedge"             => Some(TokenKind::Keyword),
        "primitive"           => Some(TokenKind::Keyword),
        "priority"            => Some(TokenKind::Keyword),
        "program"             => Some(TokenKind::Keyword),
        "property"            => Some(TokenKind::Keyword),
        "protected"           => Some(TokenKind::Keyword),
        "pull0"               => Some(TokenKind::Keyword),
        "pull1"               => Some(TokenKind::Keyword),
        "pulldown"            => Some(TokenKind::Keyword),
        "pullup"              => Some(TokenKind::Keyword),
        "pulsestyle_ondetect" => Some(TokenKind::Keyword),
        "pulsestyle_onevent"  => Some(TokenKind::Keyword),
        "pure"                => Some(TokenKind::Keyword),
        "rand"                => Some(TokenKind::Keyword),
        "randc"               => Some(TokenKind::Keyword),
        "randcase"            => Some(TokenKind::Keyword),
        "randsequence"        => Some(TokenKind::Keyword),
        "rcmos"               => Some(TokenKind::Keyword),
        "ref"                 => Some(TokenKind::KwRef  ),
        "reg"                 => Some(TokenKind::Keyword),
        "reject_on"           => Some(TokenKind::Keyword),
        "release"             => Some(TokenKind::Keyword),
        "repeat"              => Some(TokenKind::Keyword),
        "restrict"            => Some(TokenKind::Keyword),
        "return"              => Some(TokenKind::Keyword),
        "rnmos"               => Some(TokenKind::Keyword),
        "rpmos"               => Some(TokenKind::Keyword),
        "rtran"               => Some(TokenKind::Keyword),
        "rtranif0"            => Some(TokenKind::Keyword),
        "rtranif1"            => Some(TokenKind::Keyword),
        "s_always"            => Some(TokenKind::Keyword),
        "s_eventually"        => Some(TokenKind::Keyword),
        "s_nexttime"          => Some(TokenKind::Keyword),
        "s_until"             => Some(TokenKind::Keyword),
        "s_until_with"        => Some(TokenKind::Keyword),
        "scalared"            => Some(TokenKind::Keyword),
        "sequence"            => Some(TokenKind::Keyword),
        "showcancelled"       => Some(TokenKind::Keyword),
        "signed"              => Some(TokenKind::KwSigning),
        "small"               => Some(TokenKind::Keyword),
        "soft"                => Some(TokenKind::Keyword),
        "solve"               => Some(TokenKind::Keyword),
        "specify"             => Some(TokenKind::Keyword),
        "specparam"           => Some(TokenKind::Keyword),
        "static"              => Some(TokenKind::KwStatic),
        "strong"              => Some(TokenKind::Keyword),
        "strong0"             => Some(TokenKind::Keyword),
        "strong1"             => Some(TokenKind::Keyword),
        "struct"              => Some(TokenKind::Keyword),
        "super"               => Some(TokenKind::Keyword),
        "supply0"             => Some(TokenKind::KwSupply),
        "supply1"             => Some(TokenKind::KwSupply),
        "sync_accept_on"      => Some(TokenKind::Keyword),
        "sync_reject_on"      => Some(TokenKind::Keyword),
        "table"               => Some(TokenKind::Keyword),
        "tagged"              => Some(TokenKind::Keyword),
        "task"                => Some(TokenKind::Keyword),
        "this"                => Some(TokenKind::Keyword),
        "throughout"          => Some(TokenKind::Keyword),
        "timeprecision"       => Some(TokenKind::Keyword),
        "timeunit"            => Some(TokenKind::Keyword),
        "tran"                => Some(TokenKind::Keyword),
        "tranif0"             => Some(TokenKind::Keyword),
        "tranif1"             => Some(TokenKind::Keyword),
        "tri"                 => Some(TokenKind::KwNetType ),
        "tri0"                => Some(TokenKind::KwNetType ),
        "tri1"                => Some(TokenKind::KwNetType ),
        "triand"              => Some(TokenKind::KwNetType ),
        "trior"               => Some(TokenKind::KwNetType ),
        "trireg"              => Some(TokenKind::KwNetType ),
        "type"                => Some(TokenKind::Keyword),
        "typedef"             => Some(TokenKind::Keyword),
        "union"               => Some(TokenKind::Keyword),
        "unique"              => Some(TokenKind::Keyword),
        "unique0"             => Some(TokenKind::Keyword),
        "unsigned"            => Some(TokenKind::KwSigning),
        "until"               => Some(TokenKind::Keyword),
        "until_with"          => Some(TokenKind::Keyword),
        "untyped"             => Some(TokenKind::Keyword),
        "use"                 => Some(TokenKind::Keyword),
        "uwire"               => Some(TokenKind::KwNetType ),
        "var"                 => Some(TokenKind::KwVar),
        "vectored"            => Some(TokenKind::Keyword),
        "virtual"             => Some(TokenKind::Keyword),
        "wait"                => Some(TokenKind::Keyword),
        "wait_order"          => Some(TokenKind::Keyword),
        "wand"                => Some(TokenKind::KwNetType),
        "weak"                => Some(TokenKind::Keyword),
        "weak0"               => Some(TokenKind::Keyword),
        "weak1"               => Some(TokenKind::Keyword),
        "while"               => Some(TokenKind::Keyword),
        "wildcard"            => Some(TokenKind::Keyword),
        "wire"                => Some(TokenKind::KwNetType),
        "with"                => Some(TokenKind::Keyword),
        "within"              => Some(TokenKind::Keyword),
        "wor"                 => Some(TokenKind::KwNetType),
        "xnor"                => Some(TokenKind::Keyword),
        "xor"                 => Some(TokenKind::Keyword),
        "PATHPULSE$"          => Some(TokenKind::Keyword),
        _ => None
    }
}

//-----------------------------------------------------------------------------
// List of SV keywords / Base type
pub fn basetype_from_str(w: &str) -> Option<TokenKind> {
    match w {
        "byte"      |
        "shortint"  |
        "int"       |
        "longint"   |
        "integer"   |
        "time"      => Some(TokenKind::TypeIntAtom),
        "bit"       |
        "logic"     => Some(TokenKind::TypeIntVector),
        "real"      |
        "realtime"  |
        "shortreal" => Some(TokenKind::TypeReal),
        //
        "string"    => Some(TokenKind::TypeString),
        "chandle"   => Some(TokenKind::TypeCHandle),
        "event"     => Some(TokenKind::TypeEvent),
        "void"      => Some(TokenKind::TypeVoid),
        _ => None
    }
}

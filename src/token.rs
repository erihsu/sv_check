use crate::position::Position;

use std::fmt;

/// The kind of a Token
#[derive(PartialEq, Clone, Debug)]
pub enum TokenKind {
    // Keywords
    KwModule,KwEndModule, KwImport, KwParam, KwLParam, KwAssign, KwStatic, KwAutomatic,
    KwInput , KwOutput, KwInout , KwRef, KwVar, KwIntf,
    KwNetType, KwSupply, KwSigning, KwEnum, KwStruct, KwUnion, KwPacked, KwTypedef,
    KwAlways, KwAlwaysC, KwAlwaysF, KwAlwaysL, KwEdge, KwOr, KwIff,
    KwBegin, KwEnd, KwIf, KwElse, KwFor,
    KwCase, KwEndcase, KwDefault, KwMatch, KwInside, KwUnique, KwUnique0, KwPriority, KwTagged,
    KwPackage, KwEndPackage, KwGenerate, KwEndGenerate,
    KwReg, KwVector, KwDrive, KwCharge,
    Keyword,    // Reserved SystemVerilog keyword
    // Base Type
    TypeIntAtom, TypeIntVector, TypeReal, TypeGenvar,
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
    SensiAll,   // (*)
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
            TokenKind::Keyword           => write!(f, "Keyword        "),
            TokenKind::KwModule          => write!(f, "Kw:module      "),
            TokenKind::KwEndModule       => write!(f, "Kw:endmodule   "),
            TokenKind::KwPackage         => write!(f, "Kw:package     "),
            TokenKind::KwEndPackage      => write!(f, "Kw:endpackage  "),
            TokenKind::KwGenerate        => write!(f, "Kw:generate    "),
            TokenKind::KwEndGenerate     => write!(f, "Kw:endgenerate "),
            TokenKind::KwBegin           => write!(f, "Kw:begin       "),
            TokenKind::KwEnd             => write!(f, "Kw:end         "),
            TokenKind::KwIff             => write!(f, "Kw:iff         "),
            TokenKind::KwIf              => write!(f, "Kw:if          "),
            TokenKind::KwElse            => write!(f, "Kw:else        "),
            TokenKind::KwFor             => write!(f, "Kw:for        "),
            TokenKind::KwCase            => write!(f, "Kw:case        "),
            TokenKind::KwEndcase         => write!(f, "Kw:endcase     "),
            TokenKind::KwParam           => write!(f, "Kw:param       "),
            TokenKind::KwLParam          => write!(f, "Kw:localparam  "),
            TokenKind::KwImport          => write!(f, "Kw:import      "),
            TokenKind::KwAssign          => write!(f, "Kw:assign      "),
            TokenKind::KwStatic          => write!(f, "Kw:lifetime    "),
            TokenKind::KwAutomatic       => write!(f, "Kw:lifetime    "),
            TokenKind::KwInput           => write!(f, "Kw:input       "),
            TokenKind::KwOutput          => write!(f, "Kw:output      "),
            TokenKind::KwInout           => write!(f, "Kw:inout       "),
            TokenKind::KwRef             => write!(f, "Kw:ref         "),
            TokenKind::KwVar             => write!(f, "Kw:var         "),
            TokenKind::KwIntf            => write!(f, "Kw:interface   "),
            TokenKind::KwNetType         => write!(f, "Kw:nettype     "),
            TokenKind::KwSupply          => write!(f, "Kw:supply      "),
            TokenKind::KwSigning         => write!(f, "Kw:signing     "),
            TokenKind::KwEnum            => write!(f, "Kw:enum        "),
            TokenKind::KwStruct          => write!(f, "Kw:struct      "),
            TokenKind::KwUnion           => write!(f, "Kw:union       "),
            TokenKind::KwTypedef         => write!(f, "Kw:typedef     "),
            TokenKind::KwAlways          => write!(f, "Kw:always      "),
            TokenKind::KwAlwaysC         => write!(f, "Kw:always_comb "),
            TokenKind::KwAlwaysF         => write!(f, "Kw:always_ff   "),
            TokenKind::KwAlwaysL         => write!(f, "Kw:always_latch"),
            TokenKind::KwEdge            => write!(f, "Kw:edge        "),
            TokenKind::KwOr              => write!(f, "Kw:or          "),
            TokenKind::KwMatch           => write!(f, "Kw:matches     "),
            TokenKind::KwInside          => write!(f, "Kw:inside      "),
            TokenKind::KwDefault         => write!(f, "Kw:default     "),
            TokenKind::KwUnique          => write!(f, "Kw:unique      "),
            TokenKind::KwUnique0         => write!(f, "Kw:unique0     "),
            TokenKind::KwPriority        => write!(f, "Kw:priority    "),
            TokenKind::KwTagged          => write!(f, "Kw:tagged      "),
            TokenKind::KwPacked          => write!(f, "Kw:packed      "),
            TokenKind::KwReg             => write!(f, "Kw:reg         "),
            TokenKind::KwDrive           => write!(f, "Kw:drive       "),
            TokenKind::KwCharge          => write!(f, "Kw:charge      "),
            TokenKind::KwVector          => write!(f, "Kw:vector      "),
            TokenKind::TypeIntAtom       => write!(f, "Type:IntAtom   "),
            TokenKind::TypeIntVector     => write!(f, "Type:IntVector "),
            TokenKind::TypeReal          => write!(f, "Type:Real      "),
            TokenKind::TypeString        => write!(f, "Type:String    "),
            TokenKind::TypeCHandle       => write!(f, "Type:C Handle  "),
            TokenKind::TypeVoid          => write!(f, "Type:Void      "),
            TokenKind::TypeEvent         => write!(f, "Type:Event     "),
            TokenKind::TypeGenvar        => write!(f, "Type:genvar    "),
            TokenKind::Ident             => write!(f, "Ident          "),
            TokenKind::Casting           => write!(f, "Casting        "),
            TokenKind::Macro             => write!(f, "Macro          "),
            TokenKind::Comment           => write!(f, "Comment        "),
            TokenKind::Attribute         => write!(f, "Attribute      "),
            TokenKind::SystemTask        => write!(f, "SystemTask     "),
            TokenKind::Str               => write!(f, "Str            "),
            TokenKind::Integer           => write!(f, "Integer        "),
            TokenKind::Real              => write!(f, "Real           "),
            TokenKind::OpPlus            => write!(f, "OpPlus         "),
            TokenKind::OpMinus           => write!(f, "OpMinus        "),
            TokenKind::OpIncrDecr        => write!(f, "OpIncrDecr     "),
            TokenKind::OpBang            => write!(f, "OpBang         "),
            TokenKind::OpTilde           => write!(f, "OpTilde        "),
            TokenKind::OpAnd             => write!(f, "OpAnd          "),
            TokenKind::OpNand            => write!(f, "OpNand         "),
            TokenKind::OpOr              => write!(f, "OpOr           "),
            TokenKind::OpNor             => write!(f, "OpNor          "),
            TokenKind::OpXor             => write!(f, "OpXor          "),
            TokenKind::OpXnor            => write!(f, "OpXnor         "),
            TokenKind::OpStar            => write!(f, "OpStar         "),
            TokenKind::OpDiv             => write!(f, "OpDiv          "),
            TokenKind::OpMod             => write!(f, "OpMod          "),
            TokenKind::OpEq              => write!(f, "OpEq           "),
            TokenKind::OpEq2             => write!(f, "OpEq2          "),
            TokenKind::OpEq3             => write!(f, "OpEq3          "),
            TokenKind::OpEq2Que          => write!(f, "OpEq2Que       "),
            TokenKind::OpDiff            => write!(f, "OpDiff         "),
            TokenKind::OpDiff2           => write!(f, "OpDiff2        "),
            TokenKind::OpDiffQue         => write!(f, "OpDiffQue      "),
            TokenKind::OpLogicAnd        => write!(f, "OpLogicAnd     "),
            TokenKind::OpTimingAnd       => write!(f, "OpTimingAnd    "),
            TokenKind::OpLogicOr         => write!(f, "OpLogicOr      "),
            TokenKind::OpPow             => write!(f, "OpPow          "),
            TokenKind::OpLT              => write!(f, "OpLT           "),
            TokenKind::OpLTE             => write!(f, "OpLTE          "),
            TokenKind::OpGT              => write!(f, "OpGT           "),
            TokenKind::OpGTE             => write!(f, "OpGTE          "),
            TokenKind::OpSL              => write!(f, "OpSL           "),
            TokenKind::OpSR              => write!(f, "OpSR           "),
            TokenKind::OpSShift          => write!(f, "OpSShift       "),
            TokenKind::OpImpl            => write!(f, "OpImpl         "),
            TokenKind::OpSeqRel          => write!(f, "OpSeqRel       "),
            TokenKind::OpFatArrL         => write!(f, "OpFatArrL      "),
            TokenKind::OpStarLT          => write!(f, "OpStarLT       "),
            TokenKind::OpEquiv           => write!(f, "OpEquiv        "),
            TokenKind::OpCompAss         => write!(f, "OpCompAss      "),
            TokenKind::OpDist            => write!(f, "OpDist         "),
            TokenKind::ParenLeft         => write!(f, "ParenLeft      "),
            TokenKind::ParenRight        => write!(f, "ParenRight     "),
            TokenKind::CurlyLeft         => write!(f, "CurlyLeft      "),
            TokenKind::CurlyRight        => write!(f, "CurlyRight     "),
            TokenKind::SquareLeft        => write!(f, "SquareLeft     "),
            TokenKind::SquareRight       => write!(f, "SquareRight    "),
            TokenKind::TickCurly         => write!(f, "TickCurly      "),
            TokenKind::OpRange           => write!(f, "OpRange        "),
            TokenKind::SensiAll          => write!(f, "SensiAll       "),
            TokenKind::Comma             => write!(f, "Comma          "),
            TokenKind::Que               => write!(f, "Que            "),
            TokenKind::Colon             => write!(f, "Colon          "),
            TokenKind::Scope             => write!(f, "Scope          "),
            TokenKind::SemiColon         => write!(f, "SemiColon      "),
            TokenKind::At                => write!(f, "At             "),
            TokenKind::At2               => write!(f, "At2            "),
            TokenKind::Hash              => write!(f, "Hash           "),
            TokenKind::Hash2             => write!(f, "Hash2          "),
            TokenKind::Dot               => write!(f, "Dot            "),
            TokenKind::DotStar           => write!(f, "DotStar        "),
            TokenKind::Dollar            => write!(f, "Dollar         "),
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
        "accept_on"           => Some(TokenKind::Keyword    ),
        "alias"               => Some(TokenKind::Keyword    ),
        "always"              => Some(TokenKind::KwAlways   ),
        "always_comb"         => Some(TokenKind::KwAlwaysC  ),
        "always_ff"           => Some(TokenKind::KwAlwaysF  ),
        "always_latch"        => Some(TokenKind::KwAlwaysL  ),
        "and"                 => Some(TokenKind::Keyword    ),
        "assert"              => Some(TokenKind::Keyword    ),
        "assign"              => Some(TokenKind::KwAssign   ),
        "assume"              => Some(TokenKind::Keyword    ),
        "automatic"           => Some(TokenKind::KwAutomatic),
        "before"              => Some(TokenKind::Keyword    ),
        "begin"               => Some(TokenKind::KwBegin    ),
        "bind"                => Some(TokenKind::Keyword    ),
        "bins"                => Some(TokenKind::Keyword    ),
        "binsof"              => Some(TokenKind::Keyword    ),
        "break"               => Some(TokenKind::Keyword    ),
        "buf"                 => Some(TokenKind::Keyword    ),
        "bufif0"              => Some(TokenKind::Keyword    ),
        "bufif1"              => Some(TokenKind::Keyword    ),
        "case"                => Some(TokenKind::KwCase     ),
        "casex"               => Some(TokenKind::KwCase     ),
        "casez"               => Some(TokenKind::KwCase     ),
        "cell"                => Some(TokenKind::Keyword    ),
        "checker"             => Some(TokenKind::Keyword    ),
        "class"               => Some(TokenKind::Keyword    ),
        "clocking"            => Some(TokenKind::Keyword    ),
        "cmos"                => Some(TokenKind::Keyword    ),
        "config"              => Some(TokenKind::Keyword    ),
        "const"               => Some(TokenKind::Keyword    ),
        "constraint"          => Some(TokenKind::Keyword    ),
        "context"             => Some(TokenKind::Keyword    ),
        "continue"            => Some(TokenKind::Keyword    ),
        "cover"               => Some(TokenKind::Keyword    ),
        "covergroup"          => Some(TokenKind::Keyword    ),
        "coverpoint"          => Some(TokenKind::Keyword    ),
        "cross"               => Some(TokenKind::Keyword    ),
        "deassign"            => Some(TokenKind::Keyword    ),
        "default"             => Some(TokenKind::KwDefault  ),
        "defparam"            => Some(TokenKind::Keyword    ),
        "design"              => Some(TokenKind::Keyword    ),
        "disable"             => Some(TokenKind::Keyword    ),
        "dist"                => Some(TokenKind::Keyword    ),
        "do"                  => Some(TokenKind::Keyword    ),
        "edge"                => Some(TokenKind::KwEdge     ),
        "else"                => Some(TokenKind::KwElse     ),
        "end"                 => Some(TokenKind::KwEnd      ),
        "endcase"             => Some(TokenKind::KwEndcase  ),
        "endchecker"          => Some(TokenKind::Keyword    ),
        "endclass"            => Some(TokenKind::Keyword    ),
        "endclocking"         => Some(TokenKind::Keyword    ),
        "endconfig"           => Some(TokenKind::Keyword    ),
        "endfunction"         => Some(TokenKind::Keyword    ),
        "endgenerate"         => Some(TokenKind::KwEndGenerate),
        "endgroup"            => Some(TokenKind::Keyword    ),
        "endinterface"        => Some(TokenKind::Keyword    ),
        "endmodule"           => Some(TokenKind::KwEndModule),
        "endpackage"          => Some(TokenKind::KwEndPackage),
        "endprimitive"        => Some(TokenKind::Keyword    ),
        "endprogram"          => Some(TokenKind::Keyword    ),
        "endproperty"         => Some(TokenKind::Keyword    ),
        "endspecify"          => Some(TokenKind::Keyword    ),
        "endsequence"         => Some(TokenKind::Keyword    ),
        "endtable"            => Some(TokenKind::Keyword    ),
        "endtask"             => Some(TokenKind::Keyword    ),
        "enum"                => Some(TokenKind::KwEnum     ),
        "eventually"          => Some(TokenKind::Keyword    ),
        "expect"              => Some(TokenKind::Keyword    ),
        "export"              => Some(TokenKind::Keyword    ),
        "extends"             => Some(TokenKind::Keyword    ),
        "extern"              => Some(TokenKind::Keyword    ),
        "final"               => Some(TokenKind::Keyword    ),
        "first_match"         => Some(TokenKind::Keyword    ),
        "for"                 => Some(TokenKind::KwFor      ),
        "force"               => Some(TokenKind::Keyword    ),
        "foreach"             => Some(TokenKind::Keyword    ),
        "forever"             => Some(TokenKind::Keyword    ),
        "fork"                => Some(TokenKind::Keyword    ),
        "forkjoin"            => Some(TokenKind::Keyword    ),
        "function"            => Some(TokenKind::Keyword    ),
        "generate"            => Some(TokenKind::KwGenerate ),
        "global"              => Some(TokenKind::Keyword    ),
        "highz0"              => Some(TokenKind::KwDrive    ),
        "highz1"              => Some(TokenKind::KwDrive    ),
        "if"                  => Some(TokenKind::KwIf       ),
        "iff"                 => Some(TokenKind::KwIff      ),
        "ifnone"              => Some(TokenKind::Keyword    ),
        "ignore_bins"         => Some(TokenKind::Keyword    ),
        "illegal_bins"        => Some(TokenKind::Keyword    ),
        "implements"          => Some(TokenKind::Keyword    ),
        "implies"             => Some(TokenKind::Keyword    ),
        "import"              => Some(TokenKind::KwImport   ),
        "incdir"              => Some(TokenKind::Keyword    ),
        "include"             => Some(TokenKind::Keyword    ),
        "initial"             => Some(TokenKind::Keyword    ),
        "inout"               => Some(TokenKind::KwInout    ),
        "input"               => Some(TokenKind::KwInput    ),
        "inside"              => Some(TokenKind::KwInside   ),
        "instance"            => Some(TokenKind::Keyword    ),
        "interconnect"        => Some(TokenKind::Keyword    ),
        "interface"           => Some(TokenKind::KwIntf     ),
        "intersect"           => Some(TokenKind::Keyword    ),
        "join"                => Some(TokenKind::Keyword    ),
        "join_any"            => Some(TokenKind::Keyword    ),
        "join_none"           => Some(TokenKind::Keyword    ),
        "large"               => Some(TokenKind::KwCharge    ),
        "let"                 => Some(TokenKind::Keyword    ),
        "liblist"             => Some(TokenKind::Keyword    ),
        "library"             => Some(TokenKind::Keyword    ),
        "local"               => Some(TokenKind::Keyword    ),
        "localparam"          => Some(TokenKind::KwLParam   ),
        "macromodule"         => Some(TokenKind::KwModule   ),
        "matches"             => Some(TokenKind::KwMatch    ),
        "medium"              => Some(TokenKind::KwCharge   ),
        "modport"             => Some(TokenKind::Keyword    ),
        "module"              => Some(TokenKind::KwModule   ),
        "nand"                => Some(TokenKind::Keyword    ),
        "negedge"             => Some(TokenKind::KwEdge     ),
        "nettype"             => Some(TokenKind::Keyword    ),
        "new"                 => Some(TokenKind::Keyword    ),
        "nexttime"            => Some(TokenKind::Keyword    ),
        "nmos"                => Some(TokenKind::Keyword    ),
        "nor"                 => Some(TokenKind::Keyword    ),
        "noshowcancelled"     => Some(TokenKind::Keyword    ),
        "not"                 => Some(TokenKind::Keyword    ),
        "notif0"              => Some(TokenKind::Keyword    ),
        "notif1"              => Some(TokenKind::Keyword    ),
        "null"                => Some(TokenKind::Keyword    ),
        "or"                  => Some(TokenKind::KwOr       ),
        "output"              => Some(TokenKind::KwOutput   ),
        "package"             => Some(TokenKind::KwPackage  ),
        "packed"              => Some(TokenKind::KwPacked   ),
        "parameter"           => Some(TokenKind::KwParam    ),
        "pmos"                => Some(TokenKind::Keyword    ),
        "posedge"             => Some(TokenKind::KwEdge     ),
        "primitive"           => Some(TokenKind::Keyword    ),
        "priority"            => Some(TokenKind::KwPriority ),
        "program"             => Some(TokenKind::Keyword    ),
        "property"            => Some(TokenKind::Keyword    ),
        "protected"           => Some(TokenKind::Keyword    ),
        "pull0"               => Some(TokenKind::KwDrive    ),
        "pull1"               => Some(TokenKind::KwDrive    ),
        "pulldown"            => Some(TokenKind::Keyword    ),
        "pullup"              => Some(TokenKind::Keyword    ),
        "pulsestyle_ondetect" => Some(TokenKind::Keyword    ),
        "pulsestyle_onevent"  => Some(TokenKind::Keyword    ),
        "pure"                => Some(TokenKind::Keyword    ),
        "rand"                => Some(TokenKind::Keyword    ),
        "randc"               => Some(TokenKind::Keyword    ),
        "randcase"            => Some(TokenKind::Keyword    ),
        "randsequence"        => Some(TokenKind::Keyword    ),
        "rcmos"               => Some(TokenKind::Keyword    ),
        "ref"                 => Some(TokenKind::KwRef      ),
        "reg"                 => Some(TokenKind::KwReg      ),
        "reject_on"           => Some(TokenKind::Keyword    ),
        "release"             => Some(TokenKind::Keyword    ),
        "repeat"              => Some(TokenKind::Keyword    ),
        "restrict"            => Some(TokenKind::Keyword    ),
        "return"              => Some(TokenKind::Keyword    ),
        "rnmos"               => Some(TokenKind::Keyword    ),
        "rpmos"               => Some(TokenKind::Keyword    ),
        "rtran"               => Some(TokenKind::Keyword    ),
        "rtranif0"            => Some(TokenKind::Keyword    ),
        "rtranif1"            => Some(TokenKind::Keyword    ),
        "s_always"            => Some(TokenKind::Keyword    ),
        "s_eventually"        => Some(TokenKind::Keyword    ),
        "s_nexttime"          => Some(TokenKind::Keyword    ),
        "s_until"             => Some(TokenKind::Keyword    ),
        "s_until_with"        => Some(TokenKind::Keyword    ),
        "scalared"            => Some(TokenKind::KwVector   ),
        "sequence"            => Some(TokenKind::Keyword    ),
        "showcancelled"       => Some(TokenKind::Keyword    ),
        "signed"              => Some(TokenKind::KwSigning  ),
        "small"               => Some(TokenKind::KwCharge   ),
        "soft"                => Some(TokenKind::Keyword    ),
        "solve"               => Some(TokenKind::Keyword    ),
        "specify"             => Some(TokenKind::Keyword    ),
        "specparam"           => Some(TokenKind::Keyword    ),
        "static"              => Some(TokenKind::KwStatic   ),
        "strong"              => Some(TokenKind::Keyword    ),
        "strong0"             => Some(TokenKind::KwDrive    ),
        "strong1"             => Some(TokenKind::KwDrive    ),
        "struct"              => Some(TokenKind::KwStruct   ),
        "super"               => Some(TokenKind::Keyword    ),
        "supply0"             => Some(TokenKind::KwSupply   ),
        "supply1"             => Some(TokenKind::KwSupply   ),
        "sync_accept_on"      => Some(TokenKind::Keyword    ),
        "sync_reject_on"      => Some(TokenKind::Keyword    ),
        "table"               => Some(TokenKind::Keyword    ),
        "tagged"              => Some(TokenKind::KwTagged   ),
        "task"                => Some(TokenKind::Keyword    ),
        "this"                => Some(TokenKind::Keyword    ),
        "throughout"          => Some(TokenKind::Keyword    ),
        "timeprecision"       => Some(TokenKind::Keyword    ),
        "timeunit"            => Some(TokenKind::Keyword    ),
        "tran"                => Some(TokenKind::Keyword    ),
        "tranif0"             => Some(TokenKind::Keyword    ),
        "tranif1"             => Some(TokenKind::Keyword    ),
        "tri"                 => Some(TokenKind::KwNetType  ),
        "tri0"                => Some(TokenKind::KwNetType  ),
        "tri1"                => Some(TokenKind::KwNetType  ),
        "triand"              => Some(TokenKind::KwNetType  ),
        "trior"               => Some(TokenKind::KwNetType  ),
        "trireg"              => Some(TokenKind::KwNetType  ),
        "type"                => Some(TokenKind::Keyword    ),
        "typedef"             => Some(TokenKind::KwTypedef  ),
        "union"               => Some(TokenKind::KwUnion    ),
        "unique"              => Some(TokenKind::KwUnique   ),
        "unique0"             => Some(TokenKind::KwUnique0  ),
        "unsigned"            => Some(TokenKind::KwSigning  ),
        "until"               => Some(TokenKind::Keyword    ),
        "until_with"          => Some(TokenKind::Keyword    ),
        "untyped"             => Some(TokenKind::Keyword    ),
        "use"                 => Some(TokenKind::Keyword    ),
        "uwire"               => Some(TokenKind::KwNetType  ),
        "var"                 => Some(TokenKind::KwVar      ),
        "vectored"            => Some(TokenKind::KwVector   ),
        "virtual"             => Some(TokenKind::Keyword    ),
        "wait"                => Some(TokenKind::Keyword    ),
        "wait_order"          => Some(TokenKind::Keyword    ),
        "wand"                => Some(TokenKind::KwNetType  ),
        "weak"                => Some(TokenKind::Keyword    ),
        "weak0"               => Some(TokenKind::KwDrive    ),
        "weak1"               => Some(TokenKind::KwDrive    ),
        "while"               => Some(TokenKind::Keyword    ),
        "wildcard"            => Some(TokenKind::Keyword    ),
        "wire"                => Some(TokenKind::KwNetType  ),
        "with"                => Some(TokenKind::Keyword    ),
        "within"              => Some(TokenKind::Keyword    ),
        "wor"                 => Some(TokenKind::KwNetType  ),
        "xnor"                => Some(TokenKind::Keyword    ),
        "xor"                 => Some(TokenKind::Keyword    ),
        "PATHPULSE$"          => Some(TokenKind::Keyword    ),
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
        "genvar"    => Some(TokenKind::TypeGenvar ),
        "string"    => Some(TokenKind::TypeString),
        "chandle"   => Some(TokenKind::TypeCHandle),
        "event"     => Some(TokenKind::TypeEvent),
        "void"      => Some(TokenKind::TypeVoid),
        _ => None
    }
}

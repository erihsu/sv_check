// This file is part of sv_check and subject to the terms of MIT Licence
// Copyright (c) 2019, clams@mail.com

use crate::lex::position::Position;

use std::fmt;

/// The kind of a Token
#[derive(PartialEq, Clone, Debug)]
pub enum TokenKind {
    // Keywords
    KwModule,KwEndModule, KwImport, KwExport, KwParam, KwLParam, KwAssign, KwDefparam, KwStatic, KwAutomatic,
    KwInput , KwOutput, KwInout , KwRef, KwVar, KwTimeunit, KwTimeprec,
    KwNetType, KwSupply, KwSigning, KwEnum, KwStruct, KwUnion, KwPacked, KwTypedef, KwType,
    KwAlways, KwAlwaysC, KwAlwaysF, KwAlwaysL, KwInitial, KwEdge, KwOr, KwIff, KwBind,
    KwBegin, KwEnd, KwIf, KwElse, KwFor, KwForever, KwForeach, KwRepeat, KwWhile,KwDo,
    KwCase, KwEndcase, KwDefault, KwMatch, KwInside, KwUnique, KwUnique0, KwPriority, KwTagged,
    KwPackage, KwEndPackage, KwIntf, KwEndIntf, KwGenerate, KwEndGenerate,KwEndGroup,
    KwReg, KwVector, KwDrive, KwCharge,
    KwModport, KwClocking, KwEndClocking, KwGlobal,
    KwClass, KwEndClass, KwVirtual, KwPure, KwExtends, KwImplements, KwLocal, KwProtected, KwExtern, KwContext,
    KwFunction, KwEndFunction, KwTask, KwEndTask, KwRand, KwNew, KwConst, KwSuper, KwThis, KwNull,
    KwReturn, KwBreak, KwContinue, KwFork, KwJoin, KwDisable, KwWait, KwWaitOrder,
    KwAssert, KwCover,
    KwConstraint, KwWith, KwCovergroup,
    Keyword,    // Reserved SystemVerilog keyword
    // Base Type
    TypeIntAtom, TypeIntVector, TypeReal, TypeGenvar,
    TypeString, TypeCHandle, TypeVoid, TypeEvent,
    SystemTask, // $...
    Casting,    // type'
    Macro,      // `my_macro
    IdentInterpolated, // ``macro_ident
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
    Dollar, LineCont
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
            TokenKind::Keyword           => write!(f, "Keyword"        ),
            TokenKind::KwModule          => write!(f, "Kw:module"      ),
            TokenKind::KwEndModule       => write!(f, "Kw:endmodule"   ),
            TokenKind::KwPackage         => write!(f, "Kw:package"     ),
            TokenKind::KwEndPackage      => write!(f, "Kw:endpackage"  ),
            TokenKind::KwGenerate        => write!(f, "Kw:generate"    ),
            TokenKind::KwEndGenerate     => write!(f, "Kw:endgenerate" ),
            TokenKind::KwEndGroup        => write!(f, "Kw:endgroup"    ),
            TokenKind::KwBind            => write!(f, "Kw:bind"        ),
            TokenKind::KwBegin           => write!(f, "Kw:begin"       ),
            TokenKind::KwEnd             => write!(f, "Kw:end"         ),
            TokenKind::KwIff             => write!(f, "Kw:iff"         ),
            TokenKind::KwIf              => write!(f, "Kw:if"          ),
            TokenKind::KwElse            => write!(f, "Kw:else"        ),
            TokenKind::KwFor             => write!(f, "Kw:for"         ),
            TokenKind::KwForever         => write!(f, "Kw:forever"     ),
            TokenKind::KwForeach         => write!(f, "Kw:foreach"     ),
            TokenKind::KwRepeat          => write!(f, "Kw:repeat"      ),
            TokenKind::KwWhile           => write!(f, "Kw:while"       ),
            TokenKind::KwDo              => write!(f, "Kw:do"          ),
            TokenKind::KwCase            => write!(f, "Kw:case"        ),
            TokenKind::KwEndcase         => write!(f, "Kw:endcase"     ),
            TokenKind::KwParam           => write!(f, "Kw:param"       ),
            TokenKind::KwLParam          => write!(f, "Kw:localparam"  ),
            TokenKind::KwImport          => write!(f, "Kw:import"      ),
            TokenKind::KwExport          => write!(f, "Kw:export"      ),
            TokenKind::KwAssign          => write!(f, "Kw:assign"      ),
            TokenKind::KwDefparam        => write!(f, "Kw:defparam"    ),
            TokenKind::KwStatic          => write!(f, "Kw:lifetime"    ),
            TokenKind::KwAutomatic       => write!(f, "Kw:lifetime"    ),
            TokenKind::KwInput           => write!(f, "Kw:input"       ),
            TokenKind::KwOutput          => write!(f, "Kw:output"      ),
            TokenKind::KwInout           => write!(f, "Kw:inout"       ),
            TokenKind::KwRef             => write!(f, "Kw:ref"         ),
            TokenKind::KwVar             => write!(f, "Kw:var"         ),
            TokenKind::KwIntf            => write!(f, "Kw:interface"   ),
            TokenKind::KwEndIntf         => write!(f, "Kw:endinterface"),
            TokenKind::KwNetType         => write!(f, "Kw:nettype"     ),
            TokenKind::KwSupply          => write!(f, "Kw:supply"      ),
            TokenKind::KwSigning         => write!(f, "Kw:signing"     ),
            TokenKind::KwEnum            => write!(f, "Kw:enum"        ),
            TokenKind::KwStruct          => write!(f, "Kw:struct"      ),
            TokenKind::KwUnion           => write!(f, "Kw:union"       ),
            TokenKind::KwTypedef         => write!(f, "Kw:typedef"     ),
            TokenKind::KwType            => write!(f, "Kw:type"        ),
            TokenKind::KwAlways          => write!(f, "Kw:always"      ),
            TokenKind::KwAlwaysC         => write!(f, "Kw:always_comb" ),
            TokenKind::KwAlwaysF         => write!(f, "Kw:always_ff"   ),
            TokenKind::KwAlwaysL         => write!(f, "Kw:always_latch"),
            TokenKind::KwInitial         => write!(f, "Kw:initial"     ),
            TokenKind::KwEdge            => write!(f, "Kw:edge"        ),
            TokenKind::KwOr              => write!(f, "Kw:or"          ),
            TokenKind::KwMatch           => write!(f, "Kw:matches"     ),
            TokenKind::KwInside          => write!(f, "Kw:inside"      ),
            TokenKind::KwDefault         => write!(f, "Kw:default"     ),
            TokenKind::KwUnique          => write!(f, "Kw:unique"      ),
            TokenKind::KwUnique0         => write!(f, "Kw:unique0"     ),
            TokenKind::KwPriority        => write!(f, "Kw:priority"    ),
            TokenKind::KwTagged          => write!(f, "Kw:tagged"      ),
            TokenKind::KwPacked          => write!(f, "Kw:packed"      ),
            TokenKind::KwReg             => write!(f, "Kw:reg"         ),
            TokenKind::KwDrive           => write!(f, "Kw:drive"       ),
            TokenKind::KwCharge          => write!(f, "Kw:charge"      ),
            TokenKind::KwVector          => write!(f, "Kw:vector"      ),
            TokenKind::KwModport         => write!(f, "Kw:modport"     ),
            TokenKind::KwClocking        => write!(f, "Kw:clocking"    ),
            TokenKind::KwEndClocking     => write!(f, "Kw:endclocking" ),
            TokenKind::KwGlobal          => write!(f, "Kw:global"      ),
            TokenKind::KwTimeunit        => write!(f, "Kw:timeunit"    ),
            TokenKind::KwTimeprec        => write!(f, "Kw:timeprec"    ),
            TokenKind::KwClass           => write!(f, "Kw:class"       ),
            TokenKind::KwEndClass        => write!(f, "Kw:endclass"    ),
            TokenKind::KwVirtual         => write!(f, "Kw:virtual"     ),
            TokenKind::KwPure            => write!(f, "Kw:pure"        ),
            TokenKind::KwContext         => write!(f, "Kw:context"     ),
            TokenKind::KwExtends         => write!(f, "Kw:extends"     ),
            TokenKind::KwImplements      => write!(f, "Kw:implements"  ),
            TokenKind::KwLocal           => write!(f, "Kw:local"       ),
            TokenKind::KwProtected       => write!(f, "Kw:protected"   ),
            TokenKind::KwExtern          => write!(f, "Kw:extern"      ),
            TokenKind::KwFunction        => write!(f, "Kw:function"    ),
            TokenKind::KwTask            => write!(f, "Kw:task"        ),
            TokenKind::KwEndFunction     => write!(f, "Kw:endfunction" ),
            TokenKind::KwEndTask         => write!(f, "Kw:endtask"     ),
            TokenKind::KwRand            => write!(f, "Kw:rand"        ),
            TokenKind::KwNew             => write!(f, "Kw:new"         ),
            TokenKind::KwConst           => write!(f, "Kw:const"       ),
            TokenKind::KwSuper           => write!(f, "Kw:super"       ),
            TokenKind::KwThis            => write!(f, "Kw:this"        ),
            TokenKind::KwNull            => write!(f, "Kw:null"        ),
            TokenKind::KwReturn          => write!(f, "Kw:return"      ),
            TokenKind::KwBreak           => write!(f, "Kw:break"       ),
            TokenKind::KwContinue        => write!(f, "Kw:continue"    ),
            TokenKind::KwAssert          => write!(f, "Kw:assert"      ),
            TokenKind::KwCover           => write!(f, "Kw:cover"       ),
            TokenKind::KwFork            => write!(f, "Kw:fork"        ),
            TokenKind::KwDisable         => write!(f, "Kw:disable"     ),
            TokenKind::KwJoin            => write!(f, "Kw:join"        ),
            TokenKind::KwWait            => write!(f, "Kw:wait"        ),
            TokenKind::KwWaitOrder       => write!(f, "Kw:wait_order"  ),
            TokenKind::KwConstraint      => write!(f, "Kw:constraint"  ),
            TokenKind::KwWith            => write!(f, "Kw:with"        ),
            TokenKind::KwCovergroup      => write!(f, "Kw:covergroup"  ),
            TokenKind::TypeIntAtom       => write!(f, "Type:IntAtom"   ),
            TokenKind::TypeIntVector     => write!(f, "Type:IntVector" ),
            TokenKind::TypeReal          => write!(f, "Type:Real"      ),
            TokenKind::TypeString        => write!(f, "Type:String"    ),
            TokenKind::TypeCHandle       => write!(f, "Type:C Handle"  ),
            TokenKind::TypeVoid          => write!(f, "Type:Void"      ),
            TokenKind::TypeEvent         => write!(f, "Type:Event"     ),
            TokenKind::TypeGenvar        => write!(f, "Type:genvar"    ),
            TokenKind::Ident             => write!(f, "Ident"          ),
            TokenKind::IdentInterpolated => write!(f, "IdentInterpolated"),
            TokenKind::Casting           => write!(f, "Casting"        ),
            TokenKind::Macro             => write!(f, "Macro"          ),
            TokenKind::Comment           => write!(f, "Comment"        ),
            TokenKind::Attribute         => write!(f, "Attribute"      ),
            TokenKind::SystemTask        => write!(f, "SystemTask"     ),
            TokenKind::Str               => write!(f, "String"         ),
            TokenKind::Integer           => write!(f, "Integer"        ),
            TokenKind::Real              => write!(f, "Real"           ),
            TokenKind::OpPlus            => write!(f, "Op:Plus"        ),
            TokenKind::OpMinus           => write!(f, "Op:Minus"       ),
            TokenKind::OpIncrDecr        => write!(f, "Op:IncrDecr"    ),
            TokenKind::OpBang            => write!(f, "Op:Bang"        ),
            TokenKind::OpTilde           => write!(f, "Op:Tilde"       ),
            TokenKind::OpAnd             => write!(f, "Op:And"         ),
            TokenKind::OpNand            => write!(f, "Op:Nand"        ),
            TokenKind::OpOr              => write!(f, "Op:Or"          ),
            TokenKind::OpNor             => write!(f, "Op:Nor"         ),
            TokenKind::OpXor             => write!(f, "Op:Xor"         ),
            TokenKind::OpXnor            => write!(f, "Op:Xnor"        ),
            TokenKind::OpStar            => write!(f, "Op:Star"        ),
            TokenKind::OpDiv             => write!(f, "Op:Div"         ),
            TokenKind::OpMod             => write!(f, "Op:Mod"         ),
            TokenKind::OpEq              => write!(f, "Op:Eq"          ),
            TokenKind::OpEq2             => write!(f, "Op:Eq2"         ),
            TokenKind::OpEq3             => write!(f, "Op:Eq3"         ),
            TokenKind::OpEq2Que          => write!(f, "Op:Eq2Que"      ),
            TokenKind::OpDiff            => write!(f, "Op:Diff"        ),
            TokenKind::OpDiff2           => write!(f, "Op:Diff2"       ),
            TokenKind::OpDiffQue         => write!(f, "Op:DiffQue"     ),
            TokenKind::OpLogicAnd        => write!(f, "Op:LogicAnd"    ),
            TokenKind::OpTimingAnd       => write!(f, "Op:TimingAnd"   ),
            TokenKind::OpLogicOr         => write!(f, "Op:LogicOr"     ),
            TokenKind::OpPow             => write!(f, "Op:Pow"         ),
            TokenKind::OpLT              => write!(f, "Op:LT"          ),
            TokenKind::OpLTE             => write!(f, "Op:LTE"         ),
            TokenKind::OpGT              => write!(f, "Op:GT"          ),
            TokenKind::OpGTE             => write!(f, "Op:GTE"         ),
            TokenKind::OpSL              => write!(f, "Op:SL"          ),
            TokenKind::OpSR              => write!(f, "Op:SR"          ),
            TokenKind::OpSShift          => write!(f, "Op:SShift"      ),
            TokenKind::OpImpl            => write!(f, "Op:Impl"        ),
            TokenKind::OpSeqRel          => write!(f, "Op:SeqRel"      ),
            TokenKind::OpFatArrL         => write!(f, "Op:FatArrL"     ),
            TokenKind::OpStarLT          => write!(f, "Op:StarLT"      ),
            TokenKind::OpEquiv           => write!(f, "Op:Equiv"       ),
            TokenKind::OpCompAss         => write!(f, "Composed Assignement (+=, -=, ...)"),
            TokenKind::OpDist            => write!(f, ":= or :/"),
            TokenKind::ParenLeft         => write!(f, "( (ParenLeft)"),
            TokenKind::ParenRight        => write!(f, ") (ParenRight)"),
            TokenKind::CurlyLeft         => write!(f, "{{"),
            TokenKind::CurlyRight        => write!(f, "}}"),
            TokenKind::SquareLeft        => write!(f, "["),
            TokenKind::SquareRight       => write!(f, "]"),
            TokenKind::TickCurly         => write!(f, "'{{"),
            TokenKind::OpRange           => write!(f, "+: or -:"),
            TokenKind::SensiAll          => write!(f, "(*)"),
            TokenKind::Comma             => write!(f, ", (Comma)"),
            TokenKind::Que               => write!(f, "?"),
            TokenKind::Colon             => write!(f, ": (Colon)"),
            TokenKind::Scope             => write!(f, ":: (Scope)"),
            TokenKind::SemiColon         => write!(f, ";"),
            TokenKind::At                => write!(f, "@"),
            TokenKind::At2               => write!(f, "@@"),
            TokenKind::Hash              => write!(f, "#"),
            TokenKind::Hash2             => write!(f, "##"),
            TokenKind::Dot               => write!(f, ". (Dot)"),
            TokenKind::DotStar           => write!(f, ".*"),
            TokenKind::Dollar            => write!(f, "$"),
            TokenKind::LineCont          => write!(f, "\\ (LineCont)"),
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
        "assert"              => Some(TokenKind::KwAssert   ),
        "assign"              => Some(TokenKind::KwAssign   ),
        "assume"              => Some(TokenKind::Keyword    ),
        "automatic"           => Some(TokenKind::KwAutomatic),
        "before"              => Some(TokenKind::Keyword    ),
        "begin"               => Some(TokenKind::KwBegin    ),
        "bind"                => Some(TokenKind::KwBind     ),
        "bins"                => Some(TokenKind::Keyword    ),
        "binsof"              => Some(TokenKind::Keyword    ),
        "break"               => Some(TokenKind::KwBreak    ),
        "buf"                 => Some(TokenKind::Keyword    ),
        "bufif0"              => Some(TokenKind::Keyword    ),
        "bufif1"              => Some(TokenKind::Keyword    ),
        "case"                => Some(TokenKind::KwCase     ),
        "casex"               => Some(TokenKind::KwCase     ),
        "casez"               => Some(TokenKind::KwCase     ),
        "cell"                => Some(TokenKind::Keyword    ),
        "checker"             => Some(TokenKind::Keyword    ),
        "class"               => Some(TokenKind::KwClass    ),
        "clocking"            => Some(TokenKind::KwClocking ),
        "cmos"                => Some(TokenKind::Keyword    ),
        "config"              => Some(TokenKind::Keyword    ),
        "const"               => Some(TokenKind::KwConst    ),
        "constraint"          => Some(TokenKind::KwConstraint),
        "context"             => Some(TokenKind::KwContext   ),
        "continue"            => Some(TokenKind::KwContinue ),
        "cover"               => Some(TokenKind::KwCover    ),
        "covergroup"          => Some(TokenKind::KwCovergroup),
        "coverpoint"          => Some(TokenKind::Keyword    ),
        "cross"               => Some(TokenKind::Keyword    ),
        "deassign"            => Some(TokenKind::Keyword    ),
        "default"             => Some(TokenKind::KwDefault  ),
        "defparam"            => Some(TokenKind::KwDefparam ),
        "design"              => Some(TokenKind::Keyword    ),
        "disable"             => Some(TokenKind::KwDisable  ),
        "dist"                => Some(TokenKind::Keyword    ),
        "do"                  => Some(TokenKind::KwDo       ),
        "edge"                => Some(TokenKind::KwEdge     ),
        "else"                => Some(TokenKind::KwElse     ),
        "end"                 => Some(TokenKind::KwEnd      ),
        "endcase"             => Some(TokenKind::KwEndcase  ),
        "endchecker"          => Some(TokenKind::Keyword    ),
        "endclass"            => Some(TokenKind::KwEndClass ),
        "endclocking"         => Some(TokenKind::KwEndClocking),
        "endconfig"           => Some(TokenKind::Keyword    ),
        "endfunction"         => Some(TokenKind::KwEndFunction ),
        "endgenerate"         => Some(TokenKind::KwEndGenerate),
        "endgroup"            => Some(TokenKind::KwEndGroup    ),
        "endinterface"        => Some(TokenKind::KwEndIntf  ),
        "endmodule"           => Some(TokenKind::KwEndModule),
        "endpackage"          => Some(TokenKind::KwEndPackage),
        "endprimitive"        => Some(TokenKind::Keyword    ),
        "endprogram"          => Some(TokenKind::Keyword    ),
        "endproperty"         => Some(TokenKind::Keyword    ),
        "endspecify"          => Some(TokenKind::Keyword    ),
        "endsequence"         => Some(TokenKind::Keyword    ),
        "endtable"            => Some(TokenKind::Keyword    ),
        "endtask"             => Some(TokenKind::KwEndTask  ),
        "enum"                => Some(TokenKind::KwEnum     ),
        "eventually"          => Some(TokenKind::Keyword    ),
        "expect"              => Some(TokenKind::Keyword    ),
        "export"              => Some(TokenKind::KwExport   ),
        "extends"             => Some(TokenKind::KwExtends  ),
        "extern"              => Some(TokenKind::KwExtern   ),
        "final"               => Some(TokenKind::Keyword    ),
        "first_match"         => Some(TokenKind::Keyword    ),
        "for"                 => Some(TokenKind::KwFor      ),
        "force"               => Some(TokenKind::Keyword    ),
        "foreach"             => Some(TokenKind::KwForeach  ),
        "forever"             => Some(TokenKind::KwForever  ),
        "fork"                => Some(TokenKind::KwFork     ),
        "forkjoin"            => Some(TokenKind::Keyword    ),
        "function"            => Some(TokenKind::KwFunction ),
        "generate"            => Some(TokenKind::KwGenerate ),
        "global"              => Some(TokenKind::KwGlobal   ),
        "highz0"              => Some(TokenKind::KwDrive    ),
        "highz1"              => Some(TokenKind::KwDrive    ),
        "if"                  => Some(TokenKind::KwIf       ),
        "iff"                 => Some(TokenKind::KwIff      ),
        "ifnone"              => Some(TokenKind::Keyword    ),
        "ignore_bins"         => Some(TokenKind::Keyword    ),
        "illegal_bins"        => Some(TokenKind::Keyword    ),
        "implements"          => Some(TokenKind::KwImplements),
        "implies"             => Some(TokenKind::Keyword    ),
        "import"              => Some(TokenKind::KwImport   ),
        "incdir"              => Some(TokenKind::Keyword    ),
        "include"             => Some(TokenKind::Keyword    ),
        "initial"             => Some(TokenKind::KwInitial  ),
        "inout"               => Some(TokenKind::KwInout    ),
        "input"               => Some(TokenKind::KwInput    ),
        "inside"              => Some(TokenKind::KwInside   ),
        "instance"            => Some(TokenKind::Keyword    ),
        "interconnect"        => Some(TokenKind::Keyword    ),
        "interface"           => Some(TokenKind::KwIntf     ),
        "intersect"           => Some(TokenKind::Keyword    ),
        "join"                => Some(TokenKind::KwJoin     ),
        "join_any"            => Some(TokenKind::KwJoin     ),
        "join_none"           => Some(TokenKind::KwJoin     ),
        "large"               => Some(TokenKind::KwCharge   ),
        "let"                 => Some(TokenKind::Keyword    ),
        "liblist"             => Some(TokenKind::Keyword    ),
        "library"             => Some(TokenKind::Keyword    ),
        "local"               => Some(TokenKind::KwLocal    ),
        "localparam"          => Some(TokenKind::KwLParam   ),
        "macromodule"         => Some(TokenKind::KwModule   ),
        "matches"             => Some(TokenKind::KwMatch    ),
        "medium"              => Some(TokenKind::KwCharge   ),
        "modport"             => Some(TokenKind::KwModport  ),
        "module"              => Some(TokenKind::KwModule   ),
        "nand"                => Some(TokenKind::Keyword    ),
        "negedge"             => Some(TokenKind::KwEdge     ),
        "nettype"             => Some(TokenKind::Keyword    ),
        "new"                 => Some(TokenKind::KwNew      ),
        "nexttime"            => Some(TokenKind::Keyword    ),
        "nmos"                => Some(TokenKind::Keyword    ),
        "nor"                 => Some(TokenKind::Keyword    ),
        "noshowcancelled"     => Some(TokenKind::Keyword    ),
        "not"                 => Some(TokenKind::Keyword    ),
        "notif0"              => Some(TokenKind::Keyword    ),
        "notif1"              => Some(TokenKind::Keyword    ),
        "null"                => Some(TokenKind::KwNull      ),
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
        "protected"           => Some(TokenKind::KwProtected),
        "pull0"               => Some(TokenKind::KwDrive    ),
        "pull1"               => Some(TokenKind::KwDrive    ),
        "pulldown"            => Some(TokenKind::Keyword    ),
        "pullup"              => Some(TokenKind::Keyword    ),
        "pulsestyle_ondetect" => Some(TokenKind::Keyword    ),
        "pulsestyle_onevent"  => Some(TokenKind::Keyword    ),
        "pure"                => Some(TokenKind::KwPure     ),
        "rand"                => Some(TokenKind::KwRand     ),
        "randc"               => Some(TokenKind::KwRand     ),
        "randcase"            => Some(TokenKind::Keyword    ),
        "randsequence"        => Some(TokenKind::Keyword    ),
        "rcmos"               => Some(TokenKind::Keyword    ),
        "ref"                 => Some(TokenKind::KwRef      ),
        "reg"                 => Some(TokenKind::KwReg      ),
        "reject_on"           => Some(TokenKind::Keyword    ),
        "release"             => Some(TokenKind::Keyword    ),
        "repeat"              => Some(TokenKind::KwRepeat   ),
        "restrict"            => Some(TokenKind::Keyword    ),
        "return"              => Some(TokenKind::KwReturn   ),
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
        "super"               => Some(TokenKind::KwSuper    ),
        "supply0"             => Some(TokenKind::KwSupply   ),
        "supply1"             => Some(TokenKind::KwSupply   ),
        "sync_accept_on"      => Some(TokenKind::Keyword    ),
        "sync_reject_on"      => Some(TokenKind::Keyword    ),
        "table"               => Some(TokenKind::Keyword    ),
        "tagged"              => Some(TokenKind::KwTagged   ),
        "task"                => Some(TokenKind::KwTask     ),
        "this"                => Some(TokenKind::KwThis     ),
        "throughout"          => Some(TokenKind::Keyword    ),
        "timeprecision"       => Some(TokenKind::KwTimeprec ),
        "timeunit"            => Some(TokenKind::KwTimeunit ),
        "tran"                => Some(TokenKind::Keyword    ),
        "tranif0"             => Some(TokenKind::Keyword    ),
        "tranif1"             => Some(TokenKind::Keyword    ),
        "tri"                 => Some(TokenKind::KwNetType  ),
        "tri0"                => Some(TokenKind::KwNetType  ),
        "tri1"                => Some(TokenKind::KwNetType  ),
        "triand"              => Some(TokenKind::KwNetType  ),
        "trior"               => Some(TokenKind::KwNetType  ),
        "trireg"              => Some(TokenKind::KwNetType  ),
        "type"                => Some(TokenKind::KwType     ),
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
        "virtual"             => Some(TokenKind::KwVirtual  ),
        "wait"                => Some(TokenKind::KwWait     ),
        "wait_order"          => Some(TokenKind::KwWaitOrder),
        "wand"                => Some(TokenKind::KwNetType  ),
        "weak"                => Some(TokenKind::Keyword    ),
        "weak0"               => Some(TokenKind::KwDrive    ),
        "weak1"               => Some(TokenKind::KwDrive    ),
        "while"               => Some(TokenKind::KwWhile    ),
        "wildcard"            => Some(TokenKind::Keyword    ),
        "wire"                => Some(TokenKind::KwNetType  ),
        "with"                => Some(TokenKind::KwWith     ),
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

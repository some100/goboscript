use logos::Logos;

use super::literal::*;
use crate::misc::SmolStr;

#[derive(Debug, Logos, Clone)]
#[logos(skip r"[ \r\t\f]+")]
#[logos(skip r"#[^\n]*\n")]
pub enum Token {
    #[token("%define")]
    Define,
    #[token("%undef")]
    Undef,
    #[token("\n")]
    Newline,
    #[token("\\")]
    Backslash,
    #[regex(r"[_a-zA-Z][_a-zA-Z0-9]*", name)]
    Name(SmolStr),
    #[regex(r"\$[_a-zA-Z0-9]+", arg)]
    Arg(SmolStr),
    #[regex(r"0b[0-1][_0-1]*", bin)]
    Bin(i64),
    #[regex(r"0o[0-7][_0-7]*", oct)]
    Oct(i64),
    #[regex(r"[0-9][_0-9]*", priority=2, callback=int)]
    Int(i64),
    #[regex(r"0x[0-9a-fA-F][_0-9a-fA-F]*", hex)]
    Hex(i64),
    #[regex(r"(0|[1-9][0-9]*)(\.[0-9]+)?([Ee][\-+][0-9]+)?", priority=1, callback=float)]
    Float(f64),
    #[regex(r#""([^"\\]|\\["\\/bfnrt]|\\u[0-9a-zA-Z]{4})*""#, string)]
    Str(SmolStr),
    #[regex(r#"```([^`]|\n)*```"#, cmd)]
    Cmd(SmolStr),
    #[token("costumes")]
    Costumes,
    #[token("sounds")]
    Sounds,
    #[token("local")]
    Local,
    #[token("proc")]
    Proc,
    #[token("func")]
    Func,
    #[token("return")]
    Return,
    #[token("nowarp")]
    NoWarp,
    #[token("on")]
    On,
    #[token("onflag")]
    OnFlag,
    #[token("onkey")]
    OnKey,
    #[token("onclick")]
    OnClick,
    #[token("onbackdrop")]
    OnBackdrop,
    #[token("onloudness")]
    OnLoudness,
    #[token("ontimer")]
    OnTimer,
    #[token("onclone")]
    OnClone,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("elif")]
    Elif,
    #[token("until")]
    Until,
    #[token("for")]
    For,
    #[token("forever")]
    Forever,
    #[token("repeat")]
    Repeat,
    #[token(",")]
    Comma,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("=")]
    Assign,
    #[token("==")]
    Eq,
    #[token("++")]
    Increment,
    #[token("--")]
    Decrement,
    #[token("+=")]
    AssignAdd,
    #[token("-=")]
    AssignSubtract,
    #[token("*=")]
    AssignMultiply,
    #[token("/=")]
    AssignDivide,
    #[token("//=")]
    AssignFloorDiv,
    #[token("%=")]
    AssignModulo,
    #[token("&=")]
    AssignJoin,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(".")]
    Dot,
    #[token("!=")]
    Ne,
    #[token("<")]
    Lt,
    #[token(">")]
    Gt,
    #[token("<=")]
    Le,
    #[token(">=")]
    Ge,
    #[token("not")]
    Not,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("in")]
    In,
    #[token("&")]
    Amp,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("//")]
    FloorDiv,
    #[token("%")]
    Percent,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token("length")]
    Length,
    #[token("round")]
    Round,
    #[token("abs")]
    Abs,
    #[token("floor")]
    Floor,
    #[token("ceil")]
    Ceil,
    #[token("sqrt")]
    Sqrt,
    #[token("sin")]
    Sin,
    #[token("cos")]
    Cos,
    #[token("tan")]
    Tan,
    #[token("asin")]
    Asin,
    #[token("acos")]
    Acos,
    #[token("atan")]
    Atan,
    #[token("ln")]
    Ln,
    #[token("log")]
    Log,
    #[token("antiln")]
    Antiln,
    #[token("antilog")]
    Antilog,
    #[token("show")]
    Show,
    #[token("hide")]
    Hide,
    #[token("add")]
    Add,
    #[token("to")]
    To,
    #[token("delete")]
    Delete,
    #[token("insert")]
    Insert,
    #[token("at")]
    At,
    #[token("of")]
    Of,
    #[token("as")]
    As,
    #[token("enum")]
    Enum,
    #[token("struct")]
    Struct,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("list")]
    List,
    #[token("cloud")]
    Cloud,
    #[token("|>")]
    Pipe,
}

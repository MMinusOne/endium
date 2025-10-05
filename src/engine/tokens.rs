#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Const,
    Let,
    Var,
    Function,
    Return,
    Yield,
    If,
    Else,
    Switch,
    Case,
    Break,
    Continue,
    Default,
    For,
    While,
    Do,
    Try,
    Catch,
    Finally,
    Throw,
    Class,
    Extends,
    Super,
    This,
    New,
    Import,
    Export,
    From,
    As,
    Async,
    Await,
    With,
    In,
    Of,
    InstanceOf,
    Typeof,
    Delete,
    Void,

    // Literals
    True,
    False,
    Null,
    Undefined,
    Number(String),
    BigNumber(String),
    String(String),
    Identifier(String),
    TemplateString(Vec<Token>),

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Exponent,

    // Assignment
    Assign,
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    ExponentAssign,

    // Increment/Decrement
    Increment,
    Decrement,

    // Comparison
    Equal,
    NotEqual,
    StrictEqual,
    StrictNotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,

    // Logical
    LogicalAnd,
    LogicalOr,
    LogicalNot,

    // Bitwise
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseNot,
    LeftShift,
    RightShift,
    UnsignedRightShift,

    // Other operators
    ArrowFunction,
    Ternary(Vec<Token>, Vec<Token>, Vec<Token>),
    NullishCoalescing,
    OptionalChaining,
    Spread,

    // Punctuation
    Semicolon,
    Comma,
    Dot,
    Colon,

    // Brackets
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    // Special
    Newline,
    Comment(String),
    BlockComment(String),

    // End of file
    Eof,
    NoToken,
}

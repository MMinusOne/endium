pub enum Tokens {
    // Variable assignment
    Const,
    Let,
    Var,

    //Booleans
    True,
    False,

    // Misc
    Increment,     // ++
    Decrement,     // --
    Null,          // null
    Undefined,     // undefined
    With,          // with
    Comment,       // //
    Spread,        // ...
    OptionalChain, // ?.

    // Math
    Addition,       // +
    Subtraction,    // -
    Multiplication, // *
    Division,       // /
    Exponentiation,       // **
    Modulo,         // %

    // Assignment
    Assignment,     // =
    AssignAddition, // +=
    AssignSubtract, // -=
    AssignMultiply, // *=
    AssignDivide,   // /=

    //Loops
    For,      // for
    While,    // while
    Continue, // continue
    Do,       // do

    // Functions,
    Function,      // function
    Return,        // return,
    Yield,         // yield
    ArrowFunction, // =>

    // Comparasions
    Equal,               // ==
    StrictlyEqual,       // ===
    StrictlyNotEqual,    // !==
    NotEqual,            // !=
    BiggerThan,          // >
    LesserThan,          // <
    NotBiggerThan,       // !<
    NotLessThan,         // >!
    BiggerThanOrEqualTo, // >=
    LessThanOrEqualTo,   // =<
    InstanceOf,          // instanceof
    In,                  // in
    BitWiseAnd,          // &
    LogicalAnd,          // &&
    LogicalOr,           // ||
    Ternary,             // ?
    Negation,            // !
    NullishCaelscing,    // ??

    //Promises
    Async, // async
    Await, // await

    //Statements
    If,      // if
    Else,    // else
    Switch,  // switch
    Case,    // case
    Break,   // break
    Default, // default

    // Classes
    Class,   // class
    Extends, // extends
    Super,   // super
    This,    //this
    New,     // new

    // Error handling
    Try,     // try
    Catch,   // catch
    Finally, // finally

    // Punctuation
    Comma,            // /
    Semicolon,        // ;
    Colon,            // :
    Dot,              // .
    OpenParanthesis,  // (
    CloseParanthesis, // )
    OpenBrace,        // {
    CloseBrace,       // }
    OpenBracket,      // [
    CloseBracket,     // ]
    Quote,            // '
    DoubleQuote,      // "
    Backtick,         // `

    // Modules
    Import, // import
    Export, // export
}

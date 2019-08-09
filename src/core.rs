use std::collections::HashMap;

use crate::parse_tree::Node;
use crate::parser::Parser;

///////////////////////
// Token Definitions //
///////////////////////
#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum TokenType {
    Term,
    Identifier,

    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Exponent,

    Equals,
    DoubleEquals,
    TripleEquals,
    LessThan,
    LessThanEquals,
    GreaterThan,
    GreaterThanEquals,
    Not,

    LeftCurlyBrace,
    RightCurlyBrace,
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Colon,
    Semicolon,
    Newline,
    EndOfFile,

    For,
    While,
    Define,
    If,
    Elif,
    Else,

    None,
    At,
    Dot,
    Ellipsis,

    Mango,
    StatementSuite,
    StatementSuiteFunction,
    StatementSuiteClass,
    StatementListClass,
    StatementRestricted,
    StatementList,
    Statement,
    StatementSimple,
    StatementComplex,
    StatementLimited,
    StatementFunction,
    StatementClass,
    StatementListFunction,
    StatementExpression,
    StatementExpressionP,
    StatementExpression2,
    StatementExpression2p,
    StatementExpression3,
    StatementAssignment,
    StatementConditional,
    StatementConditional2,
    StatementConditional3,
    StatementConditionalElif,
    StatementConditionalElse,
    StatementConditionalTest,
    ConditionalExpression,
    ComparisonOperator,
    ComparisonOperatorUnary,
    StatementLoop,
    StatementLoopFor,
    StatementLoopForOptions,
    StatementLoopWhile,
    StatementDefineFunction,
    FunctionParams,
    StatementDefineClass,
}

///////////////////
// Lexer Structs //
///////////////////
#[derive(Debug, Clone)]
pub enum PrimitiveType {
    Float,
    Integer,
    String,
    Boolean,
    Object,
    Function,
    None,
}

#[derive(Debug, Clone)]
pub struct LexerResult {
    pub token: String,
    pub inferred_type: PrimitiveType,
    pub token_type: TokenType,
}

impl LexerResult {
    pub fn to_string(&self) -> String {
        return format!("LexerResult: (token_type: {:?})", self.token_type);
    }
}

////////////////////
// Parser Structs //
////////////////////
#[derive(Debug, Eq, PartialEq, Hash)]
pub enum ParserAction {
    Shift,
    Reduce,
    Accept,
    Goto,
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ActionNode {
    pub action: ParserAction,
    pub value: i32,
}

impl ActionNode {
    pub fn to_string(&self) -> String {
        return format!("ActionNode: (action: {:?}, value: {:?})", self.action, self.value);
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct GotoNode {
    pub token_type: TokenType,
    pub value: i32,
}

/////////////////////
// Scoping Structs //
/////////////////////
pub struct Scope {
    pub symbol_table: HashMap<String, Box<dyn Node + Sync>>,
}

impl Default for Scope {
    fn default() -> Self {
        Scope {
            symbol_table: HashMap::<String, Box<dyn Node + Sync>>::new(),
        }
    }
}


/////////////////
// Keyword Map //
/////////////////
pub fn identifier_to_enum(symbol: &str) -> TokenType {
    match symbol {
        "if" => TokenType::If,
        "elif" => TokenType::Elif,
        "else" => TokenType::Else,
        "for" => TokenType::For,
        "while" => TokenType::While,
        _ => TokenType::None,
    }
}

////////////////
// Symbol Map //
////////////////
pub fn symbol_to_enum(symbol: &str) -> TokenType {
    match symbol {
        "+" => TokenType::Add,
        "-" => TokenType::Subtract,
        "*" => TokenType::Multiply,
        "/" => TokenType::Divide,
        "%" => TokenType::Modulo,
        "^" => TokenType::Exponent,
        "=" => TokenType::Equals,
        "==" => TokenType::DoubleEquals,
        "===" => TokenType::TripleEquals,
        "<" => TokenType::LessThan,
        "<=" => TokenType::LessThanEquals,
        ">" => TokenType::GreaterThan,
        ">=" => TokenType::GreaterThanEquals,
        "!" => TokenType::Not,
        "{" => TokenType::LeftCurlyBrace,
        "}" => TokenType::RightCurlyBrace,
        "(" => TokenType::LeftParenthesis,
        ")" => TokenType::RightParenthesis,
        "," => TokenType::Comma,
        ":" => TokenType::Colon,
        ";" => TokenType::Semicolon,
        "\n" => TokenType::Newline,
        "@" => TokenType::At,
        "." => TokenType::Dot,
        ".." => TokenType::Ellipsis,
        "$" => TokenType::EndOfFile,
        _ => TokenType::None,
    }
}
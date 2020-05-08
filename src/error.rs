use crate::tokens::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedOperator(Token),
    UnexpectedToken(Token),
    ExpectedIdentifier(Token),
    ExpectedLiteral(Token),
    TokenStreamEmpty,
}
#[derive(Debug)]
pub enum CompileError {
    UndefinedVariable,
    ContextStreamEmpty,
    UnexpectedExpression,
    UnexpectedBinaryOperator(Token),
    UnexpectedLogicalOperator(Token),
    UnexpectedUnaryOperator(Token),
    ReturnInScript,
}
#[derive(Debug)]
pub enum RuntimeError {
    StackEmpty,
    CallFrameEmpty,
    IndexOutOfBounds,
    IncorrectArity,
    UndefinedVariable,
    ExpectedStringConstant,
    ExpectedArray,
    ExpectedNumber,
    ExpectedCallee,
}

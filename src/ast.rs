//! The abstract syntax tree of buildlua.
//!
//! Most of this file is based on the syntax of lua page in the lua manual.
//!
//! The page can be found at http://www.lua.org/manual/5.2/manual.html#9.

/// The largest unit of code in lua. Equates to an entire file of code.
pub struct Chunk(pub Block);

pub struct Block(pub Option<Vec<Statement>>, pub Option<Box<ReturnStatement>>);

pub enum Statement {
    Semicolon,
    Assignment(Box<VariableList>, Box<ExpressionList>),
    FunctionCall(Box<FunctionCall>),
    Label(Label),
    Break,
    Goto(Label),
    Do(Box<Block>),
    While { exp: Box<Expression>, do_: Box<Block> },
    Repeat { block: Box<Block>, until: Box<Expression> },
    If {
        condition: Box<Expression>, then: Box<Block>,
        elseif_condition: Option<Box<Expression>>, elsethen: Option<Box<Block>>,
        else_: Box<Block>
    },
    ForStepping { name: String, from: Box<Expression>, to: Box<Expression>, step: Option<Box<Expression>>, block: Box<Block> },
    ForIn { name_list: Box<NameList>, in_: Box<ExpressionList>, do_: Box<Block> },
    Function(FunctionName, Box<FunctionBody>),
    LocalFunction { name: String, body: Box<FunctionBody> },
    LocalVariableBinding(NameList, Option<ExpressionList>),
}

pub struct ReturnStatement(pub Box<ExpressionList>);
pub struct Label(pub String);

pub struct FunctionName {
    pub first_dot_access: String,
    pub rest_dot_access: Option<Vec<String>>,
    pub self_name: Option<String>,
}

pub struct VariableList {
    pub first: Box<Variable>,
    pub rest: Option<Vec<Variable>>,
}

pub enum Variable {
    Name(String),
    ArrayAccess { from: Box<PrefixExpression>, key: Box<Expression> },
    DotAccess { from: Box<PrefixExpression>, key: String },
}

pub struct NameList(pub String, pub Option<Vec<String>>);
pub struct ExpressionList(pub Box<Expression>, pub Option<Vec<Expression>>);

pub enum Expression {
    Nil,
    False,
    True,
    Number(f64),
    String(String),
    /// Reperesents the lua ... variable in a function that takes extended arguments with a ...
    /// at the end of it's argument list.
    ExtendedArgumentAccess,
    FunctionDefine(Box<FunctionDefine>),
    PrefixExpression(Box<PrefixExpression>),
    TableConstructor(Box<TableConstructor>),
    BinaryOperation(Box<Expression>, Box<Expression>),
    UnaryOperation(Box<Expression>),
}

pub enum PrefixExpression {
    Variable(Box<Variable>),
    FunctionCall(Box<FunctionCall>),
    Parenthesis(Box<Expression>),
}

pub enum FunctionCall {
    /// Calling the function with a . to access through tables or through the global enviroment,
    /// not passing a self argument.
    ///
    /// # Example
    /// ```lua
    /// abcde("foo")
    /// bar.foo("foobar")
    /// ```
    Static(Box<PrefixExpression>, Box<FunctionArguments>),
    /// Calling with a self argument, using a : access.
    ///
    /// # Example
    /// ```lua
    /// foo:bar("barfoo")
    /// ```
    SelfTaking(Box<PrefixExpression>, String, FunctionArguments),
}

pub enum FunctionArguments {
    /// Calling a function normally.
    ///
    /// # Example
    /// ```lua
    /// foo("abc")
    /// ```
    Parenthesis(Option<Box<ExpressionList>>),
    /// Calling the function with a single table.
    ///
    /// # Example
    /// ```lua
    /// foo{a = "aaa"}
    /// ```
    TableConstructor(Box<TableConstructor>),
    /// Calls the function with a singlur literal string.
    ///
    /// # Example
    /// ```lua
    /// foo"abc"
    /// ```
    String(String), // This is really weird. I see the point, but why?
}

pub struct FunctionDefine(pub Box<FunctionBody>);
pub struct FunctionBody(pub Option<Box<ParameterList>>, pub Box<Block>);

pub enum ParameterList {
    /// A normal parameter list, without any extended arguments or special features.
    ///
    /// # Example
    /// ```lua
    /// function foo(a, b)
    /// end
    /// ```
    NameList(Box<NameList>),
    /// A parameter list with both arguments and an extended argument.
    ///
    /// # Example
    /// ```lua
    /// function foo(a, b, ...)
    /// ```
    ExtendedArguments(Box<NameList>),
    /// A function with an extended argument, but no regular arguments.
    ///
    /// # Example
    /// ```lua
    /// function foo(...)
    /// ```
    ExtendedArgumentsVoid,
}

pub struct TableConstructor(pub Box<FieldList>);
pub struct FieldList(pub Box<Field>, pub Option<Vec<Field>>);

pub enum Field {
    /// A field in a table that takes an expression for the name of the field.
    ExpressionForName { name: Box<Expression>, equals: Box<Expression> },
    /// A normal field.
    ///
    /// # Example
    /// ```lua
    /// a = { b = "bbb" }
    /// ```
    Equals { name: String, equals: Box<Expression> },
    /// A field in the form of an array.
    ///
    /// # Example
    /// ```lua
    /// a = { "bbb" }
    /// ```
    ArrayStyle(Box<Expression>),
}

pub enum BinaryOperation {
    /// + sigil.
    Plus,
    /// - sigil, when used as a binary operation.
    Minus,
    /// * sigil.
    Times,
    /// / sigil.
    Devide,
    /// ^ sigil.
    Exponent,
    /// % sigil.
    Modulo,
    /// .. sigil.
    Concatanate,
    /// < sigil.
    LessThan,
    /// <= sigil.
    LessThanOrEqual,
    /// > sigil.
    GreaterThan,
    /// >= sigil.
    GreaterThanOrEqual,
    /// == sigil.
    Equal,
    /// ~= sigil.
    NotEqual,
    And,
    Or,
}

pub enum UnaryOperation {
    /// - sigil, when used as a unary operation.
    Negate,
    Not,
    /// # sigil.
    Length,
}

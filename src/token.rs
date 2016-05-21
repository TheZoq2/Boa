
#[derive(Clone,Copy,Eq,PartialEq,Hash,Debug)]
pub enum TokenType {
    Whitespace,
    LineComment,
    BlockComment,

    StringLiteral,

    Identifier,
    Keyword,
    Number,
    HexNumber,

    Operator,
    Assignment,

    EndStatement,
    OpenPar,
    ClosePar,
    OpenCurl,
    CloseCurl,
    OpenSq,
    CloseSq,
}

/*
   Represents a token of the language that is created by the lexer
   and parsed by the parser
 */
#[derive(Clone,Eq,PartialEq,Debug)]
pub struct Token
{
    lexeme: String, //The 'contents' of the token
    token_type: TokenType, //The type of the token
}
impl Token 
{
    pub fn new(lexeme: String, token_type: TokenType) -> Token 
    {
        Token {
            lexeme: lexeme,
            token_type: token_type
        }
    }
}

extern crate regex

enum TokenType {
    identifier,
    value,
    operator,
    assignment,

    if_statement,
}

#[derive(Clone, Copy)]
pub struct StatementTemplate
{
    token_type: TokenType,

    inner_statements: std::vec<StatementTemplate>,
}

impl StatementTemplate
{
    pub fn new() -> StatementTemplate 
    {
        StatementTemplate {
            
        }
    }
}

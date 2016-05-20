
use regex::{Regex};
use std::collections::{HashMap};

const OPERATORS: [&'static str; 9] = [
    "==",
    "!=",
    "<=",
    ">=",
    "+",
    "-",
    "/",
    "*",
    "!"
];

const KEYWORDS: [&'static str; 9] = [
    "if",
    "elseif",
    "else",

    "def",
    "for",
    "foreach",
    "while",

    "type",
    "implement",
];

#[derive(Clone,Copy,Eq,PartialEq,Hash)]
enum TokenType {
    Whitespace,

    Identifier,
    Operator,
    Keyword,
    Number,
    HexNumber,

    Assignment,

    OpenPar,
    ClosePar,
    OpenCurl,
    CloseCurl,
    OpenSq,
    CloseSq,
    Quotes,
}

pub struct Lexer
{
    token_templates: HashMap<TokenType, Regex>,
}

impl Lexer
{
    pub fn new() -> Lexer 
    {
        let mut result = Lexer {
            token_templates: HashMap::new()
        };

        result.setup_templates();

        return result;
    }
    
    fn setup_templates(&mut self)
    {
        self.token_templates = HashMap::new();

        self.add_token_template(TokenType::Whitespace, r"\A\s\s*[\S$]?");
        self.add_token_template(TokenType::Identifier, r"[a-zA-Z_]\w*\b");
        self.add_token_template(TokenType::Number, r"\A[0-9]*[.]?[0-9]*\b");
        self.add_token_template(TokenType::HexNumber, r"\A[0]x[0-9A-Fa-f][0-9A-Fa-f]*\b");

        let mut operator_regex = String::from("[");
        for op in OPERATORS.into_iter()
        {
            operator_regex.push_str(op);
        }
        operator_regex.push_str("]*");
        self.add_token_template(TokenType::Operator, operator_regex.as_str());
    }

    fn add_token_template(&mut self, token_type: TokenType, reg: &str)
    {
        self.token_templates.insert(token_type, Regex::new(reg).unwrap());
    }

    #[cfg(test)]
    fn matches_token(&self, token_type: TokenType, string: &String) -> bool
    {
        let re = match self.token_templates.get(&token_type){
            Some(re) => re,
            None => {println!("Warning, no such token type in lexer"); return false}
        };

        //return re.is_match(string);
        match re.find(string){
            Some((0, len)) => if len == string.len()
                              {
                                  return true
                              }
                              else
                              {
                                  return false
                              },
            _ => return false
        };
    }
}

#[cfg(test)]
mod lexer_tests
{
    use lexer::Lexer;
    use lexer::TokenType;

    #[test]
    fn simple_regex_test()
    {
        let lexer = Lexer::new();

        //Whitespace test
        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from(" ")), true);
        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from("    ")), true);
        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from(" \t \n \t ")), true);
        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from("\n\t \n \t ")), true);

        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from("")), false);
        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from("a   ")), false);
        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from(" a ")), false);

        //Identifier tests
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("abuadmewkh")), true);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("abuadmewkh2")), true);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("ab2ad5e0kh2")), true);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("a_b2ad_5e0_kh2")), true);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("_a_b2ad_5e0_kh2")), true);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("_A_b2aQ_5e0_Hh2")), true);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("A_b2aQ_5e0_Hh2")), true);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("A_b2aQ_5e0_Hh2_")), true);

        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("1abuadmewkh")), false);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("1abu;admewkh")), false);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("A_b2aQ_5e0_Hh2_{")), false);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("A_b2aQ_5e0_Hh2_.")), false);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("A_b2aQ_5e0_Hh2_;")), false);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("A_b2aQ_5e0_Hh2_`")), false);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("A_b2aQ_5e0_Hh2_,")), false);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("")), false);

        //Number tests
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("1223")), true);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0")), true);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("00000001")), true);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("00205001")), true);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0.5")), true);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from(".5")), true);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from(".50000000")), true);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("3.141527")), true);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from(".141527")), true);

        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("518248.")), false);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("")), false);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0x005")), false);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("5a")), false);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("abcd")), false);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0,5")), false);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0,5")), false);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("500.0.1")), false);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("500..1")), false);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("")), false);

        //Hex number tests
        assert_eq!(lexer.matches_token(TokenType::HexNumber, &String::from("0x0")), true);
        assert_eq!(lexer.matches_token(TokenType::HexNumber, &String::from("0x000000000")), true);
        assert_eq!(lexer.matches_token(TokenType::HexNumber, &String::from("0x00ac0b00a")), true);
        assert_eq!(lexer.matches_token(TokenType::HexNumber, &String::from("0xABCDEF0")), true);
        assert_eq!(lexer.matches_token(TokenType::HexNumber, &String::from("0xabcdef0")), true);
        assert_eq!(lexer.matches_token(TokenType::HexNumber, &String::from("0x0123456789abcdef")), true);

        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("")), false);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0x")), false);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0x0G")), false);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0x0q")), false);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0xq")), false);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0x_")), false);
    }
}


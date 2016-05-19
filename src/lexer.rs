
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
        self.add_token_template(TokenType::Identifier, r"\A[:alpha:_][:alphanum:_]*[^:alphanum:_$]");
        self.add_token_template(TokenType::Number, r"\A[:digit:]*\.+[:digits:]");
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
                                  println!("Wrong length {} != {}", len, string.len());
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
    }
}


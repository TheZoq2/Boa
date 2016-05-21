
use regex::{Regex};
use std::collections::{HashMap};

use token::*;

const OPERATORS: [&'static str; 17] = [
    "==",
    "!=",
    "<=",
    ">=",
    ">",
    "<",

    //These need to go before the shorter operators to make sure they are captured
    //first by regex
    "++",
    "--",
    "+=",
    "-=",
    "*=",
    "/=",

    "+",
    "-",
    "/",
    "*",
    "!",

];

fn get_operator_regex() -> String
{
    let mut operator_regex = String::from(r"\A(");
    for op in OPERATORS.into_iter()
    {
        //Escape all characters that need to be escaped
        let mut escaped = op.replace("*", r"\*");
        escaped = escaped.replace("+", r"\+");
        escaped = escaped.replace("-", r"-");

        operator_regex.push_str("(");
        operator_regex.push_str(escaped.as_str());
        operator_regex.push_str(")|");
    }
    //Remove any trailing | character
    if operator_regex.ends_with("|")
    {
        operator_regex.pop();
    }
    operator_regex.push_str(r")(\b|\B|$)");

    return operator_regex;
}

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


struct TokenTemplate
{
    token_type: TokenType,
    regex: Regex
}

/*
 * Lexer that goes through the code and tokenises it
 */
pub struct Lexer
{
    token_templates: Vec<TokenTemplate>,
}

impl Lexer
{
    pub fn new() -> Lexer 
    {
        let mut result = Lexer {
            token_templates: Vec::new()
        };

        result.setup_templates();

        return result;
    }

    //Returns a tokenized version of the string
    pub fn tokenize(&self, code: String) -> Vec<Token>
    {
        let mut current_code = code.clone();

        let mut tokens = Vec::new();
        while current_code.len() != 0
        {
            let mut found_matching = false;

            //Go through the token templates to find matches
            for token_template in &self.token_templates
            {
                let re = &token_template.regex;

                match re.find(current_code.as_str()){
                    Some((0, end)) => {
                        let new_token = Token::new(current_code.drain(..end).collect(), token_template.token_type);

                        tokens.push(new_token);
                        found_matching = true;
                        break;
                    }
                    Some((start, _)) => panic!("Found match but not at start. Type: {:?}, Code: {}", token_template.token_type, current_code),
                    _ =>{} //TODO: Do something nicer than this
                }
            }

            if !found_matching
            {
                //Somehow alert the user that no matching patterns were found
                panic!("No matching pattern was found for the character {}", code.chars().next().unwrap());
            }
        }
        return tokens;
    }
    
    fn setup_templates(&mut self)
    {
        self.add_token_template(TokenType::Whitespace, r"\A\s\s*(\b|\B|$)");
        self.add_token_template(TokenType::LineComment, r"\A(?m:#.*$)");
        self.add_token_template(TokenType::BlockComment, r"\A/\*(.|\s)*\*/");

        self.add_token_template(TokenType::StringLiteral, r#"\A("(.|\s)*")|(\A'(.|\s)*')"#);

        self.add_token_template(TokenType::Identifier, r"\A[a-zA-Z_]\w*\b");
        self.add_token_template(TokenType::Number, r"\A[0-9][0-9]*[.]?[0-9]*\b");
        self.add_token_template(TokenType::HexNumber, r"\A[0]x[0-9A-Fa-f][0-9A-Fa-f]*\b");

        self.add_token_template(TokenType::EndStatement, ";");

        self.add_token_template(TokenType::Operator, get_operator_regex().as_str());
    }

    fn add_token_template(&mut self, token_type: TokenType, reg: &str)
    {
        //Create a new template and add it to the list
        self.token_templates.push(TokenTemplate{token_type: token_type, regex: Regex::new(reg).unwrap()});
    }
}

/////////////////////////////////////////////////////////////////////////////////
///                 Test tings
/////////////////////////////////////////////////////////////////////////////////
//Test helper functions
#[cfg(test)]
#[derive(Eq,PartialEq,Debug)]
enum MatchType
{
    Match,
    WrongLen(usize),
    NoMatch,
    NoRegex
}

#[cfg(test)]
impl Lexer
{
    fn matches_token(&self, token_type: TokenType, string: &String) -> MatchType
    {
        let mut regex: Option<&Regex> = None;

        for template in &self.token_templates
        {
            if template.token_type == token_type
            {
                regex = Some(&template.regex);
            }
        }

        let re = match regex{
            Some(re) => re,
            None => {println!("Warning, no such token type in lexer"); return MatchType::NoRegex}
        };

        //return re.is_match(string);
        match re.find(string){
            Some((0, len)) => if len == string.len()
                              {
                                  return MatchType::Match
                              }
                              else
                              {
                                  return MatchType::WrongLen(len)
                              },
            _ => return MatchType::NoMatch
        };
    }
}

#[cfg(test)]
mod lexer_tests
{
    use lexer::Lexer;
    use token::TokenType;
    use token::Token;
    use lexer::MatchType;

    #[test]
    fn simple_regex_test()
    {
        let lexer = Lexer::new();

        //Whitespace test
        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from(" ")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from("    ")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from(" \t \n \t ")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from("\n\t \n \t ")), MatchType::Match);

        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from("")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from("a   ")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from(" a ")), MatchType::WrongLen(1));
        assert_eq!(lexer.matches_token(TokenType::Whitespace, &String::from(" + ")), MatchType::WrongLen(1));

        //Identifier tests
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("abuadmewkh")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("abuadmewkh2")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("ab2ad5e0kh2")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("a_b2ad_5e0_kh2")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("_a_b2ad_5e0_kh2")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("_A_b2aQ_5e0_Hh2")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("A_b2aQ_5e0_Hh2")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("A_b2aQ_5e0_Hh2_")), MatchType::Match);

        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("1abuadmewkh")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("abu;admewkh")), MatchType::WrongLen(3));
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("A2_{")), MatchType::WrongLen(3));
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from(" + ")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("+ ")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from(" + de")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Identifier, &String::from("")), MatchType::NoMatch);

        //Number tests
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("1223")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("00000001")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("00205001")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0.5")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("3.141527")), MatchType::Match);

        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("58.")), MatchType::WrongLen(2));
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("192,")), MatchType::WrongLen(3));
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0x005")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("5a")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("abcd")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0,5")), MatchType::WrongLen(1));
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("500.0.1")), MatchType::WrongLen(5));
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("500..1")), MatchType::WrongLen(3));
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from(".5")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from(".141527")), MatchType::NoMatch);

        //Hex number tests
        assert_eq!(lexer.matches_token(TokenType::HexNumber, &String::from("0x0")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::HexNumber, &String::from("0x000000000")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::HexNumber, &String::from("0x00ac0b00a")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::HexNumber, &String::from("0xABCDEF0")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::HexNumber, &String::from("0xabcdef0")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::HexNumber, &String::from("0x0123456789abcdef")), MatchType::Match);

        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0x")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0x0G")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0x0q")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0xq")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Number, &String::from("0x_")), MatchType::NoMatch);

        //Operator tests
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("==")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("!=")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("<=")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from(">=")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("<")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from(">")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("+")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("-")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("*")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("/")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("+=")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("-=")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("*=")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("/=")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("++")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("--")), MatchType::Match);

        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from(" + ")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from(" == ")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from(" !==")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("===")), MatchType::WrongLen(2));
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("=")), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("**")), MatchType::WrongLen(1));
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("+++")), MatchType::WrongLen(2));
        assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("   jwad++")), MatchType::NoMatch);

        //TODO:
        //assert_eq!(lexer.matches_token(TokenType::Operator, &String::from("===")), MatchType::NoMatch);

        //Comments
        assert_eq!(lexer.matches_token(TokenType::LineComment, &String::from("#++abc123")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::LineComment, &String::from("#")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::LineComment, &String::from("#yolo")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::BlockComment, &String::from("/**/")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::BlockComment, &String::from("/*yoloswag\n multiline*/")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::BlockComment, &String::from("/*yoloswagsinglelinecomment*/")), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::BlockComment, &String::from("/*    */")), MatchType::Match);

        //String literals
        assert_eq!(lexer.matches_token(TokenType::StringLiteral, &String::from(r#""yolo\nmulti""#)), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::StringLiteral, &String::from(r#""yolo""#)), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::StringLiteral, &String::from(r#"'yolo'"#)), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::StringLiteral, &String::from(r#"'y o " lo" '"#)), MatchType::Match);
        assert_eq!(lexer.matches_token(TokenType::StringLiteral, &String::from(r#""yo'lo'""#)), MatchType::Match);

        assert_eq!(lexer.matches_token(TokenType::StringLiteral, &String::from(r#""yolo'"#)), MatchType::NoMatch);
        assert_eq!(lexer.matches_token(TokenType::StringLiteral, &String::from(r#"   "yolo""#)), MatchType::NoMatch);
    }

    #[test]
    fn simple_lexer_test()
    {
        let lexer = Lexer::new();

        //Simple operator test
        {
            let code = String::from("abc + de");
            let tokens = vec!(
                    Token::new(String::from("abc"), TokenType::Identifier),
                    Token::new(String::from(" "), TokenType::Whitespace),
                    Token::new(String::from("+"), TokenType::Operator),
                    Token::new(String::from(" "), TokenType::Whitespace),
                    Token::new(String::from("de"), TokenType::Identifier),
                );
            assert_eq!(lexer.tokenize(code), tokens);
        }

        //More complex operator test
        {
            let code = String::from("abc + de * 0.5");
            let tokens = vec!(
                    Token::new(String::from("abc"), TokenType::Identifier),
                    Token::new(String::from(" "), TokenType::Whitespace),
                    Token::new(String::from("+"), TokenType::Operator),
                    Token::new(String::from(" "), TokenType::Whitespace),
                    Token::new(String::from("de"), TokenType::Identifier),
                    Token::new(String::from(" "), TokenType::Whitespace),
                    Token::new(String::from("*"), TokenType::Operator),
                    Token::new(String::from(" "), TokenType::Whitespace),
                    Token::new(String::from("0.5"), TokenType::Number),
                );
            assert_eq!(lexer.tokenize(code), tokens);
        }

        //More complex operator test with hex value
        {
            let code = String::from("abc + de * 0xaa5");
            let tokens = vec!(
                    Token::new(String::from("abc"), TokenType::Identifier),
                    Token::new(String::from(" "), TokenType::Whitespace),
                    Token::new(String::from("+"), TokenType::Operator),
                    Token::new(String::from(" "), TokenType::Whitespace),
                    Token::new(String::from("de"), TokenType::Identifier),
                    Token::new(String::from(" "), TokenType::Whitespace),
                    Token::new(String::from("*"), TokenType::Operator),
                    Token::new(String::from(" "), TokenType::Whitespace),
                    Token::new(String::from("0xaa5"), TokenType::HexNumber),
                );
            assert_eq!(lexer.tokenize(code), tokens);
        }

        //Single operator test
        {
            let code = String::from("abc++");
            let tokens = vec!(
                    Token::new(String::from("abc"), TokenType::Identifier),
                    Token::new(String::from("++"), TokenType::Operator),
                );
            assert_eq!(lexer.tokenize(code), tokens);
        }

        //Regular comment test
        {
            let code = String::from("#Comment test\n abc++");
            let tokens = vec!(
                    Token::new(String::from("#Comment test"), TokenType::LineComment),
                    Token::new(String::from("\n "), TokenType::Whitespace),
                    Token::new(String::from("abc"), TokenType::Identifier),
                    Token::new(String::from("++"), TokenType::Operator),
                );
            assert_eq!(lexer.tokenize(code), tokens);
        }

        //Multiline comment test
        {
            let code = String::from("/*Multi\n line\n comment\n test*/abc++");
            let tokens = vec!(
                    Token::new(String::from("/*Multi\n line\n comment\n test*/"), TokenType::BlockComment),
                    Token::new(String::from("abc"), TokenType::Identifier),
                    Token::new(String::from("++"), TokenType::Operator),
                );
            assert_eq!(lexer.tokenize(code), tokens);
        }
    }
}

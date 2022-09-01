use std::collections::HashMap;
use tex_parser::ast::*;

// function to extract the next n adjacent tokens from the latex ast from
// the macro with the specified content in its MacroName
fn get_n_adjacent_tokens_to_macro(tokens: &Vec<Token>, content: &str, n: usize) -> Vec<Token> {
    let mut adjacent_tokens: Vec<Token> = Vec::new();
    for (i, token) in tokens.iter().enumerate() {
        match token {
            Token::Macro(macro_token) => match macro_token.name.content.as_str() {
                tok_content => {
                    if tok_content == content {
                        for j in i + 1..i + n + 1 {
                            adjacent_tokens.push(tokens[j].clone());
                        }
                    }
                }
            },
            _ => (),
        }
    }
    adjacent_tokens
}

// function which converts a parsed LaTeX group from the AST into a string representing the
// value of the command's macro
fn get_group_value(group: &Group) -> String {
    let mut value: String = String::new();
    for token in group.tokens.iter() {
        match token {
            // push the token's content into the value string
            // regardless of type
            Token::CharTokens(char_tokens) => {
                value.push_str(&char_tokens.content);
            }
            Token::Number(number) => {
                value.push_str(&number.content);
            }
            Token::Macro(macro_token) => {
                // if the Macro is a "textless", print "<" instead
                match macro_token.name.content.as_str() {
                    "textless" => {
                        value.push_str("<");
                    }
                    _ => {
                        value.push_str(&macro_token.name.content);
                    }
                }
            }
            _ => (),
        }
    }
    value
}

// gets the hashmap of all newcommand values in a file
pub fn get_newcommand_hashmap(tokens: &Vec<Token>) -> HashMap<String, String> {
    // get all newcommand macros in the file and their 2 most adjacent tokens
    let newcommand_tokens = get_n_adjacent_tokens_to_macro(&tokens, "newcommand", 2);

    // create hashmap from the content of the first token in each newcommand tokens pair
    // to the second token in the pair, after converting the second token, which is a
    // Group, to a string
    let mut newcommand_hashmap: HashMap<String, String> = HashMap::new();
    for (i, token) in newcommand_tokens.iter().enumerate() {
        // match the every other token pair
        if i % 2 == 0 {
            // the first item is a Macro with a MacroName, extract the content from the MacroName
            let name = match token {
                Token::Macro(macro_token) => macro_token.name.content.clone(),
                _ => panic!("expected Token::Macro"),
            };
            let value = match &newcommand_tokens[i + 1] {
                Token::Group(group) => get_group_value(&group),
                _ => panic!("expected Token::Group"),
            };
            newcommand_hashmap.insert(name, value);
        }
    }
    newcommand_hashmap
}

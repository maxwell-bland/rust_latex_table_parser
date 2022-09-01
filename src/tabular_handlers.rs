use tex_parser::ast::*;

/*
 * Rcursively parse the AST and return a hashmap of Environments with names 
 * matching the givent matc string
 */
pub fn get_environments(tokens: &Vec<Token>, matc: &str) -> Vec<Environment> {
    let mut environments: Vec<Environment> = Vec::new();
    for token in tokens.iter() {
        match token {
            Token::SpecialMacro(special_macro) => {
                match special_macro {
                    SpecialMacro::Environment(environment) => {
                        if environment.name.content == matc {
                            environments.push(environment.clone());
                        }
                    }
                    _ => (),
                }
            }
            _ => (),

        }
    }

    environments
}

// function that parses out the row data from a tabular environemnt by splitting out each line
// and columns
pub fn parse_tabular(tabular_tokens: &Vec<Token>) -> Vec<Vec<String>> {
    // split tabular by MacroName { content: "\\" }
    let mut rows: Vec<Vec<String>> = Vec::new();
    let mut row: Vec<String> = Vec::new();
    let mut col: String = String::new();

    for token in tabular_tokens.iter() {
        match token {
            Token::Macro(macro_token) => match macro_token.name.content.as_str() {
                "\\" => {
                    row.push(col);
                    rows.push(row);
                    row = Vec::new();
                    col = String::new();
                }
                _ => (col.push_str(&macro_token.name.content)),
            },
            Token::AlignmentTab(_) => {
                row.push(col);
                col = String::new();
            }
            Token::CharTokens(char_tokens) => {
                col.push_str(&char_tokens.content);
            }
            Token::Group(group) => {
                let mut macro_name = String::new();
                for token in group.tokens.iter() {
                    match token {
                        Token::Macro(macro_token) => {
                            macro_name.push_str(&macro_token.name.content);
                        },
                        Token::CharTokens(char_tokens) => {
                            macro_name.push_str(&char_tokens.content);
                        },
                        _ => (),
                    }
                }
                // if the column is not empty, push it to the row
                if !col.is_empty() {
                    row.push(col);
                    col = String::new();
                }
                col.push_str(&macro_name);
            },
            _ => (),
        }
    }

    rows.push(row.clone());
    rows
}

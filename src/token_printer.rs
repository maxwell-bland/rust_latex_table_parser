use tex_parser::ast::*;

/*
 Token variants:
     SpecialMacro(SpecialMacro)
     Macro(Macro)
     FullComment(FullComment)
     Group(Group)
     DollarInlineMath(DollarInlineMath)
     AlignmentTab(AlignmentTab)
     ParBreak(ParBreak)
     MacroParameter(MacroParameter)
     Ignore(Ignore)
     Number(Number)
     Whitespace(Whitespace)
     Punctuation(Punctuation)
     CharTokens(CharTokens)
     BeginGroup(BeginGroup)
     EndGroup(EndGroup)
     MathShift(MathShift)

 SpecialMacro Variants:
     Verb(Verb)
     VerbatimEnvironment(VerbatimEnvironment)
     DisplayMath(DisplayMath)
     ParenthesizedInlineMath(ParenthesizedInlineMath)
     MathEnvironment(MathEnvironment)
     Environment(Environment)

Function to recursively match each token variant and print it
*/
pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        match token {
            Token::SpecialMacro(special_macro) => {
                match special_macro {
                    SpecialMacro::Verb(verb) => {
                        println!("{:?}", verb);
                    }
                    SpecialMacro::VerbatimEnvironment(verbatim_environment) => {
                        println!("{:?}", verbatim_environment);
                    }
                    SpecialMacro::DisplayMath(display_math) => {
                        println!("{:?}", display_math);
                    }
                    SpecialMacro::ParenthesizedInlineMath(parenthesized_inline_math) => {
                        println!("{:?}", parenthesized_inline_math);
                    }
                    SpecialMacro::MathEnvironment(math_environment) => {
                        println!("{:?}", math_environment);
                    }
                    SpecialMacro::Environment(environment) => {
                        println!("{:?}", environment);
                    }
                }
            }
            Token::Macro(macro_) => {
                println!("Macro: {:?}", macro_);
            }
            Token::FullComment(full_comment) => {
                println!("FullComment: {:?}", full_comment);
            }
            Token::Group(group) => {
                println!("Group: {:?}", group);
            }
            Token::DollarInlineMath(dollar_inline_math) => {
                println!("DollarInlineMath: {:?}", dollar_inline_math);
            }
            Token::AlignmentTab(alignment_tab) => {
                println!("AlignmentTab: {:?}", alignment_tab);
            }
            Token::ParBreak(par_break) => {
                println!("ParBreak: {:?}", par_break);
            }
            Token::MacroParameter(macro_parameter) => {
                println!("MacroParameter: {:?}", macro_parameter);
            }
            Token::Ignore(ignore) => {
                println!("Ignore: {:?}", ignore);
            }
            Token::Number(number) => {
                println!("Number: {:?}", number);
            }
            Token::Whitespace(whitespace) => {
                println!("Whitespace: {:?}", whitespace);
            }
            Token::Punctuation(punctuation) => {
                println!("Punctuation: {:?}", punctuation);
            }
            Token::CharTokens(char_tokens) => {
                println!("CharTokens: {:?}", char_tokens);
            }
            Token::BeginGroup(begin_group) => {
                println!("BeginGroup: {:?}", begin_group);
            }
            Token::EndGroup(end_group) => {
                println!("EndGroup: {:?}", end_group);
            }
            Token::MathShift(math_shift) => {
                println!("MathShift: {:?}", math_shift);
            }
        }
    }
}

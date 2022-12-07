/*  Mark Clemmer
    10/12/2022
    Square and Circle Lexical and Syntax Analysis
*/

use std::env;
use std::fs::File;
use std::io::prelude::*;

// For the Lexer, I must go through the input file and create the tokens and compare them to the grammar to see if they match
// I need a compare method with all of the cases for the grammar and I will check the tokens with it


/* STEPS:
    1. Read the file character by character
    2. Create a function to check if the characters match any of the tokens. If they don't match, the lexer analysis should fail or panic
 */

// This is the enum with the Token types
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Token {
    ID,
    NUM,
    SEMICOLON,
    COLON,
    COMMA,
    PERIOD,
    LPAREN,
    RPAREN,
    ASSIGN,
    DEFINITIONS,
    OPERATIONS,
    POINT,
    CIRCLE,
    SQUARE,
    PRINT,
    CONTAINED,
    INTERSECTS,
    END,
}

// This is the struct that holds the values of the tokens and lexemes
struct LexemeToken {
    // Have the tokens and the lexemes in here
    token_type: Token,
    lexeme: String
}

// This is the struct that holds the values of the id variable, definition type, left value, and right value
struct SymbolValue {
    variable: String,
    def_type: String,
    left_val: String,
    right_val: String
}

/*  This function is used to check if the given character is a letter between a and z. It checks to see if
    the character is uppercase or lowercase. It returns true if it is uppercase, and false if it is not. */
fn check_letter(val: char) -> bool {
    if val >= 'a' && val <= 'z' {
        return true;
    } else if val >= 'A' && val <= 'Z'{
        // There should not be any uppercase letters
        return true;
    } else {
        return false;
    }
}

/*  This function checks to see if the given character is a digit between 0 and 9. It returns true if it is
    between 0 and 9 and false if it is not. */
fn check_digit(val: char) -> bool {
    if val >= '0' && val <= '9' {
        return true;
    } else {
        return false;
    }
}

/*  Brings in the input file String contents and then converts it to a chars to read character by character.
    This function will then make a vector of the different tokens. It returns a boolean value of true if
    the lexer succeeds and false if it has an error. The lexical error is printed.
    The vector is passed by reference to send the tokens back to the main method.
 */
fn lex(contents: String, tokens: &mut Vec<LexemeToken>) -> bool {
    // Go through the contents string character by character
        // Convert the contents of the file to a string of characters
        let mut file_data = contents.chars();

        // This string is used to make the lexemes if they are made of multiple characters
        let mut lex_builder = String::from("");
        let mut dig_builder = String::from("");

        // This initializes tok so that the unused variable worning is not generated
        // While loop through the content, character by character
        while let Some(i) = file_data.next() {
            // Space characters are brought into this, so I need to check to make sure the character is not a space before putting it into a token
            // Now the issue is figuring out how to make a token with the lexemes that have multiple characters, like definitions
            /*  1. If I have a letter, make a string and add the letters to it until there is a non-letter. When you reach this point, check
                   to see if the string matches any of the lexemes for the tokens, and if not, it is an ID. Then reset the string to be empty
                   Also check to see if there are any upper-case letters. If there are, this is a LEXICAL ERROR.

                2. If I have a number, check to see if there are more characters in that number and stop when you run into a non-digit.
                   Add the digits to a string to make a number token
                3. If the character is not a letter or a number, it should match one of the specific lexemes for the tokens
            */
            if check_letter(i) {
                if i.is_uppercase() {
                    println!("Lexical Error: {}", i);
                    return false;
                }
                lex_builder.push(i);
            } else {
                // check if the lex_builder string needs to be turned into a lexeme
                if !lex_builder.is_empty() {
                    if lex_builder == "definitions" {
                        let tok = LexemeToken {
                            token_type: Token::DEFINITIONS,
                            lexeme: lex_builder.clone(),
                        };
                        tokens.push(tok);
                        
                    } else if lex_builder == "operations" {
                        let tok = LexemeToken {
                            token_type: Token::OPERATIONS,
                            lexeme: lex_builder.clone(),
                        };
                        tokens.push(tok);
                        
                    } else if lex_builder == "point" {
                        let tok = LexemeToken {
                            token_type: Token::POINT,
                            lexeme: lex_builder.clone(),
                        };
                        tokens.push(tok);
                            
                    } else if lex_builder == "circle" {
                        let tok = LexemeToken {
                            token_type: Token::CIRCLE,
                            lexeme: lex_builder.clone(),
                        };
                        tokens.push(tok);
                    
                    } else if lex_builder == "square" {
                        let tok = LexemeToken {
                            token_type: Token::SQUARE,
                            lexeme: lex_builder.clone(),
                        };
                        tokens.push(tok);
                        
                    } else if lex_builder == "print" {
                        let tok = LexemeToken {
                            token_type: Token::PRINT,
                            lexeme: lex_builder.clone(),
                        };
                        tokens.push(tok);
                        
                    } else if lex_builder == "contained" {
                        let tok = LexemeToken {
                            token_type: Token::CONTAINED,
                            lexeme: lex_builder.clone(),
                        };
                        tokens.push(tok);
                        
                    } else if lex_builder == "intersects" {
                        let tok = LexemeToken {
                            token_type: Token::INTERSECTS,
                            lexeme: lex_builder.clone(),
                        };
                        tokens.push(tok);
                        
                    } else if lex_builder == "end" {
                        let tok = LexemeToken {
                            token_type: Token::END,
                            lexeme: lex_builder.clone(),
                        };
                        tokens.push(tok);
                    } else {
                        // The lexeme is an ID
                        let tok = LexemeToken {
                            token_type: Token::ID,
                            lexeme: lex_builder.clone(),
                        };
                        tokens.push(tok);
                    }

                    // Reset the value of the string
                    lex_builder.clear();
                }

                if check_digit(i) {
                    dig_builder.push(i);
                } else {
                    if dig_builder != "" {
                        let tok = LexemeToken {
                            token_type: Token::NUM,
                            lexeme: dig_builder.clone(),
                        };
                        tokens.push(tok);
                        dig_builder.clear();
                    }

                    if i == ';' {
                        let tok = LexemeToken {
                            token_type: Token::SEMICOLON,
                            lexeme: String::from(";"),
                        };
                        tokens.push(tok);
                    } else if i == ':' {
                        let tok = LexemeToken {
                            token_type: Token::COLON,
                            lexeme: String::from(":"),
                        };
                        tokens.push(tok);

                    } else if i == ',' {
                        let tok = LexemeToken {
                            token_type: Token::COMMA,
                            lexeme: String::from(","),
                        };
                        tokens.push(tok);
                    } else if i == '.' {
                        let tok = LexemeToken {
                            token_type: Token::PERIOD,
                            lexeme: String::from("."),
                        };
                        tokens.push(tok);
                    } else if i == '(' {
                        let tok = LexemeToken {
                            token_type: Token::LPAREN,
                            lexeme: String::from("("),
                        };
                        tokens.push(tok);
                    } else if i == ')' {
                        let tok = LexemeToken {
                            token_type:Token::RPAREN,
                            lexeme: String::from(")"),
                        };
                        tokens.push(tok);
                    } else if i == '=' {
                        let tok = LexemeToken {
                            token_type: Token::ASSIGN,
                            lexeme: String::from("="),
                        };
                        tokens.push(tok);
                    } else if i.is_whitespace() {
                        // Spaces can exist, but I don't need to do anything with it
                    } else if i == '\n' {
                        // newlines can exist, but I don't need to do anything with them
                    } else if i == '\t' {
                    } else {
                        // The lexeme doesn't match any of the tokens, so there is an error
                        println!("Lexical Error: {}", i);
                        return false;
                    }
                }
            }
        }

        return true;
}

/*  This is the OPERATION part of the grammar that checks to see if the operation follows the format
    for print, contained, or intersects. */
fn parse_operation(tokens: &Vec<LexemeToken>, count: &mut usize, pass: &mut bool) {
    if matches!(tokens[*count].token_type, Token::PRINT) {
        *count = *count + 1;
        if matches!(tokens[*count].token_type, Token::LPAREN) {
            *count = *count + 1;
            if matches!(tokens[*count].token_type, Token::ID) {
                *count = *count + 1;
                if matches!(tokens[*count].token_type, Token::RPAREN) {
                    *count = *count + 1;
                    
                } else {
                    println!("Syntax Error: right parenthesis expected");
                    *pass = false;
                    return;
                }
            } else {
                println!("Syntax Error: id expected");
                *pass = false;
                return;
            }
        } else {
            println!("Syntax Error: left parenthesis expected");
            *pass = false;
            return;
        }
        
    } else if matches!(tokens[*count].token_type, Token::CONTAINED) || matches!(tokens[*count].token_type, Token::INTERSECTS) {
        *count = *count + 1;
        if matches!(tokens[*count].token_type, Token::LPAREN) {
            *count = *count + 1;
            if matches!(tokens[*count].token_type, Token::ID) {
                *count = *count + 1;
                if matches!(tokens[*count].token_type, Token::COMMA) {
                    *count = *count + 1;
                    if matches!(tokens[*count].token_type, Token::ID) {
                        *count = *count + 1;
                        if matches!(tokens[*count].token_type, Token::RPAREN) {
                            *count = *count + 1;
                            
                        } else {
                            println!("Syntax Error: right parenthesis expected");
                            *pass = false;
                            return;
                        }
                    } else {
                        println!("Syntax Error: id expected");
                        *pass = false;
                        return;
                    }
                } else {
                    println!("Syntax Error: comma expected");
                    *pass = false;
                    return;
                }
            } else {
                println!("Syntax Error: id expected");
                *pass = false;
                return;
            }
        } else {
            println!("Syntax Error: left parenthesis expected");
            *pass = false;
            return;
        }
    } else {
        println!("Syntax Error: print, intersects or contained expected");
        *pass = false;
        return;
    }
}

/*  This is the OPERATIONS part of the grammar that calls parse_operation and calls pars_operations recursively
    if there is another operation to perform */
fn parse_operations(tokens: &Vec<LexemeToken>, count: &mut usize, pass: &mut bool) {
    if *pass == false {
        // There is an error
        return;
    }

    parse_operation(&tokens, count, pass);

    if matches!(tokens[*count].token_type, Token::SEMICOLON) {
        *count = *count + 1;
        parse_operations(&tokens, count, pass);
    } else {
        if matches!(tokens[*count].token_type, Token::PRINT) || matches!(tokens[*count].token_type, Token::CONTAINED)
        || matches!(tokens[*count].token_type, Token::INTERSECTS){
            println!("Syntax Error: semicolon expected");
            *pass = false;
            return;
        }
    }    
}

/* This is the DEF part of the grammar. It checks to see if the tokens follow the pattern of: ID = point()/circle()/square() */
fn parse_def(tokens: &Vec<LexemeToken>, count: &mut usize, pass: &mut bool) {
    if matches!(tokens[*count].token_type, Token::ID) {
        *count = *count + 1;
        if matches!(tokens[*count].token_type, Token::ASSIGN) {
            *count = *count + 1;
            if matches!(tokens[*count].token_type, Token::POINT) || matches!(tokens[*count].token_type, Token::CIRCLE)
             || matches!(tokens[*count].token_type, Token::SQUARE) {
                let definition_type = &tokens[*count].token_type;
                *count = *count + 1;
                if matches!(tokens[*count].token_type, Token::LPAREN) {
                    *count = *count + 1;

                    // Check if point, circle, or square was entered to check for the correct syntax
                    if matches!(definition_type, Token::POINT) {
                        if matches!(tokens[*count].token_type, Token::NUM) {
                            *count = *count + 1;
                            if matches!(tokens[*count].token_type, Token::COMMA) {
                                *count = *count + 1;
    
                                if matches!(tokens[*count].token_type, Token::NUM) {
                                    *count = *count + 1;
                                    if matches!(tokens[*count].token_type, Token::RPAREN) {
                                        *count = *count + 1;
                                    } else {
                                        println!("Syntax Error: right parenthesis expected");
                                        *pass = false;
                                        return;
                                    }
                                } else {
                                    println!("Syntax Error: number expected");
                                    *pass = false;
                                    return;
                                }
                            } else {
                                println!("Syntax Error: comma expected");
                                *pass = false;
                                return;
                            }
                        } else {
                            println!("Syntax Error: number expected");
                            *pass = false;
                            return;
                        }
                        
                    } else if matches!(definition_type, Token::CIRCLE) || matches!(definition_type, Token::SQUARE) {
                        if matches!(tokens[*count].token_type, Token::ID) {
                            *count = *count + 1;
                            if matches!(tokens[*count].token_type, Token::COMMA) {
                                *count = *count + 1;
    
                                if matches!(tokens[*count].token_type, Token::NUM) {
                                    *count = *count + 1;
                                    if matches!(tokens[*count].token_type, Token::RPAREN) {
                                        *count = *count + 1;
                                        return;
                                        
                                    } else {
                                        println!("Syntax Error: right parenthesis expected");
                                        *pass = false;
                                    }
                                } else {
                                    println!("Syntax Error: number expected");
                                    *pass = false;
                                    return;
                                }
                            } else {
                                println!("Syntax Error: comma expected");
                                *pass = false;
                                return;
                            }
                        } else {
                            println!("Syntax Error: id expected");
                            *pass = false;
                            return;
                        }
                    }
                } else {
                    println!("Syntax Error: left parenthesis expected");
                    *pass = false;
                    return;
                }
            } else {
                println!("Syntax Error: point, circle, or square expected");
                *pass = false;
                return;
            }
        } else {
            println!("Syntax Error: assignment operator(=) expected");
            *pass = false;
            return;
        }

    } else {
        println!("Syntax Error: ID expected");
        *pass = false;
        return;
    }

}

/* This is the DEFS part of the grammar that calls def once and if there is a semicolon it calls defs */
fn parse_defs(tokens: &Vec<LexemeToken>, count: &mut usize, pass: &mut bool) {
    if *pass == false {
        // There is an error
        return;
    }

    parse_def(&tokens, count, pass);

    if matches!(tokens[*count].token_type, Token::SEMICOLON) {
        *count = *count + 1;
        parse_defs(&tokens, count, pass);
    } else {
        if matches!(tokens[*count].token_type, Token::ID) {
            println!("Syntax Error: semicolon expected");
            *pass = false;
            return;
        }
    }
}

/*  This is the function that runs the syntax analyzer
    It checks to see if the given grammar is followed. If it is not, a Syntax Error is printed
    and the function returns false. If the syntax analyzer succeeds, it returns true. */
fn syntax(tokens: &Vec<LexemeToken>) -> bool {
    // This is the counter to go through the token vector
    let mut count: usize = 2;
    let mut pass: bool = true;
    // Check if the first token is definitions
    if matches!(tokens[0].token_type, Token::DEFINITIONS) {
        if matches!(tokens[1].token_type, Token::COLON) {
            // call parse_defs and keep calling it until the token is operations
            /* I should pass a boolean value in to check if the syntax analyzer passes and maybe a vector
                value to output what the error is for. Or I have the counter passed and return when there is an error
                and print the value of the token is that had the error when I return it or something
            */
            // Use recursion for all of the uppercase nonterminals and I need to pass the counter into the all of the functions to keep incrementing them
            // I shouldn't have to check for the operations token to go back to the syntax analyzer. I should recursively go back out
            parse_defs(&tokens, &mut count, &mut pass);
            // Now the next token should be operations, then the colon
            if matches!(tokens[count].token_type, Token::OPERATIONS) {
                count = count + 1;
                if matches!(tokens[count].token_type, Token::COLON) {
                    count = count + 1;
                    parse_operations(&tokens, &mut count, &mut pass)
                } else {
                    println!("Syntax Error: colon expected");
                     pass = false;
                }

            } else {
                println!("Syntax Error: operations expected");
                pass = false;
            }
        } else {
            println!("Syntax Error: colon expected");
            pass = false;
            //return;
        }
    } else {
        println!("Syntax Error: definitions expected");
        pass = false;
    }


    return pass;
}


/* This function finds the index of the id in the symbol table and returns it */
fn find_id(table: &Vec<SymbolValue>, id: &String) -> usize {
    let mut count: usize = 0;
    for i in table.iter() {
        if i.variable == *id {
            return count;
        }
        count = count + 1;
    }
    // The id was not found and an index out of bounds will be returned
    return count;
}

/*  This function goes through the defintions and pushes them into the output vector with the correct
    scheme format. It returns true if there is no error and false if there is a user error when the
    id that was entered was not defined. */
fn scheme_def(table: &Vec<SymbolValue>, id_index: &mut usize, output: &mut Vec<String>) -> bool{
    if table[*id_index].def_type == "point" {
        // Print the left and right value
        output.push(" (makepoint ".to_string());
        output.push(table[*id_index].left_val.clone());
        output.push(" ".to_string());
        output.push(table[*id_index].right_val.clone());
        output.push(") ".to_string());
    } else {
        // This is for circle or square
        // The left value is an id, so check to see if there is a matching id in the symbol table
        let mut new_index: usize = find_id(&table, &table[*id_index].left_val);
        if new_index >= table.len() {
            println!("User Error: The given id cannot be found");
            return false;
        }
    
        scheme_def(&table, &mut new_index, output);

        // After going through the new id, print the right value at id_index
        output.push(table[*id_index].right_val.clone());
    }
    return true;
}

/*  This function creates the scheme output using a vector and then prints it. It brings in the tokens vector
    and the symbol table vector to reference, and the counter, which is set to the first operation.
    Go through the vector until I reach the OPERATIONS and COLON tokens, then read the operations that must
    be done until end is reached.
    Print the operation lexeme, then find the value of the id in the symbol_table, and print the output for
    the definition type
*/
fn print_scheme(tokens: &Vec<LexemeToken>, table: &Vec<SymbolValue>, counter: &usize) {
    let mut output: Vec<String> = vec![];
    let mut count = *counter;

    while tokens[count].lexeme != "." {
        output.push("(".to_string());
        output.push(tokens[count].lexeme.clone());
        // I need to have a case for print and contained and intersects
        if matches!(tokens[count].token_type, Token::PRINT) {
            count = count + 2;   //This moves the count to be the index for the value of the id
            
            let mut id_index: usize = find_id(&table, &tokens[count].lexeme);
            if id_index >= table.len() {
                println!("User Error: The given id cannot be found");
                return;
            }
            // Print the definition type(circle or square) then find the id in the symbol table for the circle or square
            output.push("-".to_string());
            output.push(table[id_index].def_type.clone());
    
            let check: bool = scheme_def(&table, &mut id_index, &mut output);
            if !check {
                // There is an error
                return;
            }
            output.push(")\n".to_string());
            
        } else if matches!(tokens[count].token_type, Token::CONTAINED) || matches!(tokens[count].token_type, Token::INTERSECTS) {
            count = count + 2;   //This moves the count to be the index for the value of the id
            let mut id_index1: usize = find_id(&table, &tokens[count].lexeme);
            if id_index1 >= table.len() {
                println!("User Error: The given id cannot be found");
                return;
            }
            output.push("-".to_string());
            output.push(table[id_index1].def_type.clone());

            
    
            count = count + 2;
            let mut id_index2 = find_id(&table, &tokens[count].lexeme);
            if id_index2 >= table.len() {
                println!("User Error: The given id cannot be found");
                return;
            }
            output.push("-".to_string());
            output.push(table[id_index2].def_type.clone());

            let check: bool = scheme_def(&table, &mut id_index1, &mut output);
            if !check {
                // There is an error
                return;
            }
            let check: bool = scheme_def(&table, &mut id_index2, &mut output);
            if !check {
                // There is an error
                return;
            }
            output.push(")\n".to_string());
        }
        count = count + 3;
    }
    

    // Now print the output
    for i in output.iter() {
        print!("{}", i);
    }
    
}

/*  This function goes through the defintions and pushes them into the output vector with the correct
    prolog format. It returns true if there is no error and false if there is a user error when the
    id that was entered was not defined. */
fn prolog_def(table: &Vec<SymbolValue>, id_index: &mut usize, output: &mut Vec<String>) -> bool {
    if table[*id_index].def_type == "point" {
        // Print the left and right value
        output.push("(point2d(".to_string());
        output.push(table[*id_index].left_val.clone());
        output.push(",".to_string());
        output.push(table[*id_index].right_val.clone());
        output.push("), ".to_string());
    } else {
        // This is for circle or square
        // The left value is an id, so check to see if there is a matching id in the symbol table
        let mut new_index: usize = find_id(&table, &table[*id_index].left_val);
        if new_index >= table.len() {
            println!("User Error: The given id cannot be found");
            return false;
        }

        prolog_def(&table, &mut new_index, output);

        // After going through the new id, print the right value at id_index
        output.push(table[*id_index].right_val.clone());
    }
    return true;
}

/*  This function creates the prolog output using a vector and then prints it. It brings in the tokens vector
    and the symbol table vector to reference, and the counter, which is set to the first operation.
    Go through the vector until I reach the OPERATIONS and COLON tokens, then read the operations that must
    be done until end is reached.
    Print the operation lexeme, then find the value of the id in the symbol_table, and print the output for
    the definition type
*/
fn print_prolog(tokens: &Vec<LexemeToken>, table: &Vec<SymbolValue>, counter: &usize) {
    let mut output: Vec<String> = vec![];
    let mut count = *counter;

    while tokens[count].lexeme != "." {
        output.push("query(".to_string());
        // I need to have a case for print and contained and intersects
        if matches!(tokens[count].token_type, Token::PRINT) {
            count = count + 2;   //This moves the count to be the index for the value of the id
            
            let mut id_index: usize = find_id(&table, &tokens[count].lexeme);
            if id_index >= table.len() {
                println!("User Error: The given id cannot be found");
                return;
            }
            // Print the definition type(circle or square) then find the id in the symbol table for the circle or square
            
            output.push(table[id_index].def_type.clone());
    
            let check: bool = prolog_def(&table, &mut id_index, &mut output);
            if !check {
                // There is an error
                return;
            }
            output.push(")).\n".to_string());
            
        } else if matches!(tokens[count].token_type, Token::CONTAINED) || matches!(tokens[count].token_type, Token::INTERSECTS) {
            output.push(tokens[count].lexeme.clone());

            count = count + 2;   //This moves the count to be the index for the value of the id
            let mut id_index1: usize = find_id(&table, &tokens[count].lexeme);
            if id_index1 >= table.len() {
                println!("User Error: The given id cannot be found");
                return;
            }
            output.push("(".to_string());
            output.push(table[id_index1].def_type.clone());

            
    
            count = count + 2;
            let mut id_index2 = find_id(&table, &tokens[count].lexeme);
            if id_index2 >= table.len() {
                println!("User Error: The given id cannot be found");
                return;
            }

            prolog_def(&table, &mut id_index1, &mut output);
            output.push("), ".to_string());
            
            output.push(table[id_index2].def_type.clone());
            prolog_def(&table, &mut id_index2, &mut output);
            output.push("))).\n".to_string());
        }
        count = count + 3;
    }
    

    // Now print the output
    for i in output.iter() {
        print!("{}", i);
    }
    println!("writeln(T) :- write(T), nl.");
    println!("main:- forall(query(Q), Q-> (writeln(‘yes’)) ; (writeln(‘no’))),\n      halt.")
}

/* This is the main function for this file that runs all of the functions */
fn main() {
    let args: Vec<String> = env::args().collect();
    // File input
    if args.len() == 3 {
        let infile: &String = &args[1];
        if !infile.ends_with(".sc") {
            println!("Please enter a valid file");
            return;
        }

        if &args[2] != "-s" && &args[2] != "-p" {
            println!("You have entered an invalid command. Please enter -s or -p.");
            return;
        }
        
        let mut file = File::open(infile).expect("Can't open file!");
        
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read from the file");
        let mut tokens: Vec<LexemeToken> = vec![];
        
        // This will add tokens to the tokens vector and return false if there is a lexical error
        let lex_pass: bool = lex(contents, &mut tokens);
        
        if !lex_pass {
            // End the program since the lexical analysis failed
            return;
        }

        // Syntax Analyzer:
        let syntax_pass: bool = syntax(&tokens);
        if !syntax_pass {
            // End the program because the syntax analysis failed
            return;
        }

        // Make a symbol table to store the definitions to reference 
        let mut counter = 2;
        let mut symbol_table: Vec<SymbolValue> = vec![];
        // This checks if it is the colon becuase that indicates that the definitions are over and the colon after operations has been reached
        while !matches!(tokens[counter].token_type, Token::COLON) {
            // Check to make sure that only the definitions are taken. They are between definitions: and operations
            let def = SymbolValue {
                variable: tokens[counter].lexeme.clone(),
                def_type: tokens[counter+2].lexeme.clone(),
                left_val: tokens[counter+4].lexeme.clone(),
                right_val: tokens[counter+6].lexeme.clone(),
            };
            
            // Now store def in the symbol_table
            symbol_table.push(def);
            counter = counter + 9;
        }

        // Increase the counter to be at the first operation
        counter = counter + 1;

        if &args[2] == "-s" {
            println!("; processing input file {infile} \n; Lexical and Syntax analysis passed");
            
            // Now write the Scheme output
            print_scheme(&tokens, &symbol_table, &counter);
        } else if &args[2] == "-p" {
            println!("/* processing input file {infile} \n   Lexical and Syntax analysis passed */");

            // Now write the Prolog output
            print_prolog(&tokens, &symbol_table, &counter);
        }
    } else {
        println!("You have entered too many arguments.");
    }
    
}

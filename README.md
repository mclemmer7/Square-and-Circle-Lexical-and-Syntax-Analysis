# Rust Programming Assignment: Square and Circle Lexical and Syntax Analysis


## Description
This program uses the command line to run it by entering an input file and -s to generate scheme output or -p to generate prolog output. The lexer reads an input file character by character to generate lexemes, which match up with the given tokens. All of the token and lexeme pairs are put into a vector to be used by the syntax analyzer. The syntax analyzer goes through this vector to see if the given grammar is followed. Any lexical or syntax errors will be caught, and an error message will be returned saying what the error is. If both the lexical and syntax analysis pass, then the program outputs what the code from the input file would look like in either scheme or prolog.

Test files are provided to test the program out. `test4.sc` has a lexical error. `test2.sc` and `test5.sc` have syntax errors. I wrote all of the code in the main file, which is in the src folder. 


## Grammar

```
PROGRAM     -->   definitions: 
                     DEFS
                  operations:
                     OPERATIONS
                  end.
DEFS        -->   DEF | DEF; DEFS
DEF         -->   ID = point(NUM, NUM) |
                  ID = circle(ID, NUM) |
                  ID = square(ID, NUM)
OPERATIONS  -->   OPERATION | OPERATION; OPERATIONS
OPERATION   -->   print(ID) |
                  contained(ID, ID) |
                  intersects(ID, ID)
ID          -->   LETTER+
NUM         -->   DIGIT+
LETTER      -->   a | b | c | d | e | f | g | ... | z
NUM         -->   0 | 1 | 2 | 3 | 4 | 5 | 6 | ... | 9
```

The tokens of this grammar are (some lexemes are examples):
| Token | Lexeme |
| ----- | ------ |
| `ID` | `alpha` |
| `NUM` |  `256` |
| `SEMICOLON` | `;` |
| `COLON` | `:` |
| `COMMA` | `,` |
| `PERIOD` | `.` |
| `LPAREN` | `(` |
| `RPAREN` | `)` |
| `ASSIGN` | `=` |
| `DEFINITIONS` | `definitions` |
| `OPERATIONS` | `operations` |
| `POINT` | `point` |
| `CIRCLE` | `circle` |
| `SQUARE` | `square` |
| `PRINT` | `print` |
| `CONTAINED` | `contained` |
| `INTERSECTS` | `intersects` |
| `END` | `end` |



## How to run the program

### Scheme Output
To generate scheme output you will add the `-s` flag at the end of the command:
```
prompt> cargo run input.sc -s
```
### Prolog Output
To generate prolog output you will add the `-p` flag at the end of the command:
```
prompt> cargo run input.sc -p
```

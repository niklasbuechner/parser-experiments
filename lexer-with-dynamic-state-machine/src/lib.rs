fn create_lexer() {
    let code = "if hello then hi else bye 'if else' how";

    // Define tokens
    // %TOKENS%{
    //     IF,
    //     THEN,
    //     ELSE,
    //     ID(&str),
    // }

    // Define states
    // %STATES%{
    //     INITIAL,
    //     QUOTATION,
    // }

    // Allow additional logic & functions
    // - current character
    // - current line
    // - functions to use

    // Macros in regex
    // LNUM	[0-9]+
    // DNUM	([0-9]*"."[0-9]+)|([0-9]+"."[0-9]*)
    // EXPONENT_DNUM (({LNUM}|{DNUM})[eE][+-]?{LNUM})

    // Regex with code if the lexer detects a match
    // <INITIAL>"if" {
    //     self.return_token(TOKEN::IF);
    // }

    // Change states
    // <INITIAL>"'" {
    //     self.push_state(STATE::QUOTATION);
    // }

    // Symbol table - add manually?
    // %SYMBOL%{
    //     pub name: String,
    // }

    // Run 🚀
}

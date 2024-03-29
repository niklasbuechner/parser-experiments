mod first;
mod follow;
mod parser_table;
mod slr_closure;
mod slr_goto;

use super::ast;
use super::parse_string;
use super::Ast;
use super::FirstElements;
use super::Grammar;
use super::GrammarSymbol;
use super::Production;
use super::Reaction;
use super::Reduction;
use super::SlrClosure;
use super::SlrGoto;
use super::Token;

#[test]
fn table_lengths() {
    let grammar = Grammar::new();

    assert_eq!(12, grammar.get_goto_table().len());
    assert_eq!(12, grammar.get_slr_table().len());
}

#[test]
fn parse_multiplication() {
    let grammar = Grammar::new();
    let token_stream = vec![
        Token::new("Id"),
        Token::new("Times"),
        Token::new("Id"),
        Token::end_token("EndSymbol"),
    ];
    let (real_output, ast) = parse_string(&token_stream, grammar);

    let expected_output = "Shift to 5
Reduce to F and pop stack by 1
Go to 3
Reduce to T and pop stack by 1
Go to 2
Shift to 7
Shift to 5
Reduce to F and pop stack by 1
Go to 10
Reduce to T and pop stack by 3
Go to 2
Reduce to E and pop stack by 1
Go to 1
Accept
";

    assert_eq!(expected_output, real_output);

    let expected_ast = vec![Ast::Multiply(
        Box::new(Ast::Number(0)),
        Box::new(Ast::Number(0)),
    )];

    assert_eq!(expected_ast, ast);
}

#[test]
fn parse_addition() {
    let grammar = Grammar::new();
    let token_stream = vec![
        Token::new("Id"),
        Token::new("Plus"),
        Token::new("Id"),
        Token::end_token("EndSymbol"),
    ];
    let expected_ast = vec![Ast::Add(Box::new(Ast::Number(0)), Box::new(Ast::Number(0)))];

    let (_, ast) = parse_string(&token_stream, grammar);

    assert_eq!(expected_ast, ast);
}

#[test]
fn parse_addition_and_multiplication() {
    let grammar = Grammar::new();
    let token_stream = vec![
        Token::new("Id"),
        Token::new("Plus"),
        Token::new("Id"),
        Token::new("Times"),
        Token::new("Id"),
        Token::end_token("EndSymbol"),
    ];
    let expected_ast = vec![Ast::Add(
        Box::new(Ast::Multiply(
            Box::new(Ast::Number(0)),
            Box::new(Ast::Number(0)),
        )),
        Box::new(Ast::Number(0)),
    )];

    let (_, ast) = parse_string(&token_stream, grammar);

    assert_eq!(expected_ast, ast);
}

#[test]
fn parse_brackets() {
    let grammar = Grammar::new();
    let token_stream = vec![
        Token::new("OpeningBracket"),
        Token::new("Id"),
        Token::new("ClosingBracket"),
        Token::end_token("EndSymbol"),
    ];
    let expected_ast = vec![Ast::Calculation(Box::new(Ast::Number(0)))];

    let (_, ast) = parse_string(&token_stream, grammar);

    assert_eq!(expected_ast, ast);
}

#[test]
fn parse_brackets_with_addition_and_multiplication() {
    let grammar = Grammar::new();
    let token_stream = vec![
        Token::new("OpeningBracket"),
        Token::new("Id"),
        Token::new("Plus"),
        Token::new("Id"),
        Token::new("ClosingBracket"),
        Token::new("Times"),
        Token::new("Id"),
        Token::end_token("EndSymbol"),
    ];
    let expected_ast = vec![Ast::Multiply(
        Box::new(Ast::Number(0)),
        Box::new(Ast::Calculation(Box::new(Ast::Add(
            Box::new(Ast::Number(0)),
            Box::new(Ast::Number(0)),
        )))),
    )];

    let (_, ast) = parse_string(&token_stream, grammar);

    assert_eq!(expected_ast, ast);
}

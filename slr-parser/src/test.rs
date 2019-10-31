use super::parse_string;
use super::Ast;
use super::Grammar;
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
    let token_stream = vec![Token::Id, Token::Times, Token::Id, Token::EndSymbol];
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
    let token_stream = vec![Token::Id, Token::Plus, Token::Id, Token::EndSymbol];
    let expected_ast = vec![Ast::Add(Box::new(Ast::Number(0)), Box::new(Ast::Number(0)))];

    let (_, ast) = parse_string(&token_stream, grammar);

    assert_eq!(expected_ast, ast);
}

#[test]
fn parse_addition_and_multiplication() {
    let grammar = Grammar::new();
    let token_stream = vec![
        Token::Id,
        Token::Plus,
        Token::Id,
        Token::Times,
        Token::Id,
        Token::EndSymbol,
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
        Token::OpeningBracket,
        Token::Id,
        Token::ClosingBracket,
        Token::EndSymbol,
    ];
    let expected_ast = vec![Ast::Calculation(Box::new(Ast::Number(0)))];

    let (_, ast) = parse_string(&token_stream, grammar);

    assert_eq!(expected_ast, ast);
}

#[test]
fn parse_brackets_with_addition_and_multiplication() {
    let grammar = Grammar::new();
    let token_stream = vec![
        Token::OpeningBracket,
        Token::Id,
        Token::Plus,
        Token::Id,
        Token::ClosingBracket,
        Token::Times,
        Token::Id,
        Token::EndSymbol,
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

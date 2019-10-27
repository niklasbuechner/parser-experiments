use super::parse_string;
use super::Grammar;
use super::Token;

#[test]
fn parse_multiplication() {
    let grammar = Grammar::new();
    let token_stream = vec![Token::Id, Token::Times, Token::Id, Token::EndSymbol];
    let real_output = parse_string(&token_stream, grammar);

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
}

#[test]
fn table_lengths() {
    let grammar = Grammar::new();

    assert_eq!(12, grammar.get_goto_table().len());
    assert_eq!(12, grammar.get_slr_table().len());
}

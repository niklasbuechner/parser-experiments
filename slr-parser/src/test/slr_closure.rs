use super::Grammar;
use super::GrammarSymbol;
use super::Production;
use super::SlrClosure;
use super::Token;

#[test]
fn test_slr_closure() {
    let grammar = Grammar::new();
    let t_plus = Production::new(
        "T",
        vec![
            GrammarSymbol::non_terminal("T"),
            GrammarSymbol::terminal(Token::new("Times")),
            GrammarSymbol::non_terminal("F"),
        ],
        0,
        |_| {},
    );
    let t_f = Production::new("T", vec![GrammarSymbol::non_terminal("F")], 0, |_| {});
    let expected_closure = vec![
        t_plus.clone(),
        t_f.clone(),
        Production::new(
            "F",
            vec![
                GrammarSymbol::terminal(Token::new("OpeningBracket")),
                GrammarSymbol::non_terminal("E"),
                GrammarSymbol::terminal(Token::new("ClosingBracket")),
            ],
            0,
            |_| {},
        ),
        Production::new(
            "F",
            vec![GrammarSymbol::terminal(Token::new("Id"))],
            0,
            |_| {},
        ),
    ];

    let closure = SlrClosure::get_closure(&grammar, vec![t_plus, t_f]);

    assert_contents_only(expected_closure, closure);
}

fn assert_contents_only<T: std::fmt::Debug + PartialEq>(expected: Vec<T>, actual: Vec<T>) {
    println!("Expected: {:#?}", expected);
    println!("Actual: {:#?}", actual);

    assert_eq!(expected.len(), actual.len());
    expected.iter().for_each(|element| {
        if !actual.contains(element) {
            panic!("Element does not exist in actual Vec: {:#?}", element);
        }
    })
}

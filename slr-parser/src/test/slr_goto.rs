use super::GrammarSymbol;
use super::Production;
use super::SlrGoto;
use super::Token;

#[test]
fn test_get_goto_symbols_cursor_at_0() {
    let productions = vec![
        Production::new(
            "T",
            vec![
                GrammarSymbol::non_terminal("T"),
                GrammarSymbol::terminal(Token::new("Times")),
                GrammarSymbol::non_terminal("F"),
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
    let expected_symbols = vec![
        GrammarSymbol::non_terminal("T"),
        GrammarSymbol::terminal(Token::new("Id")),
    ];

    let goto_symbols = SlrGoto::get_goto_symbols(&productions);

    assert_contents_only(expected_symbols, goto_symbols);
}

#[test]
fn test_get_goto_symbols_end_of_production() {
    let productions = vec![Production::new(
        "F",
        vec![GrammarSymbol::terminal(Token::new("Id"))],
        1,
        |_| {},
    )];

    let goto_symbols = SlrGoto::get_goto_symbols(&productions);

    assert_contents_only(Vec::new(), goto_symbols);
}

#[test]
fn test_get_goto_symbols_middle_of_production() {
    let productions = vec![
        Production::new(
            "T",
            vec![
                GrammarSymbol::non_terminal("T"),
                GrammarSymbol::terminal(Token::new("Times")),
                GrammarSymbol::non_terminal("F"),
            ],
            1,
            |_| {},
        ),
        Production::new(
            "F",
            vec![
                GrammarSymbol::terminal(Token::new("OpeningBracket")),
                GrammarSymbol::non_terminal("E"),
                GrammarSymbol::terminal(Token::new("Closing")),
            ],
            1,
            |_| {},
        ),
    ];
    let expected_symbols = vec![
        GrammarSymbol::terminal(Token::new("Times")),
        GrammarSymbol::non_terminal("E"),
    ];

    let goto_symbols = SlrGoto::get_goto_symbols(&productions);

    assert_contents_only(expected_symbols, goto_symbols);
}

#[test]
fn test_get_goto() {
    let productions = vec![
        Production::new(
            "T",
            vec![
                GrammarSymbol::non_terminal("T"),
                GrammarSymbol::terminal(Token::new("Times")),
                GrammarSymbol::non_terminal("F"),
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
    let expected_productions = vec![Production::new(
        "T",
        vec![
            GrammarSymbol::non_terminal("T"),
            GrammarSymbol::terminal(Token::new("Times")),
            GrammarSymbol::non_terminal("F"),
        ],
        1,
        |_| {},
    )];

    let goto_productions = SlrGoto::get_goto(&GrammarSymbol::non_terminal("T"), &productions);

    assert_contents_only(expected_productions, goto_productions);
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

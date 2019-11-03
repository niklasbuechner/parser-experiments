use super::FirstElements;
use super::Grammar;
use super::GrammarSymbol;
use super::Token;

fn get_ll_grammar() -> Grammar {
    let mut grammar = Grammar::empty();
    grammar.set_starting_non_terminal("E");
    grammar.set_end_token("EndSymbol");

    grammar.add_production(
        "E",
        vec![
            GrammarSymbol::non_terminal("T"),
            GrammarSymbol::non_terminal("E'"),
        ],
    );

    grammar.add_production(
        "E'",
        vec![
            GrammarSymbol::terminal(Token::new("Plus")),
            GrammarSymbol::non_terminal("T"),
            GrammarSymbol::non_terminal("E'"),
        ],
    );
    grammar.add_production("E'", Vec::new());

    grammar.add_production(
        "T",
        vec![
            GrammarSymbol::non_terminal("F"),
            GrammarSymbol::non_terminal("T'"),
        ],
    );

    grammar.add_production(
        "T'",
        vec![
            GrammarSymbol::terminal(Token::new("Times")),
            GrammarSymbol::non_terminal("F"),
            GrammarSymbol::non_terminal("T'"),
        ],
    );
    grammar.add_production("T'", Vec::new());

    grammar.add_production(
        "F",
        vec![
            GrammarSymbol::terminal(Token::new("OpeningBracket")),
            GrammarSymbol::non_terminal("E"),
            GrammarSymbol::terminal(Token::new("ClosingBracket")),
        ],
    );
    grammar.add_production("F", vec![GrammarSymbol::terminal(Token::new("Id"))]);

    grammar
}

fn get_grammar_with_non_terminals_which_can_all_be_empty_sets() -> Grammar {
    let mut grammar = Grammar::empty();

    grammar.add_production(
        "A",
        vec![
            GrammarSymbol::non_terminal("B"),
            GrammarSymbol::non_terminal("C"),
        ],
    );

    grammar.add_production("B", Vec::new());
    grammar.add_production("C", Vec::new());

    grammar
}

#[test]
fn slr_first_f() {
    let grammar = Grammar::new();
    let expected_first = FirstElements {
        tokens: vec![Token::new("OpeningBracket"), Token::new("Id")],
        empty_set: false,
    };

    let first = grammar.get_first("F");

    assert_eq!(expected_first, first);
}

#[test]
fn slr_first_t() {
    let grammar = Grammar::new();
    let expected_first = FirstElements {
        tokens: vec![Token::new("OpeningBracket"), Token::new("Id")],
        empty_set: false,
    };

    let first = grammar.get_first("T");

    assert_eq!(expected_first, first);
}

#[test]
fn slr_first_e() {
    let grammar = Grammar::new();
    let expected_first = FirstElements {
        tokens: vec![Token::new("OpeningBracket"), Token::new("Id")],
        empty_set: false,
    };

    let first = grammar.get_first("E");

    assert_eq!(expected_first, first);
}

#[test]
fn slr_first_e_stroke() {
    let grammar = Grammar::new();
    let expected_first = FirstElements {
        tokens: vec![Token::new("OpeningBracket"), Token::new("Id")],
        empty_set: false,
    };

    let first = grammar.get_first("E'");

    assert_eq!(expected_first, first);
}

#[test]
fn ll_first_f() {
    let grammar = get_ll_grammar();
    let expected_first = FirstElements {
        tokens: vec![Token::new("OpeningBracket"), Token::new("Id")],
        empty_set: false,
    };

    let first = grammar.get_first("F");

    assert_eq!(expected_first, first);
}

#[test]
fn ll_first_t_stroke() {
    let grammar = get_ll_grammar();
    let expected_first = FirstElements {
        tokens: vec![Token::new("Times")],
        empty_set: true,
    };

    let first = grammar.get_first("T'");

    assert_eq!(expected_first, first);
}

#[test]
fn ll_first_t() {
    let grammar = get_ll_grammar();
    let expected_first = FirstElements {
        tokens: vec![Token::new("OpeningBracket"), Token::new("Id")],
        empty_set: false,
    };

    let first = grammar.get_first("T");

    assert_eq!(expected_first, first);
}

#[test]
fn ll_first_e_stroke() {
    let grammar = get_ll_grammar();
    let expected_first = FirstElements {
        tokens: vec![Token::new("Plus")],
        empty_set: true,
    };

    let first = grammar.get_first("E'");

    assert_eq!(expected_first, first);
}

#[test]
fn ll_first_e() {
    let grammar = get_ll_grammar();
    let expected_first = FirstElements {
        tokens: vec![Token::new("OpeningBracket"), Token::new("Id")],
        empty_set: false,
    };

    let first = grammar.get_first("E");

    assert_eq!(expected_first, first);
}

#[test]
fn test_grammar_with_non_terminals_which_can_all_be_empty_sets() {
    let grammar = get_grammar_with_non_terminals_which_can_all_be_empty_sets();
    let expected_first = FirstElements {
        tokens: Vec::new(),
        empty_set: true,
    };

    let first = grammar.get_first("A");

    assert_eq!(expected_first, first);
}

use super::ast;
use super::Grammar;
use super::Reaction;
use super::Reduction;
use super::Token;
use std::collections::HashMap;

#[test]
fn correct_action_table() {
    let grammar = Grammar::new();
    let expected_action_table = get_action_table();

    let action_table = grammar.get_slr_table();

    assert_eq!(expected_action_table, action_table);
}

#[test]
fn correct_goto_table() {
    let grammar = Grammar::new();
    let expected_goto_table = get_goto_table();

    let goto_table = grammar.get_goto_table();

    assert_eq!(expected_goto_table, goto_table);
}

fn get_action_table() -> Vec<HashMap<Token, Reaction>> {
    let mut state_0_tokens = HashMap::new();
    state_0_tokens.insert(Token::new("Id"), Reaction::ShiftTo(5));
    state_0_tokens.insert(Token::new("OpeningBracket"), Reaction::ShiftTo(4));

    let mut state_1_tokens = HashMap::new();
    state_1_tokens.insert(Token::new("Plus"), Reaction::ShiftTo(6));
    state_1_tokens.insert(Token::end_token("EndSymbol"), Reaction::Accept);

    let mut state_2_tokens = HashMap::new();
    state_2_tokens.insert(Token::new("Plus"), Reaction::Reduce(Reduction::new("E", 1)));
    state_2_tokens.insert(Token::new("Times"), Reaction::ShiftTo(7));
    state_2_tokens.insert(
        Token::new("ClosingBracket"),
        Reaction::Reduce(Reduction::new("E", 1)),
    );
    state_2_tokens.insert(
        Token::end_token("EndSymbol"),
        Reaction::Reduce(Reduction::new("E", 1)),
    );

    let mut state_3_tokens = HashMap::new();
    state_3_tokens.insert(Token::new("Plus"), Reaction::Reduce(Reduction::new("T", 1)));
    state_3_tokens.insert(
        Token::new("Times"),
        Reaction::Reduce(Reduction::new("T", 1)),
    );
    state_3_tokens.insert(
        Token::new("ClosingBracket"),
        Reaction::Reduce(Reduction::new("T", 1)),
    );
    state_3_tokens.insert(
        Token::end_token("EndSymbol"),
        Reaction::Reduce(Reduction::new("T", 1)),
    );

    let mut state_4_tokens = HashMap::new();
    state_4_tokens.insert(Token::new("Id"), Reaction::ShiftTo(5));
    state_4_tokens.insert(Token::new("OpeningBracket"), Reaction::ShiftTo(4));

    let mut state_5_tokens = HashMap::new();
    let reduce_to_number =
        Reaction::Reduce(Reduction::with_function("F", 1, ast::ast_create_number));
    state_5_tokens.insert(Token::new("Plus"), reduce_to_number.clone());
    state_5_tokens.insert(Token::new("Times"), reduce_to_number.clone());
    state_5_tokens.insert(Token::new("ClosingBracket"), reduce_to_number.clone());
    state_5_tokens.insert(Token::end_token("EndSymbol"), reduce_to_number.clone());

    let mut state_6_tokens = HashMap::new();
    state_6_tokens.insert(Token::new("Id"), Reaction::ShiftTo(5));
    state_6_tokens.insert(Token::new("OpeningBracket"), Reaction::ShiftTo(4));

    let mut state_7_tokens = HashMap::new();
    state_7_tokens.insert(Token::new("Id"), Reaction::ShiftTo(5));
    state_7_tokens.insert(Token::new("OpeningBracket"), Reaction::ShiftTo(4));

    let mut state_8_tokens = HashMap::new();
    state_8_tokens.insert(Token::new("Plus"), Reaction::ShiftTo(6));
    state_8_tokens.insert(Token::new("ClosingBracket"), Reaction::ShiftTo(11));

    let mut state_9_tokens = HashMap::new();
    let reduce_to_addition =
        Reaction::Reduce(Reduction::with_function("E", 3, ast::ast_create_addition));
    state_9_tokens.insert(Token::new("Plus"), reduce_to_addition.clone());
    state_9_tokens.insert(Token::new("Times"), Reaction::ShiftTo(7));
    state_9_tokens.insert(Token::new("ClosingBracket"), reduce_to_addition.clone());
    state_9_tokens.insert(Token::end_token("EndSymbol"), reduce_to_addition.clone());

    let mut state_10_tokens = HashMap::new();
    let reduce_to_multiplication = Reaction::Reduce(Reduction::with_function(
        "T",
        3,
        ast::ast_create_multiplication,
    ));
    state_10_tokens.insert(Token::new("Plus"), reduce_to_multiplication.clone());
    state_10_tokens.insert(Token::new("Times"), reduce_to_multiplication.clone());
    state_10_tokens.insert(
        Token::new("ClosingBracket"),
        reduce_to_multiplication.clone(),
    );
    state_10_tokens.insert(
        Token::end_token("EndSymbol"),
        reduce_to_multiplication.clone(),
    );

    let mut state_11_tokens = HashMap::new();
    let reduce_to_calculation = Reaction::Reduce(Reduction::with_function(
        "F",
        3,
        ast::ast_create_calculation,
    ));
    state_11_tokens.insert(Token::new("Plus"), reduce_to_calculation.clone());
    state_11_tokens.insert(Token::new("Times"), reduce_to_calculation.clone());
    state_11_tokens.insert(Token::new("ClosingBracket"), reduce_to_calculation.clone());
    state_11_tokens.insert(Token::end_token("EndSymbol"), reduce_to_calculation.clone());

    let mut table: Vec<HashMap<Token, Reaction>> = Vec::new();
    table.push(state_0_tokens);
    table.push(state_1_tokens);
    table.push(state_2_tokens);
    table.push(state_3_tokens);
    table.push(state_4_tokens);
    table.push(state_5_tokens);
    table.push(state_6_tokens);
    table.push(state_7_tokens);
    table.push(state_8_tokens);
    table.push(state_9_tokens);
    table.push(state_10_tokens);
    table.push(state_11_tokens);

    table
}

fn get_goto_table() -> Vec<HashMap<String, usize>> {
    let mut state_0_goto = HashMap::new();
    state_0_goto.insert("E".to_string(), 1);
    state_0_goto.insert("T".to_string(), 2);
    state_0_goto.insert("F".to_string(), 3);

    let mut state_4_goto = HashMap::new();
    state_4_goto.insert("E".to_string(), 8);
    state_4_goto.insert("T".to_string(), 2);
    state_4_goto.insert("F".to_string(), 3);

    let mut state_6_goto = HashMap::new();
    state_6_goto.insert("T".to_string(), 9);
    state_6_goto.insert("F".to_string(), 3);

    let mut state_7_goto = HashMap::new();
    state_7_goto.insert("F".to_string(), 10);

    let mut table: Vec<HashMap<String, usize>> = Vec::new();
    table.push(state_0_goto);
    table.push(HashMap::new());
    table.push(HashMap::new());
    table.push(HashMap::new());
    table.push(state_4_goto);
    table.push(HashMap::new());
    table.push(state_6_goto);
    table.push(state_7_goto);
    table.push(HashMap::new());
    table.push(HashMap::new());
    table.push(HashMap::new());
    table.push(HashMap::new());

    table
}

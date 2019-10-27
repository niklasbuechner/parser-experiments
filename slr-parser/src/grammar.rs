use super::NonTerminal;
use super::Reaction;
use super::Token;
use std::collections::HashMap;

pub(crate) struct Grammar {
    pub starting_state: usize,
}
impl Grammar {
    pub fn new() -> Self {
        Grammar { starting_state: 0 }
    }

    pub fn get_slr_table(&self) -> Vec<HashMap<Token, Reaction>> {
        let mut state_0_tokens = HashMap::new();
        state_0_tokens.insert(Token::Id, Reaction::ShiftTo(5));
        state_0_tokens.insert(Token::OpeningBracket, Reaction::ShiftTo(4));

        let mut state_1_tokens = HashMap::new();
        state_1_tokens.insert(Token::Plus, Reaction::ShiftTo(6));
        state_1_tokens.insert(Token::EndSymbol, Reaction::Accept);

        let mut state_2_tokens = HashMap::new();
        state_2_tokens.insert(
            Token::Plus,
            Reaction::ReduceToAndPopStackBy(NonTerminal::E, 1),
        );
        state_2_tokens.insert(Token::Times, Reaction::ShiftTo(7));
        state_2_tokens.insert(
            Token::ClosingBracket,
            Reaction::ReduceToAndPopStackBy(NonTerminal::E, 1),
        );
        state_2_tokens.insert(
            Token::EndSymbol,
            Reaction::ReduceToAndPopStackBy(NonTerminal::E, 1),
        );

        let mut state_3_tokens = HashMap::new();
        state_3_tokens.insert(
            Token::Plus,
            Reaction::ReduceToAndPopStackBy(NonTerminal::T, 1),
        );
        state_3_tokens.insert(
            Token::Times,
            Reaction::ReduceToAndPopStackBy(NonTerminal::T, 1),
        );
        state_3_tokens.insert(
            Token::ClosingBracket,
            Reaction::ReduceToAndPopStackBy(NonTerminal::T, 1),
        );
        state_3_tokens.insert(
            Token::EndSymbol,
            Reaction::ReduceToAndPopStackBy(NonTerminal::T, 1),
        );

        let mut state_4_tokens = HashMap::new();
        state_4_tokens.insert(Token::Id, Reaction::ShiftTo(5));
        state_4_tokens.insert(Token::OpeningBracket, Reaction::ShiftTo(4));

        let mut state_5_tokens = HashMap::new();
        state_5_tokens.insert(
            Token::Plus,
            Reaction::ReduceToAndPopStackBy(NonTerminal::F, 1),
        );
        state_5_tokens.insert(
            Token::Times,
            Reaction::ReduceToAndPopStackBy(NonTerminal::F, 1),
        );
        state_5_tokens.insert(
            Token::ClosingBracket,
            Reaction::ReduceToAndPopStackBy(NonTerminal::F, 1),
        );
        state_5_tokens.insert(
            Token::EndSymbol,
            Reaction::ReduceToAndPopStackBy(NonTerminal::F, 1),
        );

        let mut state_6_tokens = HashMap::new();
        state_6_tokens.insert(Token::Id, Reaction::ShiftTo(5));
        state_6_tokens.insert(Token::OpeningBracket, Reaction::ShiftTo(4));

        let mut state_7_tokens = HashMap::new();
        state_7_tokens.insert(Token::Id, Reaction::ShiftTo(5));
        state_7_tokens.insert(Token::OpeningBracket, Reaction::ShiftTo(4));

        let mut state_8_tokens = HashMap::new();
        state_8_tokens.insert(Token::Plus, Reaction::ShiftTo(6));
        state_8_tokens.insert(Token::ClosingBracket, Reaction::ShiftTo(11));

        let mut state_9_tokens = HashMap::new();
        state_9_tokens.insert(
            Token::Plus,
            Reaction::ReduceToAndPopStackBy(NonTerminal::E, 3),
        );
        state_9_tokens.insert(Token::Times, Reaction::ShiftTo(7));
        state_9_tokens.insert(
            Token::ClosingBracket,
            Reaction::ReduceToAndPopStackBy(NonTerminal::E, 3),
        );
        state_9_tokens.insert(
            Token::EndSymbol,
            Reaction::ReduceToAndPopStackBy(NonTerminal::E, 3),
        );

        let mut state_10_tokens = HashMap::new();
        state_10_tokens.insert(
            Token::Plus,
            Reaction::ReduceToAndPopStackBy(NonTerminal::T, 3),
        );
        state_10_tokens.insert(
            Token::Times,
            Reaction::ReduceToAndPopStackBy(NonTerminal::T, 3),
        );
        state_10_tokens.insert(
            Token::ClosingBracket,
            Reaction::ReduceToAndPopStackBy(NonTerminal::T, 3),
        );
        state_10_tokens.insert(
            Token::EndSymbol,
            Reaction::ReduceToAndPopStackBy(NonTerminal::T, 3),
        );

        let mut state_11_tokens = HashMap::new();
        state_11_tokens.insert(
            Token::Plus,
            Reaction::ReduceToAndPopStackBy(NonTerminal::F, 3),
        );
        state_11_tokens.insert(
            Token::Times,
            Reaction::ReduceToAndPopStackBy(NonTerminal::F, 3),
        );
        state_11_tokens.insert(
            Token::ClosingBracket,
            Reaction::ReduceToAndPopStackBy(NonTerminal::F, 3),
        );
        state_11_tokens.insert(
            Token::EndSymbol,
            Reaction::ReduceToAndPopStackBy(NonTerminal::F, 3),
        );

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

    pub fn get_goto_table(&self) -> Vec<HashMap<NonTerminal, usize>> {
        let mut state_0_goto = HashMap::new();
        state_0_goto.insert(NonTerminal::E, 1);
        state_0_goto.insert(NonTerminal::T, 2);
        state_0_goto.insert(NonTerminal::F, 3);

        let mut state_4_goto = HashMap::new();
        state_4_goto.insert(NonTerminal::E, 8);
        state_4_goto.insert(NonTerminal::T, 2);
        state_4_goto.insert(NonTerminal::F, 3);

        let mut state_6_goto = HashMap::new();
        state_6_goto.insert(NonTerminal::T, 9);
        state_6_goto.insert(NonTerminal::F, 3);

        let mut state_7_goto = HashMap::new();
        state_7_goto.insert(NonTerminal::F, 10);

        let mut table: Vec<HashMap<NonTerminal, usize>> = Vec::new();
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
}

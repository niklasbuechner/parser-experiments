use super::Token;
use std::collections::HashMap;

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub(crate) enum GrammarSymbol {
    Terminal(Token),
    NonTerminal(String),
    EndSymbol,
}

pub(crate) struct Grammar {

}
impl Grammar {
    pub fn new() -> Self {
        Grammar {}
    }

    pub fn get_parse_table(&self) -> HashMap<String, HashMap<Token, Vec<GrammarSymbol>>> {
        let e_opening_bracket_and_id_production = vec![
            GrammarSymbol::NonTerminal("T".to_string()),
            GrammarSymbol::NonTerminal("E'".to_string()),
        ];

        let e_stroke_plus_production = vec![
            GrammarSymbol::Terminal(Token::Plus),
            GrammarSymbol::NonTerminal("T".to_string()),
            GrammarSymbol::NonTerminal("E'".to_string()),
        ];

        let t_opening_bracket_and_id_production = vec![
            GrammarSymbol::NonTerminal("F".to_string()),
            GrammarSymbol::NonTerminal("T'".to_string()),
        ];

        let t_stroke_times_production = vec![
            GrammarSymbol::Terminal(Token::Times),
            GrammarSymbol::NonTerminal("F".to_string()),
            GrammarSymbol::NonTerminal("T'".to_string()),
        ];

        let f_id_production = vec![GrammarSymbol::Terminal(Token::Id)];
        let f_opening_bracket_production = vec![
            GrammarSymbol::Terminal(Token::OpenBracket),
            GrammarSymbol::NonTerminal("E".to_string()),
            GrammarSymbol::Terminal(Token::ClosingBracket),
        ];

        let mut e_line = HashMap::new();
        e_line.insert(
            Token::OpenBracket,
            e_opening_bracket_and_id_production.clone(),
        );
        e_line.insert(Token::Id, e_opening_bracket_and_id_production);

        let mut e_stroke_line = HashMap::new();
        e_stroke_line.insert(Token::ClosingBracket, Vec::new());
        e_stroke_line.insert(Token::Plus, e_stroke_plus_production);
        e_stroke_line.insert(Token::EndSymbol, Vec::new());

        let mut t_line = HashMap::new();
        t_line.insert(
            Token::OpenBracket,
            t_opening_bracket_and_id_production.clone(),
        );
        t_line.insert(Token::Id, t_opening_bracket_and_id_production);

        let mut t_stroke_line = HashMap::new();
        t_stroke_line.insert(Token::ClosingBracket, Vec::new());
        t_stroke_line.insert(Token::Plus, Vec::new());
        t_stroke_line.insert(Token::Times, t_stroke_times_production);
        t_stroke_line.insert(Token::EndSymbol, Vec::new());

        let mut f_line = HashMap::new();
        f_line.insert(Token::Id, f_id_production);
        f_line.insert(Token::OpenBracket, f_opening_bracket_production);

        let mut table = HashMap::new();
        table.insert("E".to_string(), e_line);
        table.insert("E'".to_string(), e_stroke_line);
        table.insert("T".to_string(), t_line);
        table.insert("T'".to_string(), t_stroke_line);
        table.insert("F".to_string(), f_line);

        table
    }
}

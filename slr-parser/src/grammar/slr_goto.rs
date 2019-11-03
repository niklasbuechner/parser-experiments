use super::GrammarSymbol;
use super::Production;

pub(crate) struct SlrGoto {}
impl SlrGoto {
    pub fn get_goto_symbols(productions: &Vec<Production>) -> Vec<GrammarSymbol> {
        productions
            .iter()
            .map(|production| production.get_next_grammar_symbol())
            .filter(|symbol| match symbol {
                None => false,
                _ => true,
            })
            .map(|symbol| symbol.unwrap())
            .cloned()
            .collect()
    }

    pub fn get_goto(symbol: &GrammarSymbol, productions: &Vec<Production>) -> Vec<Production> {
        productions
            .iter()
            .filter(|production| production.get_next_grammar_symbol() == Some(symbol))
            .cloned()
            .map(|mut production| {
                production.move_cursor_ahead();

                production
            })
            .collect()
    }
}

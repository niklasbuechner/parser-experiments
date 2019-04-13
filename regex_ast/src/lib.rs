#[derive(Debug, PartialEq)]
pub enum RegexAstElements {
    Alternation(Box<RegexAstElements>, Box<RegexAstElements>),
    Concatenation(Box<RegexAstElements>, Box<RegexAstElements>),
    Leaf(char),
    None,
    ZeroOrMore(Box<RegexAstElements>),
}

pub fn get_regex_syntax_tree(regex: &str) -> RegexAstElements {
    let regex_characters: Vec<char> = regex.chars().collect();
    let mut ast = RegexAstElements::None;

    let mut index = 0;
    let mut character;
    loop {
        if index >= regex_characters.len() {
            return ast;
        }

        character = regex_characters[index];

        match character {
            '|' => {
                match ast {
                    RegexAstElements::None => return ast,
                    _ => {
                        println!("{}", &regex[index + 1..regex_characters.len()]);
                        ast = RegexAstElements::Alternation(
                            Box::new(ast),
                            Box::new(get_regex_syntax_tree(
                                &regex[index + 1..regex_characters.len()]
                            )),
                        );

                        return ast;
                    },
                }
            },
            _ => {
                match ast {
                    RegexAstElements::None => ast = get_ast_leaf(character),
                    _ => {
                        ast = RegexAstElements::Concatenation(
                            Box::new(ast),
                            Box::new(get_ast_leaf(character)),
                        )
                    },
                }
            }
        }

        index += 1;
    }
}

pub fn get_ast_leaf(character: char) -> RegexAstElements {
    return RegexAstElements::Leaf(character);
}

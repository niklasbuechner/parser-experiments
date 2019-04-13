#[derive(Debug, PartialEq)]
pub enum RegexAstElements {
    Concatenation(Box<RegexAstElements>, Box<RegexAstElements>),
    Leaf(char),
    None,
}

pub fn get_regex_syntax_tree(regex: &str) -> RegexAstElements {
    let regex_characters: Vec<char> = regex.chars().collect();
    let mut groups = Vec::new();
    let mut current_group = String::new();

    for i in 0..regex_characters.len() {
        match regex_characters[i] {
            _ => {
                current_group.push(regex_characters[i]);
                groups.push(current_group);
                current_group = String::new();
            }
        }
    }

    let mut ast = RegexAstElements::None;
    for group in groups {
        // single character
        if group.len() == 1 {
            let characters: Vec<char> = group.chars().collect();
            let leaf_character = characters[0];
            match ast {
                RegexAstElements::None => {
                    println!("Add first leaf {}", leaf_character);
                    ast = RegexAstElements::Leaf(leaf_character);
                },
                _ => {
                        println!("Add concatenation for character {}", leaf_character);
                        ast = RegexAstElements::Concatenation(
                        Box::new(ast),
                        Box::new(RegexAstElements::Leaf(leaf_character)),
                    );
                },
                _ => {},
            }
        }

        println!("{:#?}", ast);
    }

    return ast;
}

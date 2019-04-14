#[derive(Debug, PartialEq)]
pub enum RegexAstElements {
    Alternation(Box<RegexAstElements>, Box<RegexAstElements>),
    Concatenation(Box<RegexAstElements>, Box<RegexAstElements>),
    Leaf(char),
    None,
    ZeroOrMore(Box<RegexAstElements>),
    ZeroOrOne(Box<RegexAstElements>),
}

#[derive(Debug, PartialEq)]
struct State {
    pub character: char,
    pub left_next: Option<Vec<usize>>,
    pub right_next: Option<Vec<usize>>,
    pub is_escaped: bool,
}
impl State {
    pub fn new(character: char, left_next: Option<Vec<usize>>, right_next: Option<Vec<usize>>) -> Self {
        State {
            character,
            left_next,
            right_next,
            is_escaped: false,
        }
    }

    pub fn new_escaped(character: char, left_next: Option<Vec<usize>>, right_next: Option<Vec<usize>>) -> Self {
        State {
            character,
            left_next,
            right_next,
            is_escaped: true,
        }
    }
}

pub fn get_regex_syntax_tree(regex: &str) -> RegexAstElements {
    let mut stack = Vec::with_capacity(regex.len());
    let concatenation_list = calculate_concatenation_list(&mut stack, regex);

    return get_ast_for_concatenation_list(&stack, &concatenation_list.list);
}

struct ConcatenationList {
    list: Vec<usize>,
    consumed_characters: usize,
}
impl ConcatenationList {
    pub fn new(list: Vec<usize>, consumed_characters: usize) -> Self {
        ConcatenationList {
            list,
            consumed_characters,
        }
    }
}

fn calculate_concatenation_list(stack: &mut Vec<State>, regex: &str) -> ConcatenationList {
    let regex_characters: Vec<char> = regex.chars().collect();
    let mut character_is_escaped = false;
    let mut concatenation_list = Vec::new();

    let mut index = 0;
    loop {
        if index >= regex_characters.len() {
            // Here the index is NOT one short of the amount of characters we have consumed, since
            // at the end of the last iteration it was increased by one and we have not yet
            // consumed another character.
            return ConcatenationList::new(concatenation_list, index);
        }
        let character = regex_characters[index];

        if character_is_escaped {
            concatenation_list.push(stack.len());
            stack.push(State::new_escaped(character, None, None));

            character_is_escaped = false;
            index += 1;

            continue;
        }

        match character {
            '*' => {
                let list_length = concatenation_list.len();
                let last_state_index = concatenation_list[list_length - 1];
                concatenation_list[list_length - 1] = stack.len();
                stack.push(State::new('*', Some(vec![last_state_index]), None));
            },
            '?' => {
                let list_length = concatenation_list.len();
                let last_state_index = concatenation_list[list_length - 1];
                concatenation_list[list_length - 1] = stack.len();
                stack.push(State::new('?', Some(vec![last_state_index]), None));
            },
            '+' => {
                let list_length = concatenation_list.len();
                let last_state_index = concatenation_list[list_length - 1];
                concatenation_list.push(stack.len());
                stack.push(State::new('*', Some(vec![last_state_index]), None));
            },
            '(' => {
                let mut group_list = calculate_concatenation_list(stack, &regex[index + 1..]);
                concatenation_list.append(&mut group_list.list);
                index += group_list.consumed_characters;
            },
            ')' => {
                // The index is always 1 short of how many characters we have consumed.
                return ConcatenationList::new(concatenation_list, index + 1);
            },
            '|' => {
                let first_list = concatenation_list;
                let second_concatenation_list = calculate_concatenation_list(stack, &regex[index + 1..]);
                let second_list = second_concatenation_list.list;
                stack.push(State::new('|', Some(first_list), Some(second_list)));

                return ConcatenationList::new(
                    vec![stack.len() - 1],
                    // The index is always 1 short of how many characters we have consumed.
                    index + 1 + second_concatenation_list.consumed_characters
                );
            },
            '\\' => character_is_escaped = true,
            _ => {
                concatenation_list.push(stack.len());
                stack.push(State::new(character, None, None));
            },
        }

        index += 1;
    }
}

fn get_ast_for_concatenation_list(stack: &Vec<State>, concatenation_list: &Vec<usize>) -> RegexAstElements {
    let mut ast = RegexAstElements::None;

    for index in 0..concatenation_list.len() {
        let next_state_index = concatenation_list[index];
        let state = &stack[next_state_index];

        // The operator_character is used to allow for escaped operators. Therefore the
        // operator_character should not be used when constructing an ast!
        let operator_character;
        if state.is_escaped {
            operator_character = 'a';
        } else {
            operator_character = state.character;
        }
        match operator_character {
            '*' => {
                let left_list = match state.left_next {
                    Some(ref list) => list,
                    None => panic!("This can't be happening"),
                };
                let zero_or_more_ast = RegexAstElements::ZeroOrMore(Box::new(
                    get_ast_for_concatenation_list(&stack, left_list)
                ));

                match ast {
                    RegexAstElements::None => ast = zero_or_more_ast,
                    _ => ast = RegexAstElements::Concatenation(
                        Box::new(ast),
                        Box::new(zero_or_more_ast),
                    ),
                }
            },
            '?' => {
                let left_list = match state.left_next {
                    Some(ref list) => list,
                    None => panic!("This can't be happening"),
                };
                let zero_or_one_ast = RegexAstElements::ZeroOrOne(Box::new(
                    get_ast_for_concatenation_list(&stack, left_list)
                ));

                match ast {
                    RegexAstElements::None => ast = zero_or_one_ast,
                    _ => ast = RegexAstElements::Concatenation(
                        Box::new(ast),
                        Box::new(zero_or_one_ast),
                    ),
                }
            },
            '|' => {
                let left_list = match state.left_next {
                    Some(ref list) => list,
                    None => panic!("This can't be happening"),
                };
                let right_list = match state.right_next {
                    Some(ref list) => list,
                    None => panic!("This can't be happening"),
                };

                let alternation_ast = RegexAstElements::Alternation(
                    Box::new(get_ast_for_concatenation_list(stack, left_list)),
                    Box::new(get_ast_for_concatenation_list(stack, right_list)),
                );

                match ast {
                    RegexAstElements::None => ast = alternation_ast,
                    _ => ast = RegexAstElements::Concatenation(
                        Box::new(ast),
                        Box::new(alternation_ast),
                    ),
                }
            },
            _ => {
                match ast {
                    RegexAstElements::None => ast = RegexAstElements::Leaf(state.character),
                    _ => {
                        ast = RegexAstElements::Concatenation(
                            Box::new(ast),
                            Box::new(RegexAstElements::Leaf(state.character)),
                        );
                    }
                }
            },
        }
    }

    return ast;
}

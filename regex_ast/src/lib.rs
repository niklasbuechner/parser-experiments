use std::convert::From;

#[derive(Debug, PartialEq)]
pub enum RegexAstElements {
    Alternation(Box<RegexAstElements>, Box<RegexAstElements>),
    Concatenation(Box<RegexAstElements>, Box<RegexAstElements>),
    Leaf(MatchingGroup),
    None,
    ZeroOrMore(Box<RegexAstElements>),
    ZeroOrOne(Box<RegexAstElements>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MatchingGroup {
    Character(char),
    Group(Vec<MatchingGroupElements>),
    NegativeGroup(Vec<MatchingGroupElements>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MatchingGroupElements {
    Character(char),
    Range(char, char),
}
impl From<MatchingGroup> for MatchingGroupElements {
    fn from(matching_group: MatchingGroup) -> Self {
        match matching_group {
            MatchingGroup::Character(character) => return MatchingGroupElements::Character(character),
            _ => panic!("Trying to convert a group into a character"),
        }
    }
}

#[derive(Debug, PartialEq)]
struct State {
    pub matching_group: MatchingGroup,
    pub left_next: Option<Vec<usize>>,
    pub right_next: Option<Vec<usize>>,
    pub is_escaped: bool,
}
impl State {
    pub fn new(matching_group: MatchingGroup, left_next: Option<Vec<usize>>, right_next: Option<Vec<usize>>) -> Self {
        State {
            matching_group,
            left_next,
            right_next,
            is_escaped: false,
        }
    }

    pub fn new_escaped(matching_group: MatchingGroup, left_next: Option<Vec<usize>>, right_next: Option<Vec<usize>>) -> Self {
        State {
            matching_group,
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
    let regex_characters = get_character_array(regex);
    let mut character_is_in_quotes = false;
    let mut concatenation_list = Vec::new();

    let mut index = 0;
    loop {
        if index >= regex_characters.len() {
            // Here the index is NOT one short of the amount of characters we have consumed, since
            // at the end of the last iteration it was increased by one and we have not yet
            // consumed another character.
            return ConcatenationList::new(concatenation_list, index);
        }
        let mut character = &regex_characters[index];

        if character_is_in_quotes && character != &MatchingGroup::Character('"') {
            concatenation_list.push(stack.len());
            stack.push(State::new_escaped(character.clone(), None, None));

            index += 1;

            continue;
        }

        let character_group; // Needs to be declared here due to lifetimes.
        if character == &MatchingGroup::Character('[') {
            character_group = get_character_group(&regex_characters[index + 1..]);
            character = &character_group.group;
            index += character_group.consumed_characters;
        }

        match character {
            MatchingGroup::Character('*') => {
                let list_length = concatenation_list.len();
                let last_state_index = concatenation_list[list_length - 1];
                concatenation_list[list_length - 1] = stack.len();
                stack.push(State::new(MatchingGroup::Character('*'), Some(vec![last_state_index]), None));
            },
            MatchingGroup::Character('?') => {
                let list_length = concatenation_list.len();
                let last_state_index = concatenation_list[list_length - 1];
                concatenation_list[list_length - 1] = stack.len();
                stack.push(State::new(MatchingGroup::Character('?'), Some(vec![last_state_index]), None));
            },
            MatchingGroup::Character('+') => {
                let list_length = concatenation_list.len();
                let last_state_index = concatenation_list[list_length - 1];
                concatenation_list.push(stack.len());
                stack.push(State::new(MatchingGroup::Character('*'), Some(vec![last_state_index]), None));
            },
            MatchingGroup::Character('(') => {
                let mut group_list = calculate_concatenation_list(stack, &regex[index + 1..]);
                concatenation_list.append(&mut group_list.list);
                index += group_list.consumed_characters;
            },
            MatchingGroup::Character(')') => {
                // The index is always 1 short of how many characters we have consumed.
                return ConcatenationList::new(concatenation_list, index + 1);
            },
            MatchingGroup::Character('|') => {
                let first_list = concatenation_list;
                let second_concatenation_list = calculate_concatenation_list(stack, &regex[index + 1..]);
                let second_list = second_concatenation_list.list;
                stack.push(State::new(MatchingGroup::Character('|'), Some(first_list), Some(second_list)));

                return ConcatenationList::new(
                    vec![stack.len() - 1],
                    // The index is always 1 short of how many characters we have consumed.
                    index + 1 + second_concatenation_list.consumed_characters
                );
            },
            MatchingGroup::Character('"') => character_is_in_quotes = !character_is_in_quotes,
            _ => {
                concatenation_list.push(stack.len());
                stack.push(State::new(character.clone(), None, None));
            },
        }

        index += 1;
    }
}

fn get_character_array(regex: &str) -> Vec<MatchingGroup> {
    let input_characters: Vec<char> = regex.chars().collect();
    let mut output_characters = Vec::with_capacity(input_characters.len());
    let mut state = 0;

    for index in 0..input_characters.len() {
        let current_character = input_characters[index];
        match state {
            0 => {
                match current_character {
                    '\\' => state = 1,
                    _ => output_characters.push(MatchingGroup::Character(current_character)),
                }
            },
            // the previous character was '\'
            1 => {
                match current_character {
                    'r' => {
                        output_characters.push(MatchingGroup::Character('\r'));
                        state = 0;
                    },
                    'n' => {
                        output_characters.push(MatchingGroup::Character('\n'));
                        state = 0;
                    },
                    't' => {
                        output_characters.push(MatchingGroup::Character('\t'));
                        state = 0;
                    },
                    '\\' => output_characters.push(MatchingGroup::Character('\\')),
                    _ => {
                        output_characters.push(MatchingGroup::Character('\\'));
                        output_characters.push(MatchingGroup::Character(current_character));
                    },
                }
            },
            _ => {},
        }
    }

    if state != 0 {
        output_characters.push(MatchingGroup::Character('\\'));
    }

    return output_characters;
}

struct CharacterGroupCalculation {
    pub group: MatchingGroup,
    pub consumed_characters: usize,
}
impl CharacterGroupCalculation {
    pub fn new(group: MatchingGroup, consumed_characters: usize) -> Self {
        CharacterGroupCalculation {
            group,
            consumed_characters,
        }
    }
}

fn get_character_group(characters: &[MatchingGroup]) -> CharacterGroupCalculation {
    let mut index = 0;
    let mut state = 0;
    let mut group = Vec::new();
    let mut previous_character = ' ';

    let negative_group;
    if characters.len() > 0 && characters[0] == MatchingGroup::Character('^') {
        negative_group = true;
        index += 1;
    } else {
        negative_group = false;
    }

    loop {
        let character;
        if index < characters.len() {
            character = &characters[index];
            index += 1;
        } else {
            panic!("Invalid regex, character group never closed.");
        }

        let actual_character = match character {
            MatchingGroup::Character(character) => character,
            _ => panic!("A group exists before groups were evaluated?"),
        };

        match state {
            0 => {
                match actual_character {
                    ']' => {
                        if negative_group {
                            return CharacterGroupCalculation::new(MatchingGroup::NegativeGroup(group), index + 2)
                        } else {
                            return CharacterGroupCalculation::new(MatchingGroup::Group(group), index + 2);
                        }
                    },
                    'a'..='z' | '0'..='9' => {
                        state = 1;
                        previous_character = *actual_character;
                    }
                    _ => group.push(MatchingGroupElements::from(character.clone())),
                }
            },
            // The character could be the start of a range e.g. a-c
            1 => {
                match actual_character {
                    'a'..='z' | '0'..='9' => {
                        group.push(MatchingGroupElements::Character(previous_character));
                        previous_character = *actual_character;
                    },
                    '-' => {
                        state = 2;
                    },
                    ']' => {
                        group.push(MatchingGroupElements::Character(previous_character));

                        if negative_group {
                            return CharacterGroupCalculation::new(MatchingGroup::NegativeGroup(group), index + 2)
                        } else {
                            return CharacterGroupCalculation::new(MatchingGroup::Group(group), index + 2);
                        }
                    },
                    _ => {
                        group.push(MatchingGroupElements::Character(previous_character));
                        group.push(MatchingGroupElements::from(character.clone()));

                        state = 0;
                    }
                }
            },
            // The last character was a `-` after a range starting character
            2 => {
                match actual_character {
                    'a'..='z' | '0'..='9' => {
                        group.push(MatchingGroupElements::Range(previous_character, *actual_character));
                        state = 0;
                    },
                    ']' => {
                        group.push(MatchingGroupElements::Character(previous_character));
                        group.push(MatchingGroupElements::Character('-'));

                        if negative_group {
                            return CharacterGroupCalculation::new(MatchingGroup::NegativeGroup(group), index + 2)
                        } else {
                            return CharacterGroupCalculation::new(MatchingGroup::Group(group), index + 2);
                        }
                    },
                    _ => {
                        group.push(MatchingGroupElements::Character(previous_character));
                        group.push(MatchingGroupElements::Character('-'));
                        group.push(MatchingGroupElements::from(character.clone()));

                        state = 0;
                    }
                }
            },
            _ => {},
        }
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
            operator_character = &MatchingGroup::Character('a');
        } else {
            operator_character = &state.matching_group;
        }
        match operator_character {
            MatchingGroup::Character('*') => {
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
            MatchingGroup::Character('?') => {
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
            MatchingGroup::Character('|') => {
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
                    RegexAstElements::None => ast = RegexAstElements::Leaf(state.matching_group.clone()),
                    _ => {
                        ast = RegexAstElements::Concatenation(
                            Box::new(ast),
                            Box::new(RegexAstElements::Leaf(state.matching_group.clone())),
                        );
                    }
                }
            },
        }
    }

    return ast;
}

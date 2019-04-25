mod characters;

use crate::ConcatenationList;
use crate::MatchingGroup;
use crate::MatchingGroupElements;
use crate::State;
use characters::get_character_array;
use std::convert::From;

impl From<MatchingGroup> for MatchingGroupElements {
    fn from(matching_group: MatchingGroup) -> Self {
        match matching_group {
            MatchingGroup::Character(character) => {
                return MatchingGroupElements::Character(character)
            }
            _ => panic!("Trying to convert a group into a character"),
        }
    }
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

pub(crate) fn calculate_concatenation_list(
    stack: &mut Vec<State>,
    regex: &str,
) -> ConcatenationList {
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
                stack.push(State::new(
                    MatchingGroup::Character('*'),
                    Some(vec![last_state_index]),
                    None,
                ));
            }
            MatchingGroup::Character('?') => {
                let list_length = concatenation_list.len();
                let last_state_index = concatenation_list[list_length - 1];
                concatenation_list[list_length - 1] = stack.len();
                stack.push(State::new(
                    MatchingGroup::Character('?'),
                    Some(vec![last_state_index]),
                    None,
                ));
            }
            MatchingGroup::Character('+') => {
                let list_length = concatenation_list.len();
                let last_state_index = concatenation_list[list_length - 1];
                concatenation_list.push(stack.len());
                stack.push(State::new(
                    MatchingGroup::Character('*'),
                    Some(vec![last_state_index]),
                    None,
                ));
            }
            MatchingGroup::Character('(') => {
                let mut group_list = calculate_concatenation_list(stack, &regex[index + 1..]);
                concatenation_list.append(&mut group_list.list);
                index += group_list.consumed_characters;
            }
            MatchingGroup::Character(')') => {
                // The index is always 1 short of how many characters we have consumed.
                return ConcatenationList::new(concatenation_list, index + 1);
            }
            MatchingGroup::Character('|') => {
                let first_list = concatenation_list;
                let second_concatenation_list =
                    calculate_concatenation_list(stack, &regex[index + 1..]);
                let second_list = second_concatenation_list.list;
                stack.push(State::new(
                    MatchingGroup::Character('|'),
                    Some(first_list),
                    Some(second_list),
                ));

                return ConcatenationList::new(
                    vec![stack.len() - 1],
                    // The index is always 1 short of how many characters we have consumed.
                    index + 1 + second_concatenation_list.consumed_characters,
                );
            }
            MatchingGroup::Character('"') => character_is_in_quotes = !character_is_in_quotes,
            _ => {
                concatenation_list.push(stack.len());
                stack.push(State::new(character.clone(), None, None));
            }
        }

        index += 1;
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
            0 => match actual_character {
                ']' => {
                    if negative_group {
                        return CharacterGroupCalculation::new(
                            MatchingGroup::NegativeGroup(group),
                            index,
                        );
                    } else {
                        return CharacterGroupCalculation::new(MatchingGroup::Group(group), index);
                    }
                }
                'a'..='z' | '0'..='9' => {
                    state = 1;
                    previous_character = *actual_character;
                }
                _ => group.push(MatchingGroupElements::from(character.clone())),
            },
            // The character could be the start of a range e.g. a-c
            1 => match actual_character {
                'a'..='z' | '0'..='9' => {
                    group.push(MatchingGroupElements::Character(previous_character));
                    previous_character = *actual_character;
                }
                '-' => {
                    state = 2;
                }
                ']' => {
                    group.push(MatchingGroupElements::Character(previous_character));

                    if negative_group {
                        return CharacterGroupCalculation::new(
                            MatchingGroup::NegativeGroup(group),
                            index,
                        );
                    } else {
                        return CharacterGroupCalculation::new(MatchingGroup::Group(group), index);
                    }
                }
                _ => {
                    group.push(MatchingGroupElements::Character(previous_character));
                    group.push(MatchingGroupElements::from(character.clone()));

                    state = 0;
                }
            },
            // The last character was a `-` after a range starting character
            2 => match actual_character {
                'a'..='z' | '0'..='9' => {
                    group.push(MatchingGroupElements::Range(
                        previous_character,
                        *actual_character,
                    ));
                    state = 0;
                }
                ']' => {
                    group.push(MatchingGroupElements::Character(previous_character));
                    group.push(MatchingGroupElements::Character('-'));

                    if negative_group {
                        return CharacterGroupCalculation::new(
                            MatchingGroup::NegativeGroup(group),
                            index,
                        );
                    } else {
                        return CharacterGroupCalculation::new(MatchingGroup::Group(group), index);
                    }
                }
                _ => {
                    group.push(MatchingGroupElements::Character(previous_character));
                    group.push(MatchingGroupElements::Character('-'));
                    group.push(MatchingGroupElements::from(character.clone()));

                    state = 0;
                }
            },
            _ => {}
        }
    }
}

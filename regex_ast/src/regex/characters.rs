use crate::MatchingGroup;
use std::convert::TryFrom;

pub fn get_character_array(regex: &str) -> Vec<MatchingGroup> {
    let input_characters: Vec<char> = regex.chars().collect();
    let mut first_hexa_character = ' ';
    let mut output_characters = Vec::with_capacity(input_characters.len() + 1);
    let mut state = 0;

    for index in 0..input_characters.len() {
        let current_character = input_characters[index];
        match state {
            0 => {
                match current_character {
                    '\\' => state = 1,
                    _ => output_characters.push(MatchingGroup::Character(current_character)),
                }
            }
            // The previous character was '\'
            1 => {
                match current_character {
                    'r' => {
                        output_characters.push(MatchingGroup::Character('\r'));
                        state = 0;
                    }
                    'n' => {
                        output_characters.push(MatchingGroup::Character('\n'));
                        state = 0;
                    }
                    't' => {
                        output_characters.push(MatchingGroup::Character('\t'));
                        state = 0;
                    }
                    'x' => {
                        state = 2;
                    }
                    '\\' => output_characters.push(MatchingGroup::Character('\\')),
                    _ => {
                        output_characters.push(MatchingGroup::Character('\\'));
                        output_characters.push(MatchingGroup::Character(current_character));
                    }
                }
            }
            // The previous characters where `\x`
            2 => {
                match current_character {
                    '0'..='9' | 'a'..='f' | 'A'..='F' => {
                        first_hexa_character = current_character;
                        state = 3;
                    }
                    '\\' => {
                        output_characters.push(MatchingGroup::Character('\\'));
                        output_characters.push(MatchingGroup::Character('x'));

                        state = 1;
                    }
                    _ => {
                        output_characters.push(MatchingGroup::Character('\\'));
                        output_characters.push(MatchingGroup::Character('x'));
                        output_characters.push(MatchingGroup::Character(current_character));
                    }
                }
            }
            // The previous characters where `\x` and a hex character
            3 => {
                match current_character {
                    '0'..='9' | 'a'..='f' | 'A'..='F' => {
                        let first_character_value = get_character_hex_value(first_hexa_character);
                        let second_character_value = get_character_hex_value(current_character);
                        let character_value = first_character_value * 16 + second_character_value;

                        match char::try_from(character_value) {
                            Ok(character) => {
                                output_characters.push(MatchingGroup::Character(character))
                            }
                            Err(_) => panic!("Invalid hex character found"),
                        }

                        state = 0;
                    }
                    '\\' => {
                        output_characters.push(MatchingGroup::Character('\\'));
                        output_characters.push(MatchingGroup::Character('x'));
                        output_characters.push(MatchingGroup::Character(first_hexa_character));

                        state = 1;
                    }
                    _ => {
                        output_characters.push(MatchingGroup::Character('\\'));
                        output_characters.push(MatchingGroup::Character('x'));
                        output_characters.push(MatchingGroup::Character(first_hexa_character));
                        output_characters.push(MatchingGroup::Character(current_character));

                        state = 0;
                    }
                }
            }
            _ => {}
        }
    }

    if state > 0 {
        output_characters.push(MatchingGroup::Character('\\'));
    }

    if state > 1 {
        output_characters.push(MatchingGroup::Character('x'));
    }

    if state == 3 {
        output_characters.push(MatchingGroup::Character(first_hexa_character));
    }

    return output_characters;
}

fn get_character_hex_value(character: char) -> u32 {
    return match character.to_ascii_lowercase() {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10,
        'b' => 11,
        'c' => 12,
        'd' => 13,
        'e' => 14,
        'f' => 15,
        _ => panic!("{} is no valid hex character", character),
    };
}

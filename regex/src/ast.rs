use crate::MatchingGroup;
use crate::RegexAstElements;
use crate::State;

fn get_partial_ast_for_concatenation_list(
    stack: &Vec<State>,
    concatenation_list: &Vec<usize>,
) -> RegexAstElements {
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
                    get_partial_ast_for_concatenation_list(&stack, left_list),
                ));

                match ast {
                    RegexAstElements::None => ast = zero_or_more_ast,
                    _ => {
                        ast = RegexAstElements::Concatenation(
                            Box::new(ast),
                            Box::new(zero_or_more_ast),
                        )
                    }
                }
            }
            MatchingGroup::Character('?') => {
                let left_list = match state.left_next {
                    Some(ref list) => list,
                    None => panic!("This can't be happening"),
                };
                let zero_or_one_ast = RegexAstElements::ZeroOrOne(Box::new(
                    get_partial_ast_for_concatenation_list(&stack, left_list),
                ));

                match ast {
                    RegexAstElements::None => ast = zero_or_one_ast,
                    _ => {
                        ast = RegexAstElements::Concatenation(
                            Box::new(ast),
                            Box::new(zero_or_one_ast),
                        )
                    }
                }
            }
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
                    Box::new(get_partial_ast_for_concatenation_list(stack, left_list)),
                    Box::new(get_partial_ast_for_concatenation_list(stack, right_list)),
                );

                match ast {
                    RegexAstElements::None => ast = alternation_ast,
                    _ => {
                        ast = RegexAstElements::Concatenation(
                            Box::new(ast),
                            Box::new(alternation_ast),
                        )
                    }
                }
            }
            _ => match ast {
                RegexAstElements::None => {
                    ast = RegexAstElements::Leaf(state.matching_group.clone())
                }
                _ => {
                    ast = RegexAstElements::Concatenation(
                        Box::new(ast),
                        Box::new(RegexAstElements::Leaf(state.matching_group.clone())),
                    );
                }
            },
        }
    }

    return ast;
}

pub(crate) fn get_ast_for_concatenation_list(
    stack: &Vec<State>,
    concatenation_list: &Vec<usize>,
) -> RegexAstElements {
    let ast = get_partial_ast_for_concatenation_list(stack, concatenation_list);

    return RegexAstElements::Concatenation(
        Box::new(ast),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
}

use crate::MatchingGroup;
use crate::RegexAstElements;

pub(crate) struct StateMachine {}

#[derive(Debug, PartialEq)]
enum ElementType {
    Concatenation,
    Leaf,
}

#[derive(Debug, PartialEq)]
struct StateCalculations {
    pub(self) index: usize,
    pub(self) parent_index: Option<usize>,
    pub(self) left_child_index: Option<usize>,
    pub(self) right_child_index: Option<usize>,
    pub(self) is_nullable: bool,
    pub(self) first_pos: Vec<usize>,
    pub(self) last_pos: Vec<usize>,
    pub(self) follow_pos: Vec<usize>,
    pub(self) element_type: ElementType,
    pub(self) matching_group: Option<MatchingGroup>,
}
impl StateCalculations {
    pub(self) fn new(
        index: usize,
        left_child_index: Option<usize>,
        right_child_index: Option<usize>,
        element_type: ElementType,
        matching_group: Option<MatchingGroup>,
        is_nullable: bool,
        first_pos: Vec<usize>,
        last_pos: Vec<usize>,
    ) -> StateCalculations {
        StateCalculations {
            index,
            parent_index: None,
            left_child_index,
            right_child_index,
            is_nullable,
            first_pos,
            last_pos,
            follow_pos: Vec::new(),
            element_type,
            matching_group,
        }
    }
}

pub(crate) fn create_state_machine(ast: &RegexAstElements) -> StateMachine {
    println!("{:#?}", ast);
    let mut calculation_states = Vec::with_capacity(100);
    create_calculation_stack_for_element(ast, &mut calculation_states);
    caclulate_follow_pos_for_stack(&mut calculation_states);

    println!("{:#?}", calculation_states);

    return StateMachine {};
}

fn create_calculation_stack_for_element(
    ast: &RegexAstElements,
    stack: &mut Vec<StateCalculations>,
) -> usize {
    let mut left_child_index = None;
    let mut right_child_index = None;
    let element_type;
    let mut matching_group = None;
    let current_index;
    let is_nullable;
    let mut first_pos;
    let mut last_pos;

    match ast {
        RegexAstElements::Concatenation(ref left, ref right) => {
            element_type = ElementType::Concatenation;

            let left_index = create_calculation_stack_for_element(left, stack);
            let right_index = create_calculation_stack_for_element(right, stack);

            current_index = stack.len();
            stack[left_index].parent_index = Some(current_index);
            stack[right_index].parent_index = Some(current_index);

            left_child_index = Some(left_index);
            right_child_index = Some(right_index);

            is_nullable = stack[left_index].is_nullable && stack[right_index].is_nullable;
            first_pos = stack[left_index].first_pos.clone();
            if stack[left_index].is_nullable {
                first_pos.append(&mut stack[right_index].first_pos.clone());
            }
            last_pos = stack[right_index].last_pos.clone();
            if stack[right_index].is_nullable {
                last_pos.append(&mut stack[left_index].last_pos.clone());
            }
        }
        RegexAstElements::Leaf(ref group) => {
            element_type = ElementType::Leaf;
            matching_group = Some(group.clone());

            current_index = stack.len();
            is_nullable = false;
            first_pos = vec![current_index];
            last_pos = vec![current_index];
        }
        _ => panic!("Unknown ast element"),
    }

    stack.push(StateCalculations::new(
        current_index,
        left_child_index,
        right_child_index,
        element_type,
        matching_group,
        is_nullable,
        first_pos,
        last_pos,
    ));

    return current_index;
}

fn caclulate_follow_pos_for_stack(stack: &mut Vec<StateCalculations>) {
    for i in 0..stack.len() {
        match stack[i].element_type {
            ElementType::Concatenation => {
                let last_pos_left = &stack[stack[i].left_child_index.unwrap()].last_pos.clone();
                let first_pos_right = &stack[stack[i].right_child_index.unwrap()].first_pos.clone();

                for position in last_pos_left {
                    stack[*position]
                        .follow_pos
                        .append(&mut first_pos_right.clone());
                }
            }
            _ => {}
        }
    }
}

use crate::MatchingGroup;
use crate::RegexAstElements;
use crate::RegexEngine;
use crate::StateMachine;
use crate::TransitionForMatchingGroup;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum ElementType {
    Alternation,
    Concatenation,
    Leaf,
    ZeroOrMore,
    ZeroOrOne,
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
    pub(self) matching_group_index: Option<usize>,
    pub(self) accepted: bool,
}
impl StateCalculations {
    pub(self) fn new(
        index: usize,
        left_child_index: Option<usize>,
        right_child_index: Option<usize>,
        element_type: ElementType,
        matching_group_index: Option<usize>,
        is_nullable: bool,
        first_pos: Vec<usize>,
        last_pos: Vec<usize>,
        accepted: bool,
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
            matching_group_index,
            accepted,
        }
    }
}

pub(super) struct StateMachineBuilder {
    stack: Vec<StateCalculations>,
    matching_groups: Vec<MatchingGroup>,
}
impl StateMachineBuilder {
    pub(super) fn create_regex_engine(ast: &RegexAstElements) -> RegexEngine {
        println!("{:#?}", ast);

        let mut builder = StateMachineBuilder::new();
        builder.create_calculation_stack_for_element(ast);
        builder.caclulate_follow_pos_for_stack();
        println!("{:#?}", builder.stack);

        return builder.convert_to_regex_engine();
    }

    fn new() -> StateMachineBuilder {
        StateMachineBuilder {
            stack: Vec::with_capacity(100),
            matching_groups: Vec::with_capacity(100),
        }
    }

    fn add_matching_group(&mut self, matching_group: &MatchingGroup) -> usize {
        let index = self
            .matching_groups
            .iter()
            .position(|group| group == matching_group);

        match index {
            Some(index) => return index,
            None => {
                self.matching_groups.push(matching_group.clone());

                return self.matching_groups.len() - 1;
            }
        }
    }

    fn create_calculation_stack_for_element(&mut self, ast: &RegexAstElements) -> usize {
        let mut left_child_index = None;
        let mut right_child_index = None;
        let element_type;
        let mut matching_group_index = None;
        let current_index;
        let is_nullable;
        let mut first_pos;
        let mut last_pos;
        let is_accepted_state;

        match ast {
            RegexAstElements::Alternation(ref left, ref right) => {
                element_type = ElementType::Alternation;

                let left_index = self.create_calculation_stack_for_element(left);
                let right_index = self.create_calculation_stack_for_element(right);

                current_index = self.stack.len();
                self.stack[left_index].parent_index = Some(current_index);
                self.stack[right_index].parent_index = Some(current_index);

                left_child_index = Some(left_index);
                right_child_index = Some(right_index);

                is_nullable =
                    self.stack[left_index].is_nullable || self.stack[right_index].is_nullable;
                first_pos = self.stack[left_index].first_pos.clone();
                first_pos.append(&mut self.stack[right_index].first_pos.clone());
                last_pos = self.stack[right_index].last_pos.clone();
                last_pos.append(&mut self.stack[left_index].last_pos.clone());
                is_accepted_state = false;
            }
            RegexAstElements::Concatenation(ref left, ref right) => {
                element_type = ElementType::Concatenation;

                let left_index = self.create_calculation_stack_for_element(left);
                let right_index = self.create_calculation_stack_for_element(right);

                current_index = self.stack.len();
                self.stack[left_index].parent_index = Some(current_index);
                self.stack[right_index].parent_index = Some(current_index);

                left_child_index = Some(left_index);
                right_child_index = Some(right_index);

                is_nullable =
                    self.stack[left_index].is_nullable && self.stack[right_index].is_nullable;
                first_pos = self.stack[left_index].first_pos.clone();
                if self.stack[left_index].is_nullable {
                    first_pos.append(&mut self.stack[right_index].first_pos.clone());
                }
                last_pos = self.stack[right_index].last_pos.clone();
                if self.stack[right_index].is_nullable {
                    last_pos.append(&mut self.stack[left_index].last_pos.clone());
                }
                is_accepted_state = false;
            }
            RegexAstElements::Leaf(ref group) => {
                element_type = ElementType::Leaf;
                matching_group_index = Some(self.add_matching_group(group));

                current_index = self.stack.len();
                is_nullable = false;
                first_pos = vec![current_index];
                last_pos = vec![current_index];
                is_accepted_state = group == &MatchingGroup::AcceptedState;
            }
            RegexAstElements::ZeroOrMore(ref child) => {
                element_type = ElementType::ZeroOrMore;

                let child_index = self.create_calculation_stack_for_element(child);

                current_index = self.stack.len();
                self.stack[child_index].parent_index = Some(current_index);

                left_child_index = Some(child_index);

                is_nullable = true;
                first_pos = self.stack[child_index].first_pos.clone();
                last_pos = self.stack[child_index].last_pos.clone();
                is_accepted_state = false;
            }
            RegexAstElements::ZeroOrOne(ref child) => {
                element_type = ElementType::ZeroOrOne;

                let child_index = self.create_calculation_stack_for_element(child);

                current_index = self.stack.len();
                self.stack[child_index].parent_index = Some(current_index);

                left_child_index = Some(child_index);

                is_nullable = true;
                first_pos = self.stack[child_index].first_pos.clone();
                last_pos = self.stack[child_index].last_pos.clone();
                is_accepted_state = false;
            }
            _ => panic!("Unknown ast element"),
        }

        self.stack.push(StateCalculations::new(
            current_index,
            left_child_index,
            right_child_index,
            element_type,
            matching_group_index,
            is_nullable,
            first_pos,
            last_pos,
            is_accepted_state,
        ));

        return current_index;
    }

    fn caclulate_follow_pos_for_stack(&mut self) {
        for i in 0..self.stack.len() {
            match self.stack[i].element_type {
                ElementType::Concatenation => {
                    let last_pos_left = &self.stack[self.stack[i].left_child_index.unwrap()]
                        .last_pos
                        .clone();
                    let first_pos_right = &self.stack[self.stack[i].right_child_index.unwrap()]
                        .first_pos
                        .clone();

                    for position in last_pos_left {
                        self.stack[*position]
                            .follow_pos
                            .append(&mut first_pos_right.clone());
                    }
                }
                ElementType::ZeroOrMore => {
                    let last_pos_child = &self.stack[self.stack[i].left_child_index.unwrap()]
                        .last_pos
                        .clone();
                    let first_pos_child = &self.stack[self.stack[i].left_child_index.unwrap()]
                        .first_pos
                        .clone();

                    for position in last_pos_child {
                        self.stack[*position]
                            .follow_pos
                            .append(&mut first_pos_child.clone())
                    }
                }
                _ => {}
            }
        }
    }

    fn convert_to_regex_engine(self) -> RegexEngine {
        let mut deterministic_transitions: StateMachine = HashMap::new();
        let mut deterministic_states = Vec::with_capacity(100);
        let tree_root = &self.stack[self.stack.len() - 1];
        deterministic_states.push(DeterministicState::new(0, tree_root.first_pos.clone()));

        while let Some(unmarked_state_index) =
            self.get_next_unmarked_state_index(&deterministic_states)
        {
            deterministic_states[unmarked_state_index].is_marked = true;

            for matching_group_index in 0..self.matching_groups.len() {
                let mut transition = Vec::new();

                for position in &deterministic_states[unmarked_state_index].non_deterministic_states
                {
                    let non_deterministic_state = &self.stack[position.clone()];
                    if non_deterministic_state.matching_group_index == Some(matching_group_index) {
                        transition.append(&mut non_deterministic_state.follow_pos.clone());
                    }
                }

                if transition.len() != 0 {
                    let state_id = self.get_state_id(&mut deterministic_states, transition);

                    match deterministic_transitions.get_mut(&unmarked_state_index) {
                        Some((transitions, _)) => {
                            transitions.insert(matching_group_index, state_id);
                        }
                        None => {
                            let mut transition_map: TransitionForMatchingGroup =
                                HashMap::with_capacity(10);
                            transition_map.insert(matching_group_index, state_id);

                            let is_accepted = self.contains_accepting_states(
                                &deterministic_states[unmarked_state_index]
                                    .non_deterministic_states,
                            );
                            deterministic_transitions
                                .insert(unmarked_state_index, (transition_map, is_accepted));
                        }
                    }
                }
            }
        }

        for index in 0..deterministic_states.len() {
            match deterministic_transitions.get(&index) {
                None => {
                    let is_accepted = self.contains_accepting_states(
                        &deterministic_states[index].non_deterministic_states,
                    );
                    deterministic_transitions.insert(index, (HashMap::new(), is_accepted));
                }
                _ => {}
            }
        }

        println!("{:#?}", deterministic_transitions);

        return RegexEngine::new_with_values(self.matching_groups, deterministic_transitions);
    }

    fn get_next_unmarked_state_index(&self, states: &Vec<DeterministicState>) -> Option<usize> {
        for index in 0..states.len() {
            if states[index].is_marked == false {
                return Some(index);
            }
        }

        return None;
    }

    fn get_state_id(&self, states: &mut Vec<DeterministicState>, transition: Vec<usize>) -> usize {
        for i in 0..states.len() {
            if states[i].non_deterministic_states == transition {
                return i;
            }
        }

        states.push(DeterministicState::new(states.len(), transition));
        return states.len() - 1;
    }

    fn contains_accepting_states(&self, non_deterministic_states: &Vec<usize>) -> bool {
        for state_id in non_deterministic_states {
            if self.stack[*state_id].accepted {
                return true;
            }
        }

        return false;
    }
}

struct DeterministicState {
    pub(self) index: usize,
    pub(self) non_deterministic_states: Vec<usize>,
    pub(self) is_marked: bool,
}
impl DeterministicState {
    pub fn new(index: usize, non_deterministic_states: Vec<usize>) -> Self {
        DeterministicState {
            index,
            non_deterministic_states,
            is_marked: false,
        }
    }
}

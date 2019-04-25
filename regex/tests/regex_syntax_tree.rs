use regex::get_regex_syntax_tree;
use regex::MatchingGroup;
use regex::MatchingGroupElements;
use regex::RegexAstElements;

#[test]
fn single_leaf() {
    let regex = "a";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );

    assert_eq!(expected_tree, tree);
}

#[test]
fn single_quoted_leaf() {
    let regex = "\"|\"";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('|'))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );

    assert_eq!(expected_tree, tree);
}

#[test]
fn concatenation() {
    let regex = "ab";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn multiple_concatenations() {
    let regex = "abc";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
            )),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('c'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn alternation() {
    let regex = "ab|cd";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Alternation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
            )),
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('c'))),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('d'))),
            )),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn multiple_alternations() {
    let regex = "ab|cd|ef";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Alternation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
            )),
            Box::new(RegexAstElements::Alternation(
                Box::new(RegexAstElements::Concatenation(
                    Box::new(RegexAstElements::Leaf(MatchingGroup::Character('c'))),
                    Box::new(RegexAstElements::Leaf(MatchingGroup::Character('d'))),
                )),
                Box::new(RegexAstElements::Concatenation(
                    Box::new(RegexAstElements::Leaf(MatchingGroup::Character('e'))),
                    Box::new(RegexAstElements::Leaf(MatchingGroup::Character('f'))),
                )),
            )),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn zero_or_more_repetition() {
    let regex = "b*";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::ZeroOrMore(Box::new(RegexAstElements::Leaf(
            MatchingGroup::Character('b'),
        )))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn zero_or_more_repetition_with_noise() {
    let regex = "ab*";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
            Box::new(RegexAstElements::ZeroOrMore(Box::new(
                RegexAstElements::Leaf(MatchingGroup::Character('b')),
            ))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn zero_or_one_repetition() {
    let regex = "ab?";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
            Box::new(RegexAstElements::ZeroOrOne(Box::new(
                RegexAstElements::Leaf(MatchingGroup::Character('b')),
            ))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn one_or_more_repetition() {
    let regex = "ab+";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
            )),
            Box::new(RegexAstElements::ZeroOrMore(Box::new(
                RegexAstElements::Leaf(MatchingGroup::Character('b')),
            ))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn escaped_plus_operator_through_quotes() {
    let regex = "ab\"+\"";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
            )),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('+'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn escaped_concatenation_in_quotes() {
    let regex = "\"a+?\"";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('+'))),
            )),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('?'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn escaped_concatenation_in_quotes_followed_by_normal_regex() {
    let regex = "\"a+?\"a?";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Concatenation(
                    Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
                    Box::new(RegexAstElements::Leaf(MatchingGroup::Character('+'))),
                )),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('?'))),
            )),
            Box::new(RegexAstElements::ZeroOrOne(Box::new(
                RegexAstElements::Leaf(MatchingGroup::Character('a')),
            ))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn group() {
    let regex = "a(bc)d";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Concatenation(
                    Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
                    Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
                )),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('c'))),
            )),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('d'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn group_with_alternation() {
    let regex = "a(b|c)";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
            Box::new(RegexAstElements::Alternation(
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('c'))),
            )),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn multiple_groups_with_multiple_alternations() {
    let regex = "a(b(cd|e)|fg*)h";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
                Box::new(RegexAstElements::Alternation(
                    Box::new(RegexAstElements::Concatenation(
                        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
                        Box::new(RegexAstElements::Alternation(
                            Box::new(RegexAstElements::Concatenation(
                                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('c'))),
                                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('d'))),
                            )),
                            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('e'))),
                        )),
                    )),
                    Box::new(RegexAstElements::Concatenation(
                        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('f'))),
                        Box::new(RegexAstElements::ZeroOrMore(Box::new(
                            RegexAstElements::Leaf(MatchingGroup::Character('g')),
                        ))),
                    )),
                )),
            )),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('h'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );

    assert_eq!(expected_tree, tree);
}

#[test]
fn line_breaks() {
    let regex = "a\\n";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('\n'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn backslach_at_end() {
    let regex = "a\\";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('\\'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn line_break_after_backslash() {
    let regex = "a\\\\n";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('\\'))),
            )),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('\n'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn hexa_characters() {
    let regex = "\\xff";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('ÿ'))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn multiple_hexa_characters() {
    let regex = "\\xff\\xff";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('ÿ'))),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('ÿ'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn incomplete_hexa_characters() {
    let regex = "\\xf";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('\\'))),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('x'))),
            )),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('f'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn character_group() {
    let regex = "[ab]";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Group(vec![
            MatchingGroupElements::Character('a'),
            MatchingGroupElements::Character('b'),
        ]))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn long_character_group() {
    let regex = "[abcdef]";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Group(vec![
            MatchingGroupElements::Character('a'),
            MatchingGroupElements::Character('b'),
            MatchingGroupElements::Character('c'),
            MatchingGroupElements::Character('d'),
            MatchingGroupElements::Character('e'),
            MatchingGroupElements::Character('f'),
        ]))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn character_group_with_range() {
    let regex = "[a-c]";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Group(vec![MatchingGroupElements::Range(
            'a', 'c',
        )]))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn character_group_only_looking_like_range() {
    let regex = "[a-]";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Group(vec![
            MatchingGroupElements::Character('a'),
            MatchingGroupElements::Character('-'),
        ]))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn character_group_with_other_symbols() {
    let regex = "[a-*9#_&%$@!]";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Group(vec![
            MatchingGroupElements::Character('a'),
            MatchingGroupElements::Character('-'),
            MatchingGroupElements::Character('*'),
            MatchingGroupElements::Character('9'),
            MatchingGroupElements::Character('#'),
            MatchingGroupElements::Character('_'),
            MatchingGroupElements::Character('&'),
            MatchingGroupElements::Character('%'),
            MatchingGroupElements::Character('$'),
            MatchingGroupElements::Character('@'),
            MatchingGroupElements::Character('!'),
        ]))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn negative_character_group() {
    let regex = "[^ab]";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::NegativeGroup(vec![
            MatchingGroupElements::Character('a'),
            MatchingGroupElements::Character('b'),
        ]))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn character_group_followed_by_concatenation() {
    let regex = "[ab]c";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Group(vec![
                MatchingGroupElements::Character('a'),
                MatchingGroupElements::Character('b'),
            ]))),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('c'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn character_group_within_multiple_concatenations() {
    let regex = "a[bc]d";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Group(vec![
                    MatchingGroupElements::Character('b'),
                    MatchingGroupElements::Character('c'),
                ]))),
            )),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('d'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);
}

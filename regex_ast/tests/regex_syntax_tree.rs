use regex_ast::MatchingGroup;
use regex_ast::MatchingGroupElements;
use regex_ast::RegexAstElements;
use regex_ast::get_regex_syntax_tree;

#[test]
fn single_leaf() {
    let regex = "a";
    let tree = get_regex_syntax_tree(regex);

    assert_eq!(RegexAstElements::Leaf(MatchingGroup::Character('a')), tree);
}

#[test]
fn concatenation() {
    let regex = "ab";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn multiple_concatenations() {
    let regex = "abc";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('c'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn alternation() {
    let regex = "ab|cd";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Alternation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
        )),
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('c'))),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('d'))),
        )),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn multiple_alternations() {
    let regex = "ab|cd|ef";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Alternation(
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
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn zero_or_more_repetition() {
    let regex = "b*";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::ZeroOrMore(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn zero_or_more_repetition_with_noise() {
    let regex = "ab*";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
        Box::new(RegexAstElements::ZeroOrMore(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
        )),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn zero_or_one_repetition() {
    let regex = "ab?";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
        Box::new(RegexAstElements::ZeroOrOne(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
        )),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn one_or_more_repetition() {
    let regex = "ab+";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
        )),
        Box::new(RegexAstElements::ZeroOrMore(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
        )),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn escaped_plus_operator_through_quotes() {
    let regex = "ab\"+\"";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('+'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn escaped_concatenation_in_quotes() {
    let regex = "\"a+?\"";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('+'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('?'))),
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
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('+'))),
            )),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('?'))),
        )),
        Box::new(RegexAstElements::ZeroOrOne(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
        )),
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
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
            )),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('c'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('d'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn group_with_alternation() {
    let regex = "a(b|c)";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
        Box::new(RegexAstElements::Alternation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('c'))),
        )),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn multiple_groups_with_multiple_alternations() {
    let regex = "a(b(cd|e)|fg*)h";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
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
                    Box::new(RegexAstElements::ZeroOrMore(
                        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('g'))),
                    )),
                )),
            )),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('h'))),
    );

    assert_eq!(expected_tree, tree);
}

#[test]
fn line_breaks() {
    let regex = "a\\n";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('\n'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn backslack_at_end() {
    let regex = "a\\";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('\\'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn line_break_after_backslash() {
    let regex = "a\\\\n";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('\\'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('\n'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn character_group() {
    let regex = "[ab]";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Leaf(MatchingGroup::Group(vec![
        MatchingGroupElements::Character('a'),
        MatchingGroupElements::Character('b'),
    ]));
    assert_eq!(expected_tree, tree);
}

#[test]
fn long_character_group() {
    let regex = "[abcdef]";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Leaf(MatchingGroup::Group(vec![
        MatchingGroupElements::Character('a'),
        MatchingGroupElements::Character('b'),
        MatchingGroupElements::Character('c'),
        MatchingGroupElements::Character('d'),
        MatchingGroupElements::Character('e'),
        MatchingGroupElements::Character('f'),
    ]));
    assert_eq!(expected_tree, tree);
}

#[test]
fn character_group_with_range() {
    let regex = "[a-c]";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Leaf(MatchingGroup::Group(vec![
        MatchingGroupElements::Range('a', 'c'),
    ]));
    assert_eq!(expected_tree, tree);
}

#[test]
fn character_group_only_looking_like_range() {
    let regex = "[a-]";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Leaf(MatchingGroup::Group(vec![
        MatchingGroupElements::Character('a'),
        MatchingGroupElements::Character('-'),
    ]));
    assert_eq!(expected_tree, tree);
}

#[test]
fn character_group_with_other_symbols() {
    let regex = "[a-*9#_&%$@!]";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Leaf(MatchingGroup::Group(vec![
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
    ]));
    assert_eq!(expected_tree, tree);
}

#[test]
fn negative_character_group() {
    let regex = "[^ab]";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Leaf(MatchingGroup::NegativeGroup(vec![
        MatchingGroupElements::Character('a'),
        MatchingGroupElements::Character('b'),
    ]));
    assert_eq!(expected_tree, tree);
}

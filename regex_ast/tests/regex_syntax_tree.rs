use regex_ast::Group;
use regex_ast::RegexAstElements;
use regex_ast::get_regex_syntax_tree;

#[test]
fn single_leaf() {
    let regex = "a";
    let tree = get_regex_syntax_tree(regex);

    assert_eq!(RegexAstElements::Leaf(Group::Character('a')), tree);
}

#[test]
fn concatenation() {
    let regex = "ab";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(Group::Character('a'))),
        Box::new(RegexAstElements::Leaf(Group::Character('b'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn multiple_concatenations() {
    let regex = "abc";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(Group::Character('a'))),
            Box::new(RegexAstElements::Leaf(Group::Character('b'))),
        )),
        Box::new(RegexAstElements::Leaf(Group::Character('c'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn alternation() {
    let regex = "ab|cd";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Alternation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(Group::Character('a'))),
            Box::new(RegexAstElements::Leaf(Group::Character('b'))),
        )),
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(Group::Character('c'))),
            Box::new(RegexAstElements::Leaf(Group::Character('d'))),
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
            Box::new(RegexAstElements::Leaf(Group::Character('a'))),
            Box::new(RegexAstElements::Leaf(Group::Character('b'))),
        )),
        Box::new(RegexAstElements::Alternation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf(Group::Character('c'))),
                Box::new(RegexAstElements::Leaf(Group::Character('d'))),
            )),
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf(Group::Character('e'))),
                Box::new(RegexAstElements::Leaf(Group::Character('f'))),
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
        Box::new(RegexAstElements::Leaf(Group::Character('b'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn zero_or_more_repetition_with_noise() {
    let regex = "ab*";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(Group::Character('a'))),
        Box::new(RegexAstElements::ZeroOrMore(
            Box::new(RegexAstElements::Leaf(Group::Character('b'))),
        )),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn zero_or_one_repetition() {
    let regex = "ab?";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(Group::Character('a'))),
        Box::new(RegexAstElements::ZeroOrOne(
            Box::new(RegexAstElements::Leaf(Group::Character('b'))),
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
            Box::new(RegexAstElements::Leaf(Group::Character('a'))),
            Box::new(RegexAstElements::Leaf(Group::Character('b'))),
        )),
        Box::new(RegexAstElements::ZeroOrMore(
            Box::new(RegexAstElements::Leaf(Group::Character('b'))),
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
            Box::new(RegexAstElements::Leaf(Group::Character('a'))),
            Box::new(RegexAstElements::Leaf(Group::Character('b'))),
        )),
        Box::new(RegexAstElements::Leaf(Group::Character('+'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn escaped_concatenation_in_quotes() {
    let regex = "\"a+?\"";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(Group::Character('a'))),
            Box::new(RegexAstElements::Leaf(Group::Character('+'))),
        )),
        Box::new(RegexAstElements::Leaf(Group::Character('?'))),
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
                Box::new(RegexAstElements::Leaf(Group::Character('a'))),
                Box::new(RegexAstElements::Leaf(Group::Character('+'))),
            )),
            Box::new(RegexAstElements::Leaf(Group::Character('?'))),
        )),
        Box::new(RegexAstElements::ZeroOrOne(
            Box::new(RegexAstElements::Leaf(Group::Character('a'))),
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
                Box::new(RegexAstElements::Leaf(Group::Character('a'))),
                Box::new(RegexAstElements::Leaf(Group::Character('b'))),
            )),
            Box::new(RegexAstElements::Leaf(Group::Character('c'))),
        )),
        Box::new(RegexAstElements::Leaf(Group::Character('d'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn group_with_alternation() {
    let regex = "a(b|c)";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(Group::Character('a'))),
        Box::new(RegexAstElements::Alternation(
            Box::new(RegexAstElements::Leaf(Group::Character('b'))),
            Box::new(RegexAstElements::Leaf(Group::Character('c'))),
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
            Box::new(RegexAstElements::Leaf(Group::Character('a'))),
            Box::new(RegexAstElements::Alternation(
                Box::new(RegexAstElements::Concatenation(
                    Box::new(RegexAstElements::Leaf(Group::Character('b'))),
                    Box::new(RegexAstElements::Alternation(
                        Box::new(RegexAstElements::Concatenation(
                            Box::new(RegexAstElements::Leaf(Group::Character('c'))),
                            Box::new(RegexAstElements::Leaf(Group::Character('d'))),
                        )),
                        Box::new(RegexAstElements::Leaf(Group::Character('e'))),
                    )),
                )),
                Box::new(RegexAstElements::Concatenation(
                    Box::new(RegexAstElements::Leaf(Group::Character('f'))),
                    Box::new(RegexAstElements::ZeroOrMore(
                        Box::new(RegexAstElements::Leaf(Group::Character('g'))),
                    )),
                )),
            )),
        )),
        Box::new(RegexAstElements::Leaf(Group::Character('h'))),
    );

    assert_eq!(expected_tree, tree);
}

#[test]
fn line_breaks() {
    let regex = "a\\n";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(Group::Character('a'))),
        Box::new(RegexAstElements::Leaf(Group::Character('\n'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn backslack_at_end() {
    let regex = "a\\";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(Group::Character('a'))),
        Box::new(RegexAstElements::Leaf(Group::Character('\\'))),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn line_break_after_backslash() {
    let regex = "a\\\\n";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf(Group::Character('a'))),
            Box::new(RegexAstElements::Leaf(Group::Character('\\'))),
        )),
        Box::new(RegexAstElements::Leaf(Group::Character('\n'))),
    );
    assert_eq!(expected_tree, tree);
}

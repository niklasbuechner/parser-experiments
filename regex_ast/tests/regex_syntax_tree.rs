use regex_ast::get_regex_syntax_tree;
use regex_ast::RegexAstElements;

#[test]
fn single_leaf() {
    let regex = "a";
    let tree = get_regex_syntax_tree(regex);

    assert_eq!(RegexAstElements::Leaf('a'), tree);
}

#[test]
fn concatenation() {
    let regex = "ab";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf('a')),
        Box::new(RegexAstElements::Leaf('b')),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn multiple_concatenations() {
    let regex = "abc";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf('a')),
            Box::new(RegexAstElements::Leaf('b')),
        )),
        Box::new(RegexAstElements::Leaf('c')),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn alternation() {
    let regex = "ab|cd";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Alternation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf('a')),
            Box::new(RegexAstElements::Leaf('b')),
        )),
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf('c')),
            Box::new(RegexAstElements::Leaf('d')),
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
            Box::new(RegexAstElements::Leaf('a')),
            Box::new(RegexAstElements::Leaf('b')),
        )),
        Box::new(RegexAstElements::Alternation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf('c')),
                Box::new(RegexAstElements::Leaf('d')),
            )),
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Leaf('e')),
                Box::new(RegexAstElements::Leaf('f')),
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
        Box::new(RegexAstElements::Leaf('b')),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn zero_or_more_repetition_with_noise() {
    let regex = "ab*";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf('a')),
        Box::new(RegexAstElements::ZeroOrMore(
            Box::new(RegexAstElements::Leaf('b')),
        )),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn zero_or_one_repetition() {
    let regex = "ab?";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf('a')),
        Box::new(RegexAstElements::ZeroOrOne(
            Box::new(RegexAstElements::Leaf('b')),
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
            Box::new(RegexAstElements::Leaf('a')),
            Box::new(RegexAstElements::Leaf('b')),
        )),
        Box::new(RegexAstElements::ZeroOrMore(
            Box::new(RegexAstElements::Leaf('b')),
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
            Box::new(RegexAstElements::Leaf('a')),
            Box::new(RegexAstElements::Leaf('b')),
        )),
        Box::new(RegexAstElements::Leaf('+')),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn escaped_concatenation_in_quotes() {
    let regex = "\"a+?\"";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf('a')),
            Box::new(RegexAstElements::Leaf('+')),
        )),
        Box::new(RegexAstElements::Leaf('?')),
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
                Box::new(RegexAstElements::Leaf('a')),
                Box::new(RegexAstElements::Leaf('+')),
            )),
            Box::new(RegexAstElements::Leaf('?')),
        )),
        Box::new(RegexAstElements::ZeroOrOne(
            Box::new(RegexAstElements::Leaf('a')),
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
                Box::new(RegexAstElements::Leaf('a')),
                Box::new(RegexAstElements::Leaf('b')),
            )),
            Box::new(RegexAstElements::Leaf('c')),
        )),
        Box::new(RegexAstElements::Leaf('d')),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn group_with_alternation() {
    let regex = "a(b|c)";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf('a')),
        Box::new(RegexAstElements::Alternation(
            Box::new(RegexAstElements::Leaf('b')),
            Box::new(RegexAstElements::Leaf('c')),
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
            Box::new(RegexAstElements::Leaf('a')),
            Box::new(RegexAstElements::Alternation(
                Box::new(RegexAstElements::Concatenation(
                    Box::new(RegexAstElements::Leaf('b')),
                    Box::new(RegexAstElements::Alternation(
                        Box::new(RegexAstElements::Concatenation(
                            Box::new(RegexAstElements::Leaf('c')),
                            Box::new(RegexAstElements::Leaf('d')),
                        )),
                        Box::new(RegexAstElements::Leaf('e')),
                    )),
                )),
                Box::new(RegexAstElements::Concatenation(
                    Box::new(RegexAstElements::Leaf('f')),
                    Box::new(RegexAstElements::ZeroOrMore(
                        Box::new(RegexAstElements::Leaf('g')),
                    )),
                )),
            )),
        )),
        Box::new(RegexAstElements::Leaf('h')),
    );

    assert_eq!(expected_tree, tree);
}

#[test]
fn line_breaks() {
    let regex = "a\\n";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf('a')),
        Box::new(RegexAstElements::Leaf('\n')),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn backslack_at_end() {
    let regex = "a\\";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf('a')),
        Box::new(RegexAstElements::Leaf('\\')),
    );
    assert_eq!(expected_tree, tree);
}

#[test]
fn line_break_after_backslash() {
    let regex = "a\\\\n";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Leaf('a')),
            Box::new(RegexAstElements::Leaf('\\')),
        )),
        Box::new(RegexAstElements::Leaf('\n')),
    );
    assert_eq!(expected_tree, tree);
}

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

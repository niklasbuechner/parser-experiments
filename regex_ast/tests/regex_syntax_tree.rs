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

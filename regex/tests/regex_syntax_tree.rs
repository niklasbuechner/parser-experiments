use regex::get_regex_engine;
use regex::get_regex_syntax_tree;
use regex::MatchingGroup;
use regex::MatchingGroupElements;
use regex::RegexAstElements;

#[test]
fn matches_single_leaf() {
    let regex = "a";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );

    assert_eq!(expected_tree, tree);

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("a"));
    assert_eq!(false, regex_engine.matches("aa"));
    assert_eq!(false, regex_engine.matches("b"));
}

#[test]
fn match_single_quoted_leaf() {
    let regex = "\"|\"";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('|'))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );

    assert_eq!(expected_tree, tree);

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("|"));
    assert_eq!(false, regex_engine.matches("a|"));
    assert_eq!(false, regex_engine.matches("b"));
}

#[test]
fn match_concatenation() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("ab"));
    assert_eq!(false, regex_engine.matches("a"));
    assert_eq!(false, regex_engine.matches("abc"));
    assert_eq!(false, regex_engine.matches("ba"));
}

#[test]
fn match_multiple_concatenations() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("abc"));
    assert_eq!(false, regex_engine.matches("e"));
    assert_eq!(false, regex_engine.matches("abcd"));
}

#[test]
fn match_alternation() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("ab"));
    assert_eq!(true, regex_engine.matches("cd"));
    assert_eq!(false, regex_engine.matches("bc"));
    assert_eq!(false, regex_engine.matches("abcd"));
}

#[test]
fn match_multiple_alternations() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("ab"));
    assert_eq!(true, regex_engine.matches("cd"));
    assert_eq!(true, regex_engine.matches("ef"));
    assert_eq!(false, regex_engine.matches("bc"));
    assert_eq!(false, regex_engine.matches("abcdef"));
    assert_eq!(false, regex_engine.matches("abcd"));
}

#[test]
fn match_zero_or_more_repetition() {
    let regex = "b*";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::ZeroOrMore(Box::new(
            RegexAstElements::Leaf(MatchingGroup::Character('b')),
        ))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("b"));
    assert_eq!(true, regex_engine.matches("bb"));
    assert_eq!(true, regex_engine.matches("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"));
    assert_eq!(false, regex_engine.matches("a"));
    assert_eq!(false, regex_engine.matches("bbbba"));
    assert_eq!(true, regex_engine.matches(""));
}

#[test]
fn match_zero_or_more_repetition_with_noise() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("a"));
    assert_eq!(true, regex_engine.matches("ab"));
    assert_eq!(true, regex_engine.matches("abbbbbbbbbbbbbbb"));
    assert_eq!(false, regex_engine.matches("b"));
    assert_eq!(false, regex_engine.matches(""));
}

#[test]
fn match_zero_or_one_repetition() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("a"));
    assert_eq!(true, regex_engine.matches("ab"));
    assert_eq!(false, regex_engine.matches("aba"));
    assert_eq!(false, regex_engine.matches("abb"));
}

#[test]
fn match_zero_or_one_repetition_with_noise() {
    let regex = "ab?ab";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Concatenation(
            Box::new(RegexAstElements::Concatenation(
                Box::new(RegexAstElements::Concatenation(
                    Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
                    Box::new(RegexAstElements::ZeroOrOne(Box::new(
                        RegexAstElements::Leaf(MatchingGroup::Character('b')),
                    ))),
                )),
                Box::new(RegexAstElements::Leaf(MatchingGroup::Character('a'))),
            )),
            Box::new(RegexAstElements::Leaf(MatchingGroup::Character('b'))),
        )),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("aab"));
    assert_eq!(true, regex_engine.matches("abab"));
    assert_eq!(false, regex_engine.matches("abbab"));
    assert_eq!(false, regex_engine.matches("abb"));
    assert_eq!(false, regex_engine.matches("aba"));
}

#[test]
fn match_one_or_more_repetition() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("ab"));
    assert_eq!(true, regex_engine.matches("abbbbbbbbbbbbbbb"));
    assert_eq!(false, regex_engine.matches("a"));
    assert_eq!(false, regex_engine.matches(""));
}

#[test]
fn lex_escaped_plus_operator_through_quotes() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("ab+"));
    assert_eq!(false, regex_engine.matches("ab"));
    assert_eq!(false, regex_engine.matches("abb"));
}

#[test]
fn match_escaped_concatenation_in_quotes() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("a+?"));
    assert_eq!(false, regex_engine.matches("aaab"));
    assert_eq!(false, regex_engine.matches("a"));
    assert_eq!(false, regex_engine.matches("a?"));
    assert_eq!(false, regex_engine.matches("+?"));
}

#[test]
fn match_escaped_concatenation_in_quotes_followed_by_normal_regex() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("a+?"));
    assert_eq!(true, regex_engine.matches("a+?a"));
    assert_eq!(false, regex_engine.matches("aaab"));
    assert_eq!(false, regex_engine.matches("a"));
    assert_eq!(false, regex_engine.matches("a?a"));
    assert_eq!(false, regex_engine.matches("+?a"));
}

#[test]
fn match_group() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("abcd"));
    assert_eq!(false, regex_engine.matches("abd"));
    assert_eq!(false, regex_engine.matches("acd"));
    assert_eq!(false, regex_engine.matches("ad"));
    assert_eq!(false, regex_engine.matches(""));
}

#[test]
fn match_group_with_alternation() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("ab"));
    assert_eq!(true, regex_engine.matches("ac"));
    assert_eq!(false, regex_engine.matches("abc"));
    assert_eq!(false, regex_engine.matches("a"));
}

#[test]
fn match_multiple_groups_with_multiple_alternations() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("afh"));
    assert_eq!(true, regex_engine.matches("afgh"));
    assert_eq!(true, regex_engine.matches("afggggggggggggh"));
    assert_eq!(true, regex_engine.matches("abeh"));
    assert_eq!(true, regex_engine.matches("abcdh"));
    assert_eq!(false, regex_engine.matches("abcdeh"));
    assert_eq!(false, regex_engine.matches("abfgh"));
}

#[test]
fn match_line_breaks() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("a\n"));
    assert_eq!(false, regex_engine.matches("a "));
}

#[test]
fn match_backslach_at_end() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("a\\"));
    assert_eq!(false, regex_engine.matches("a\""));
    assert_eq!(false, regex_engine.matches("\\"));
}

#[test]
fn match_line_break_after_backslash() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("a\\\n"));
    assert_eq!(false, regex_engine.matches("a\\"));
    assert_eq!(false, regex_engine.matches("a\\\\n"));
}

#[test]
fn match_hexa_characters() {
    let regex = "\\xff";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Character('ÿ'))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("ÿ"));
    assert_eq!(false, regex_engine.matches("\\xff"));
}

#[test]
fn match_multiple_hexa_characters() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("ÿÿ"));
    assert_eq!(false, regex_engine.matches("\\xff\\xff"));
}

#[test]
fn match_incomplete_hexa_characters() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("\\xf"));
    assert_eq!(false, regex_engine.matches("ÿ"));
}

#[test]
fn match_character_group() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("a"));
    assert_eq!(true, regex_engine.matches("b"));
    assert_eq!(false, regex_engine.matches("ab"));
}

#[test]
fn match_long_character_group() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("a"));
    assert_eq!(true, regex_engine.matches("b"));
    assert_eq!(true, regex_engine.matches("c"));
    assert_eq!(true, regex_engine.matches("d"));
    assert_eq!(true, regex_engine.matches("e"));
    assert_eq!(true, regex_engine.matches("f"));
    assert_eq!(false, regex_engine.matches("abcdef"));
}

#[test]
fn match_character_group_with_range() {
    let regex = "[a-c]";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::Group(vec![
            MatchingGroupElements::Range('a', 'c'),
        ]))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("a"));
    assert_eq!(true, regex_engine.matches("b"));
    assert_eq!(true, regex_engine.matches("c"));
    assert_eq!(false, regex_engine.matches("ac"));
}

#[test]
fn match_character_group_only_looking_like_range() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("a"));
    assert_eq!(true, regex_engine.matches("-"));
    assert_eq!(false, regex_engine.matches("a-"));
}

#[test]
fn match_character_group_with_other_symbols() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("a"));
    assert_eq!(true, regex_engine.matches("-"));
    assert_eq!(true, regex_engine.matches("*"));
    assert_eq!(true, regex_engine.matches("9"));
    assert_eq!(true, regex_engine.matches("#"));
    assert_eq!(true, regex_engine.matches("_"));
    assert_eq!(true, regex_engine.matches("&"));
    assert_eq!(true, regex_engine.matches("%"));
    assert_eq!(true, regex_engine.matches("$"));
    assert_eq!(true, regex_engine.matches("@"));
    assert_eq!(true, regex_engine.matches("!"));
    assert_eq!(false, regex_engine.matches("a-*9#_&%$@!"));
}

#[test]
fn match_negative_character_group() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("c"));
    assert_eq!(true, regex_engine.matches("f"));
    assert_eq!(false, regex_engine.matches("a"));
    assert_eq!(false, regex_engine.matches("ab"));
}

#[test]
fn match_empty_negative_character_group() {
    let regex = "[^]";
    let tree = get_regex_syntax_tree(regex);

    let expected_tree = RegexAstElements::Concatenation(
        Box::new(RegexAstElements::Leaf(MatchingGroup::NegativeGroup(
            Vec::new(),
        ))),
        Box::new(RegexAstElements::Leaf(MatchingGroup::AcceptedState)),
    );
    assert_eq!(expected_tree, tree);

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("c"));
    assert_eq!(true, regex_engine.matches("f"));
    assert_eq!(true, regex_engine.matches("a"));
    assert_eq!(false, regex_engine.matches("ab"));
}

#[test]
fn match_character_group_followed_by_concatenation() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("ac"));
    assert_eq!(true, regex_engine.matches("bc"));
    assert_eq!(false, regex_engine.matches("abc"));
    assert_eq!(false, regex_engine.matches(""));
}

#[test]
fn match_character_group_within_multiple_concatenations() {
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

    let regex_engine = get_regex_engine(regex);
    assert_eq!(true, regex_engine.matches("abd"));
    assert_eq!(true, regex_engine.matches("acd"));
    assert_eq!(false, regex_engine.matches("abcd"));
    assert_eq!(false, regex_engine.matches("ad"));
}

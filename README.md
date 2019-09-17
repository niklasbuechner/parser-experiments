# Parser experiments
These are my experiments in order to better understand how state machines, lexer and parsers work.

## Comparison lexer
This is my first ever hand written lexer. It is only meant to be able to lex comparison operators such as
`<= <> >= = <= >= < >`.

## ID reserved words lexer
These two lexers are also hand written lexers. I created them to understand the difference between a lexer which
detects keywords through its state machine and lexers which detect any id and then check for keywords afterwards.
The two are very interesting when comparing the code size of the two while also keeping in mind that the checking
of keywords after the state machine leads to checking these characters at least twice if not multiple times for
one keyword.

## `lex-test` and `lexer-with-dynamic-state-machine`
The two folders are purely theoretical. The first one contains a bison file and the source code which bison
generates. The files are meant to be an easy grammar to lex in order for me to understand the code bison
generates for a lexer. The second folder contains a file with all the features a good lexer generator needs
to support in order for it to have a practical application.

## Regex
The `regex/` folder contains a very small regex engine. I created this project to better understand how regexes
work and how they can be so efficient.

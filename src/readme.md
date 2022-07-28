# Stopped

- I stopped at `Longer Lexems` section

- I should re-look at nand to tetris

# Notes

- lookahead - basically how far ahead your parser will have to look in order to figure out a specific token

- maximal munch - what actually is this? I didn't really pay attention to it. I mean basically what it was saying is that whenever we are looking at a string of characters, and trying to determine what it is - what we do is - pick the on the that is the longest match - i.e. keyword or identifier

# High level stuff

- A compiler has a layered architecture - that is that it actually has multiple parts - which are?

there's actually more than one way to do this, but this is just one way.

1. scanning / lexing - this is basically where we take the raw characters from a source file, and transform them into tokens.

2. Parsing - this step is taking the tokens and producing AST / syntax trees from them. What exactly is an AST? Well I'm gonna figure out

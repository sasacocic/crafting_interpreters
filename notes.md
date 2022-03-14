# scanning

- scanning is the part where we basically do stuff.

## Lexmes and Tokens

```lox
var language = "lox";
```

That's hat lexical analysis is about. Our job is to scan through the list of characters and group them together in to the smallest sequences that still represent something. Each of these blobs of characters is called a `lexeme`. e.g. the lexems here are

`var` - `language` - `=` - `"lox"` `;`

When we take the lexeme and bundle it together with that other data, the result is a token. It includes useful stuff like

**_Token Type_**

Keywords are part of the sahp of the langauge's grammar, so they parser often has code like, "if the next token is while then do...." That means the parser wants to know not just that it has a lexeme for some identifier, but that it has a reserved word, and which keyword it is.

**_Regular Languages and Expression_**

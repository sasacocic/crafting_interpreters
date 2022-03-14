#ifndef SCANNER_H
#define SCANNER_H
/*
need to see if I can seperate this enum into another file
and import it for use here. Basically it just takes up too much
space here right now
*/

enum TokenType
{
  // single-character tokens
  LEFT_PAREN,
  RIGHT_PAREN,
  LEFT_BRACE,
  RIGHT_BRACE,
  COMMA,
  DOT,
  MINUS,
  PLUS,
  SEMICOLON,
  SLASH,
  STAR,
  // one or two character tokens
  BANG,
  BANG_EQUAL,
  EQUAL,
  EQUAL_EQUAL,
  GREATER,
  GREATER_EQUAL,
  LESS,
  LESS_EQUAL,
  // literals
  IDENTIFIER,
  STRING,
  NUMBER,
  // keywords
  AND,
  CLASS,
  ELSE,
  FALSE,
  FUN,
  FOR,
  IF,
  NIL,
  OR,
  PRINT,
  RETURN,
  SUPER,
  THIS,
  TRUE,
  VAR,
  WHILE,
  EOFF
};

struct Token_t
{
  enum TokenType type;
  char *lexme;
  char *literal;
  int line;
};
typedef struct Token_t Token;

struct Tokens_t
{
  Token *tokens;
  int tokens_len;
  int used;
};
typedef struct Tokens_t Tokens;

Token *new_token(enum TokenType type, char *lexeme, char *literal, int line);
void print_token(Token *token);
void print_tokens(Tokens *tokens);

Tokens *new_tokens();
int add_token(Tokens *tokens, Token *token);

/* ----- SCANNER ------ */
// an array of tokens or a list of tokens basically the same thing right.
// * is a pointer to a token, but a ** would work here or ... *[]
// basically I want to dynamically allocate the tokes as they come
// in so....
struct Scanner_t
{
  char *source;
  int source_length;
  Tokens *tokens;
};

typedef struct Scanner_t Scanner;
Scanner *new_scanner(char *string);
void scan_tokens(Scanner *scanner);

#endif
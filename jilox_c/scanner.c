#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <stdbool.h>
#include "scanner.h"
#include "errors.h"

/* ---- TODOS -----
- do better error handling... right now it's pretty weak.
-
*/

int start = 0;
int current = 0;
int line = 1;

Token *new_token(enum TokenType type, char *lexeme, char *literal, int line)
{
  // create on the heap, because pretty sure if you create it
  // on the stack it will disappear eventually
  Token *t = (Token *)malloc(sizeof(Token)); // is this the right size?
  t->type = type;
  t->lexme = (char *)malloc(sizeof(char) * strlen(lexeme) + 1);
  strcpy(t->lexme, lexeme);
  if (literal != NULL)
  {
    t->literal = (char *)malloc(sizeof(char) * strlen(literal) + 1);
    strcpy(t->literal, literal);
  }
  t->line = line;
  return t; // return t right not the address of t? which maybe would be the same thing?
}

void print_token(Token *token)
{
  printf("Token {\n  lexem: %s\n  literal: %s\n  line: %d\n}\n", token->lexme, token->literal, token->line);
}

void token_to_string(Token *token)
{
  printf("%u %s %s", token->type, token->lexme, token->literal);
}

Tokens *new_tokens()
{
  Tokens *t = (Tokens *)malloc(sizeof(Tokens));
  t->tokens = NULL;
  t->tokens_len = 0;
  t->used = 0;
  return t;
}

int add_token(Tokens *tokens, Token *token)
{
  // just add to the end
  // tokens->tokens starts out as NULL...

  printf("checking if 'tokens' is null: %p \n", tokens);
  printf("checking 'tokens->tokens' is null: %p \n", tokens->tokens);
  if (tokens->tokens == NULL)
  {
    printf("checking null case");
    tokens->tokens = (Token *)malloc(sizeof(Token) * 1);
    tokens->tokens_len = 1;
  }
  else if (tokens->tokens_len == tokens->used)
  {

    printf("checking length case");
    tokens->tokens = realloc(tokens->tokens, sizeof(Token) * (tokens->tokens_len * 2));
    tokens->tokens_len = tokens->tokens_len * 2;
  }

  tokens->tokens[tokens->used] = *token;
  tokens->used += 1;

  return 0;
}

void print_tokens(Tokens *tokens)
{
  for (int i = 0; i < tokens->used; i++)
  {
    // printf("printing token: [%d]\naddress: %p\n", i, tokens->tokens);
    print_token(&tokens->tokens[i]);
  }
}

// it's like what the fuck am I working with. Make the thing
// that I'm working with easy, and things will become easier
Scanner *new_scanner(char *source)
{
  // shuoldn't I be allocating enough of a Scanner and not a scanner pointer?
  Scanner *scanner = (Scanner *)malloc(sizeof(Scanner));
  // needs to be the size of the source
  scanner->source = (char *)malloc(sizeof(char) * strlen(source));
  scanner->tokens = new_tokens();
  // copy source in scanner->source
  strcpy(scanner->source, source);
  scanner->source_length = strlen(source); // ... need to test this if it'll work with multi-line strings

  return scanner;
}

// basically need to put in the 'function' inside the struct somehow

void free_scanner(Scanner *scanner)
{
  // TODO: free the scanner
}

bool isAtEnd(Scanner *scanner)
{
  // printf("current / scanner length: %d / %d\n", current, scanner->source_length);
  return current >= scanner->source_length;
}

char advance(Scanner *scanner)
{
  char cur = scanner->source[current];
  char print_helper[] = {cur, '\0'};
  printf("scanner has been advacned to: %s\n", print_helper);
  current++;
  return cur;
}

void addTokenHelper(Scanner *scanner, enum TokenType type, char *literal)
{
  char text[(current - start) + 1]; // assuming currnt - start will always be pos.
  // TODO: make sure this actually works
  memcpy(text, &scanner->source[start], current - start);
  text[current - start] = '\0'; // need to add null terminator
  add_token(scanner->tokens, new_token(type, text, literal, line));
}

void addToken(Scanner *scanner, enum TokenType type)
{
  addTokenHelper(scanner, type, NULL);
}

bool match(Scanner *scanner, char expected)
{
  if (isAtEnd(scanner))
  {
    return false;
  }
  if (scanner->source[current] != expected)
  {
    return false;
  }
  current++;
  return true;
}

char peek(Scanner *scanner)
{
  if (isAtEnd(scanner))
  {
    return '\0';
  }
  return scanner->source[current];
}

void string(Scanner *scanner)
{
  while (peek(scanner) != '"' && !isAtEnd(scanner))
  {
    if (peek(scanner) == '\n')
    {
      line++;
    }
    advance(scanner);
  }

  if (isAtEnd(scanner))
  {
    // Lox.error
    fprintf(stderr, "%s", "Unterminated string.");
    return;
  }

  // The closing "
  advance(scanner);

  // Trim the surrounding quotes
  // is this a problem becuase not static?
  char *value = (char *)malloc(sizeof(char) * (current + 1 - start));
  value[current - start] = '\0';

  // TODO: trim " " from beginning and end of string
  memcpy(value, &scanner->source[start], current - start);

  addTokenHelper(scanner, STRING, value);
  free(value); // should free value I imagine since well I'm not using it after this. Also stack would probs be fine....
}

char peekNext(Scanner *scanner)
{
  if (current + 1 >= strlen(scanner->source))
  {
    return '\0';
  }
  return scanner->source[current + 1];
}

bool isDigit(char c)
{
  return c >= '0' && c <= '9';
}

void number(Scanner *scanner)
{
  while (isDigit(peek(scanner)))
  {

    advance(scanner);
  }

  if (peek(scanner) == '.' && isDigit(peekNext(scanner)))
  {
    // consumer '.'
    advance(scanner);
    while (isDigit(peek(scanner)))
    {
      advance(scanner);
    }
  }

  char num[10];
  memcpy(num, scanner->source, current - start);
  // TODO: num here should be a double
  // probs need to write a method to do this.
  addTokenHelper(scanner, NUMBER, num);
}

bool isAlpha(char c)
{
  return (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_';
}

bool isAlphaNumberic(char c)
{
  return isAlpha(c) || isDigit(c);
}

// static in c ???

struct Tuple_t
{
  char *keyword_string;
  enum TokenType type;
};
typedef struct Tuple_t Tuple;

Tuple keywords[] = {
    {"and", AND},
    {"class", CLASS},
    {"else", ELSE},
    {"false", FALSE},
    {"for", FOR},
    {"fun", FUN},
    {"if", IF},
    {"nil", NIL},
    {"or", OR},
    {"print", PRINT},
    {"return", RETURN},
    {"super", SUPER},
    {"this", THIS},
    {"true", TRUE},
    {"var", VAR},
    {"while", WHILE},
};
const int KEYWORDS_COUNT = 16;

void identifier(Scanner *scanner)
{
  while (isAlphaNumberic(peek(scanner)))
  {
    advance(scanner);
  }

  printf("current: %d & start: %d \n", current, start);
  char *text = (char *)malloc(sizeof(char) * (current - start) + 1); // needs to be free'd after use
  memcpy(text, &scanner->source[start], current - start);
  // text[current - start] = '\0';

  // for (int j = 0; j < (current - start) + 1; j++)
  // {
  //   printf("%c\n", text[j]);
  // }
  // printf("text: %s \n", text);
  enum TokenType token_type = IDENTIFIER;
  for (int i = 0; i < KEYWORDS_COUNT; i++)
  {
    if (strcmp(keywords[i].keyword_string, text) == 0)
    {
      token_type = keywords[i].type;
      break;
    }
    // printf();
  }

  // printf("out");
  addToken(scanner, token_type);
}

void scan_token(Scanner *scanner, int *current)
{
  // bool (*func_pt)(Scanner*)()
  char c = advance(scanner);
  char print_helper[] = {c, '\0'};
  printf("looking at char: %s \n", print_helper);
  switch (c)
  {
  case '(':
    addToken(scanner, LEFT_PAREN);
    break;
  case ')':
    addToken(scanner, RIGHT_PAREN);
    break;
  case '{':
    addToken(scanner, LEFT_BRACE);
    break;
  case '}':
    addToken(scanner, RIGHT_BRACE);
    break;
  case ',':
    addToken(scanner, COMMA);
    break;
  case '.':
    addToken(scanner, DOT);
    break;
  case '-':
    addToken(scanner, MINUS);
    break;
  case '+':
    addToken(scanner, PLUS);
    break;
  case ';':
    addToken(scanner, SEMICOLON);
    break;
  case '*':
    addToken(scanner, STAR);
    break;
  case '!':
    addToken(scanner, match(scanner, '=') ? BANG_EQUAL : BANG);
    break;
  case '=':
    addToken(scanner, match(scanner, '=') ? EQUAL_EQUAL : EQUAL);
    break;
  case '<':
    addToken(scanner, match(scanner, '=') ? LESS_EQUAL : LESS);
    break;
  case '>':
    addToken(scanner, match(scanner, '=') ? GREATER_EQUAL : GREATER);
    break;
  case '/':
    if (match(scanner, '/'))
    {
      while (peek(scanner) != '\n' && !isAtEnd(scanner))
      {
        advance(scanner);
      }
    }
    else
    {
      addToken(scanner, SLASH);
    }
    break;
  case ' ':
  case '\r':
  case '\t':
    // Ignore whitespace
    break;
  case '\n':
    line++;
    break;
  case '"':
    string(scanner);
    break;
  default:
    if (isDigit(c))
    {
      number(scanner);
    }
    else if (isAlpha(c))
    {
      identifier(scanner);
    }
    else
    {
      print_error(line, "Unexpected character");
    }
    break;
    // TODO: I shuold be using the error handling thing we have, but not gonna do that.
    // Just gonna print.
    printf("there's an error reading the lox file or prompt you gave");
  }
}

void scan_tokens(Scanner *scanner)
{
  // basically should scan the tokens and get all of them
  while (!isAtEnd(scanner))
  {
    // printf("not at end\n");
    start = current;
    scan_token(scanner, &current);
  }

  // need to add to the array that is tokens
  add_token(scanner->tokens, new_token(EOF, "", NULL, line));
}
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <readline/readline.h>

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
  COMMAN,
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

/* ---- TODOS -----
- do better error handling... right now it's pretty weak.
-
*/

/* ---- Figure Out -----

- how does C traditionally do error handling?

*/

enum errors
{
  SYNTAX_ERROR,
  GENERAL_ERROR
};

// what does static do in C?

void report(int line, char *where, char *message)
{
  printf("[line %s ] Error %s : %s", line, where, message);
}

// take in error?
void print_error(int line, char *message)
{

  report(line, "where?", message);
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
};
typedef struct Tokens_t Tokens;

Token *new_token(enum TokenType type, char *lexeme, char *literal, int line)
{
  // create on the heap, because pretty sure if you create it
  // on the stack it will disappear eventually

  Token *t = (Token *)malloc(sizeof(Token)); // is this the right size?
  t->type = type;
  t->lexme = lexeme;
  t->literal = literal;
  t->line = line;
  return t; // return t right not the address of t? which maybe would be the same thing?
}

void token_to_string(Token *token)
{
  printf("%s %s %s", token->type, token->lexme, token->literal);
}

Tokens *new_tokens()
{
  Tokens *t = (Tokens *)malloc(sizeof(Tokens));
  printf("token location: %p \n", t);
  t->tokens = NULL;
  t->tokens_len = 0;
  return t;
}

int tokenize_input(Tokens *tokens, char *source)
{
  printf("tokens in toke.input: %p \n", tokens);
  printf("source in toke.input %p \n", source);

  return 0;
}

int run(char *source)
{

  printf("source in run %p \n", source);
  // tokes should be an array of string pointers
  Tokens *tokens = new_tokens();
  if (tokenize_input(tokens, source) != -1)
  {

    for (int i = 0; i < tokens->tokens_len; i++)
    {
      printf("token: %s", tokens->tokens[i].lexme);
    }
  }
  else
  {
    return -1;
  }

  return 0;
}

int runPrompt()
{
  // this should be a repl. Basically it should read the line
  // and execute the statement that we got

  char thingRead[500];
  while (1)
  {
    strcpy(thingRead, readline("> "));
    if (strncmp(thingRead, ".exit", 5) == 0)
    {
      break;
    }
    printf("read: %s \n", thingRead);
    // I wonder if printing this will mess up the cursor?
    // shouldn't right? It's an array.
    // unsure if i'll actually need to return something from
    // run to use it here...
    //
    if (run(thingRead) != -1)
    {
      print_error(0, "there's an error in your program, and I don't know which one");
    } // TODO: depending on what happens on run need to handle that.
  }

  return 0;
}

int runFile(char *fileName)
{
  printf("reading %s \n", fileName);
  FILE *fp = fopen(fileName, "r");

  if (fseek(fp, 0l, SEEK_END) != 0)
  {
    return 1; // bad this is an error ... should probs use enums
  }
  long file_size = ftell(fp);
  char *file = (char *)malloc(file_size + 1); // add 1 for null terminator
  fseek(fp, 0L, SEEK_SET);

  printf("size of file: %lu \n", file_size);
  printf("file contents is: %s \n", file);

  printf("here is the files contents\n");

  if (1 != fread(file, file_size, 1, fp))
  {
    fclose(fp);
    exit(9);
  }

  printf("read: %s \n", file);

  // int c;
  // while ((c = fgetc(fp)) != EOF)
  //{
  //   putc(c, stdout);
  // }
  fclose(fp);
  int result = run(file);
  // might want to do something here depending on the result
  free(file);
  return result;
}

int main(int argc, char *argv[])
{

  printf("argc: %d, argv:", argc);
  for (int i = 0; i < argc; i++)
  {
    printf("%s ", argv[i]);
  }
  printf("\n");

  if (argc < 2)
  {
    printf("Usage: jlox [script]\n");
    exit(1);
  }
  else if (argc == 2)
  {
    printf("running jlox file %s \n", argv[1]);
    runFile(argv[1]);
  }
  else
  {
    runPrompt();
  }

  //  FILE *pointer_to_file_stream = fopen("readme.md", "r");
  //
  //  if (pointer_to_file_stream == NULL)
  //  {
  //    printf("there's an error");
  //  }
  //
  //  printf("size of pointer: %lu, size of file?: %lu", sizeof(pointer_to_file_stream), sizeof(*pointer_to_file_stream));
}
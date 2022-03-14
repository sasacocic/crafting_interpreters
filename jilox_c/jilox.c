#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <readline/readline.h>
#include "scanner.h"
#include "errors.h"
#include <stdbool.h>

/* ---- Figure Out -----
- traditional error handling in C?
*/

int run(char *source)
{
  // TODO: this basically doesn't work........
  printf("creating scanner\n");
  Scanner *scanner = new_scanner(source);
  printf("created scanner\n");

  printf("scanning tokens now\n");
  scan_tokens(scanner);
  printf("all tokens scanned\n");
  // Tokens *tokens = new_tokens(); // new_tokens -> new Scanner
  for (int i = 0; i < scanner->tokens->used; i++)
  {
    printf("token: %s\n", ((scanner->tokens->tokens) + i)->lexme);
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
  // printf("here is the files contents\n");

  if (fread(file, file_size, 1, fp) != 1)
  {
    fclose(fp);
    exit(9);
  }

  printf("file contents is:\n ---- \n%s\n ---- \n", file);
  // printf("read: %s \n", file);

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

// just being used by me to mess around
// int testing()
// {
//   Tokens *toks = new_tokens();
//   Token *toke = new_token(RIGHT_BRACE, "test-lexeme", "test-literal", 2);
//   add_token(toks, toke);
//   add_token(toks, new_token(RIGHT_BRACE, "test-lexeme", "test-literal", 3));
//   add_token(toks, new_token(RIGHT_BRACE, "test-lexeme", "test-literal", 4));
//   add_token(toks, new_token(RIGHT_BRACE, "test-lexeme", "test-literal", 5));
//   add_token(toks, new_token(RIGHT_BRACE, "test-lexeme", "test-literal", 6));
//   printf("tokens used: %d, tokens len: %d\n", toks->used, toks->tokens_len);
//   print_tokens(toks);
//   exit(0);
// }

int main(int argc, char *argv[])
{
  // testing();
  printf("argc: %d, argv:", argc);
  for (int i = 0; i < argc; i++)
  {
    printf("%s ", argv[i]);
  }
  printf("\n");

  if (argc < 2)
  {
    // TODO: I should be printing to stderr
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
}
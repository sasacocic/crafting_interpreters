#include <stdio.h>

void report(int line, char *where, char *message)
{
  printf("[line %d ] Error %s : %s", line, where, message);
}

// take in error?
void print_error(int line, char *message)
{

  report(line, "where?", message);
};
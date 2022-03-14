enum errors
{
  SYNTAX_ERROR,
  GENERAL_ERROR
};

void report(int line, char *where, char *message);
void print_error(int line, char *message);
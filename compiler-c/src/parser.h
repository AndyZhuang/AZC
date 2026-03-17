#ifndef AZC_PARSER_H
#define AZC_PARSER_H

#include "lexer.h"
#include "ast.h"
#include <stdbool.h>

typedef struct Parser Parser;

Parser* parser_create(Lexer* lexer);
void parser_destroy(Parser* parser);
AST* parser_parse(Parser* parser);
bool parser_had_error(Parser* parser);

#endif // AZC_PARSER_H
#include <stdio.h>
#include "lexer.h"

int main() {
    const char* source = "let x = 10";
    Lexer* lexer = lexer_create(source);
    Token* tokens = lexer_tokenize(lexer);
    
    printf("Source: '%s'\n", source);
    printf("Tokens:\n");
    for (size_t i = 0; i < lexer->token_count; i++) {
        Token* t = &tokens[i];
        printf("  [%d:%d] type=%d (%s)", t->line, t->column, t->type, token_type_name(t->type));
        if (t->lexeme) printf(" lexeme='%s'", t->lexeme);
        printf("\n");
    }
    
    lexer_destroy(lexer);
    return 0;
}
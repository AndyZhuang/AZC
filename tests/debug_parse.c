#include <stdio.h>
#include <stdlib.h>
#include "lexer.h"
#include "ast.h"

int main() {
    const char* source = "let x = 10";
    
    printf("Source: '%s'\n\n", source);
    
    Lexer* lexer = lexer_create(source);
    Token* tokens = lexer_tokenize(lexer);
    
    printf("Tokens (%zu):\n", lexer->token_count);
    for (size_t i = 0; i < lexer->token_count; i++) {
        Token* t = &tokens[i];
        printf("  [%d:%d] type=%d", t->line, t->column, t->type);
        if (t->lexeme) printf(" lexeme='%s'", t->lexeme);
        printf("\n");
    }
    
    lexer_destroy(lexer);
    
    return 0;
}

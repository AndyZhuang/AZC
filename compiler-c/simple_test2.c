#include <stdio.h>
#include <stdlib.h>
#include "src/lexer.h"
#include "src/ast.h"

int main() {
    const char* source = "let x = 10";
    Lexer* lexer = lexer_create(source);
    lexer_tokenize(lexer);
    printf("Tokens: %zu\n", lexer->token_count);
    for (int i = 0; i < lexer->token_count; i++) {
        printf("  [%d] type=%d lexeme=%s\n", i, lexer->tokens[i].type, 
               lexer->tokens[i].lexeme ? lexer->tokens[i].lexeme : "(null)");
    }
    lexer_destroy(lexer);
    printf("OK\n");
    return 0;
}

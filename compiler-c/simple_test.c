#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lexer.h"
#include "ast.h"
#include "parser.h"

int main() {
    printf("Testing...\n");
    fflush(stdout);
    
    const char* source = "let x = 10";
    Lexer* lexer = lexer_create(source);
    lexer_tokenize(lexer);
    
    printf("Tokens: %zu, tokens ptr: %p\n", lexer->token_count, (void*)lexer->tokens);
    printf("Token 0 type: %d, Token 4 (EOF) type: %d\n", lexer->tokens[0].type, lexer->tokens[4].type);
    fflush(stdout);
    
    Parser* parser = parser_create(lexer);
    
    printf("Starting parse...\n");
    fflush(stdout);
    AST* ast = parser_parse(parser);
    
    printf("AST statements: %d\n", ast->stmt_count);
    
    parser_destroy(parser);
    lexer_destroy(lexer);
    ast_destroy(ast);
    
    printf("Done!\n");
    return 0;
}

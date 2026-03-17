#include <stdio.h>
#include <stdlib.h>
#include "src/lexer.h"
#include "src/ast.h"
#include "src/parser.h"

int main() {
    const char* source = "let x = 10";
    printf("Source: %s\n", source);
    
    Lexer* lexer = lexer_create(source);
    lexer_tokenize(lexer);
    printf("Tokens: %zu\n", lexer->token_count);
    
    printf("Creating parser...\n");
    Parser* parser = parser_create(lexer);
    
    printf("Parsing...\n");
    AST* ast = parser_parse(parser);
    
    printf("AST statements: %d\n", ast->stmt_count);
    
    parser_destroy(parser);
    lexer_destroy(lexer);
    ast_destroy(ast);
    
    printf("Done!\n");
    return 0;
}

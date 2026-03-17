#include <stdio.h>
#include <stdlib.h>
#include "src/lexer.h"
#include "src/ast.h"
#include "src/parser.h"
#include "src/codegen.h"

int main() {
    const char* source = "let x = 10\nlet y = 20";
    printf("Source:\n%s\n\n", source);
    
    Lexer* lexer = lexer_create(source);
    lexer_tokenize(lexer);
    printf("Tokens: %zu\n", lexer->token_count);
    
    Parser* parser = parser_create(lexer);
    AST* ast = parser_parse(parser);
    printf("AST statements: %d\n", ast->stmt_count);
    
    FILE* out = fopen("/tmp/azc_test.c", "w");
    Codegen* cg = codegen_create(out);
    codegen_gen_program(cg, ast);
    fclose(out);
    
    printf("\n=== Generated C code ===\n");
    system("cat /tmp/azc_test.c");
    
    codegen_destroy(cg);
    parser_destroy(parser);
    lexer_destroy(lexer);
    ast_destroy(ast);
    
    printf("\nDone!\n");
    return 0;
}

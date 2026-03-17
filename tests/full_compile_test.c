#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lexer.h"
#include "ast.h"
#include "parser.h"
#include "codegen.h"

void compile_and_run(const char* source) {
    printf("=== Compiling ===\n%s\n\n", source);
    
    Lexer* lexer = lexer_create(source);
    lexer_tokenize(lexer);
    
    printf("Tokens: %zu\n", lexer->token_count);
    
    Parser* parser = parser_create(lexer);
    AST* ast = parser_parse(parser);
    
    printf("AST statements: %d\n\n", ast->stmt_count);
    
    FILE* output = fopen("/tmp/azc_output.c", "w");
    Codegen* codegen = codegen_create(output);
    codegen_gen_program(codegen, ast);
    fclose(output);
    
    printf("=== Generated C code ===\n");
    system("cat /tmp/azc_output.c");
    
    printf("\n=== Compiling to executable ===\n");
    int result = system("gcc -o /tmp/azc_test /tmp/azc_output.c 2>&1");
    
    if (result == 0) {
        printf("\n=== Running ===\n");
        system("/tmp/azc_test");
    }
    
    codegen_destroy(codegen);
    parser_destroy(parser);
    lexer_destroy(lexer);
    ast_destroy(ast);
}

int main() {
    printf("AZC C Compiler - Full Pipeline Test\n");
    printf("====================================\n\n");
    
    const char* test1 = "let x = 10";
    compile_and_run(test1);
    
    printf("\n\n");
    
    const char* test2 = "let x = 10\nlet y = 20";
    compile_and_run(test2);
    
    printf("\n\n");
    
    const char* test3 = "let x = 10 + 5";
    compile_and_run(test3);
    
    return 0;
}
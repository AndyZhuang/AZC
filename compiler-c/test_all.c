#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "src/lexer.h"
#include "src/ast.h"
#include "src/parser.h"
#include "src/codegen.h"

void test(const char* name, const char* source) {
    printf("=== Test: %s ===\n", name);
    printf("Source: %s\n", source);
    
    Lexer* lexer = lexer_create(source);
    lexer_tokenize(lexer);
    
    Parser* parser = parser_create(lexer);
    AST* ast = parser_parse(parser);
    
    FILE* out = fopen("/tmp/azc_out.c", "w");
    Codegen* cg = codegen_create(out);
    codegen_gen_program(cg, ast);
    fclose(out);
    
    printf("AST stmts: %d\n", ast->stmt_count);
    
    system("cat /tmp/azc_out.c");
    printf("\n");
    
    codegen_destroy(cg);
    parser_destroy(parser);
    lexer_destroy(lexer);
    ast_destroy(ast);
}

int main() {
    test("Simple let", "let x = 10");
    test("Two lets", "let x = 10\nlet y = 20");
    test("Addition", "let x = 10 + 5");
    test("Multiplication", "let x = 3 * 4");
    test("Chained add", "let x = 1 + 2 + 3");
    test("All ops", "let x = 1 + 2 * 3 - 4");
    test("Identifier", "let name = \"test\"");
    test("Boolean", "let flag = true");
    
    return 0;
}

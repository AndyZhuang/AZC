#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "src/lexer.h"
#include "src/ast.h"
#include "src/parser.h"
#include "src/codegen.h"

int main(int argc, char** argv) {
    const char* filename = argv[1] ? argv[1] : "/tmp/test1.azc";
    FILE* f = fopen(filename, "r");
    if (!f) { printf("Cannot open %s\n", filename); return 1; }
    
    fseek(f, 0, SEEK_END);
    long len = ftell(f);
    fseek(f, 0, SEEK_SET);
    
    char* source = malloc(len + 1);
    fread(source, 1, len, f);
    source[len] = '\0';
    fclose(f);
    
    printf("=== Source ===\n%s\n\n", source);
    
    Lexer* lexer = lexer_create(source);
    lexer_tokenize(lexer);
    printf("Tokens: %zu\n", lexer->token_count);
    
    Parser* parser = parser_create(lexer);
    AST* ast = parser_parse(parser);
    printf("AST statements: %d\n", ast->stmt_count);
    
    FILE* out = fopen("/tmp/azc_out.c", "w");
    Codegen* cg = codegen_create(out);
    codegen_gen_program(cg, ast);
    fclose(out);
    
    printf("\n=== Generated C ===\n");
    system("cat /tmp/azc_out.c");
    
    return 0;
}

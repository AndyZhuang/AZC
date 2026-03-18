#define _POSIX_C_SOURCE 200809L

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lexer.h"
#include "ast.h"
#include "parser.h"
#include "codegen.h"

static void print_usage(const char* prog) {
    printf("AZC Compiler v0.6.0\n");
    printf("Usage: %s [options] <file.azc>\n", prog);
    printf("Options:\n");
    printf("  -o <file>    Output file (default: stdout)\n");
    printf("  -c           Compile only, don't assemble\n");
    printf("  -h           Show this help\n");
}

int main(int argc, char** argv) {
    if (argc < 2) {
        print_usage(argv[0]);
        return 1;
    }
    
    const char* input_file = NULL;
    const char* output_file = NULL;
    int compile_only = 0;
    
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "-o") == 0 && i + 1 < argc) {
            output_file = argv[++i];
        } else if (strcmp(argv[i], "-c") == 0) {
            compile_only = 1;
        } else if (strcmp(argv[i], "-h") == 0) {
            print_usage(argv[0]);
            return 0;
        } else if (argv[i][0] != '-') {
            input_file = argv[i];
        }
    }
    
    if (!input_file) {
        fprintf(stderr, "Error: No input file specified\n");
        return 1;
    }
    
    FILE* f = fopen(input_file, "r");
    if (!f) {
        fprintf(stderr, "Error: Cannot open file '%s'\n", input_file);
        return 1;
    }
    
    fseek(f, 0, SEEK_END);
    long len = ftell(f);
    fseek(f, 0, SEEK_SET);
    
    char* source = malloc(len + 1);
    fread(source, 1, len, f);
    source[len] = '\0';
    fclose(f);
    
    Lexer* lexer = lexer_create(source);
    lexer_tokenize(lexer);
    
    Parser* parser = parser_create(lexer);
    AST* ast = parser_parse(parser);
    
    FILE* out = stdout;
    if (output_file) {
        out = fopen(output_file, "w");
        if (!out) {
            fprintf(stderr, "Error: Cannot create output file '%s'\n", output_file);
            return 1;
        }
    }
    
    Codegen* cg = codegen_create(out);
    codegen_gen_program(cg, ast);
    
    if (output_file) fclose(out);
    
    codegen_destroy(cg);
    parser_destroy(parser);
    lexer_destroy(lexer);
    ast_destroy(ast);
    free(source);
    
    return 0;
}
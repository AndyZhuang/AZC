#ifndef AZC_CODEGEN_H
#define AZC_CODEGEN_H

#include "ast.h"
#include <stdio.h>

typedef struct {
    FILE* output;
    int indent;
    char* error_message;
} Codegen;

Codegen* codegen_create(FILE* output);
void codegen_destroy(Codegen* codegen);
void codegen_gen_program(Codegen* codegen, AST* ast);
const char* codegen_get_error(Codegen* codegen);

#endif // AZC_CODEGEN_H
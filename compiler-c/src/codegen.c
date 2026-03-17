#include "codegen.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void codegen_indent(Codegen* codegen);
static void codegen_gen_node(Codegen* codegen, ASTNode* node);
static void codegen_gen_expression(Codegen* codegen, ASTNode* node);

Codegen* codegen_create(FILE* output) {
    Codegen* codegen = (Codegen*)malloc(sizeof(Codegen));
    codegen->output = output;
    codegen->indent = 0;
    codegen->error_message = NULL;
    return codegen;
}

void codegen_destroy(Codegen* codegen) {
    if (codegen->error_message) free(codegen->error_message);
    free(codegen);
}

static void codegen_indent(Codegen* codegen) {
    for (int i = 0; i < codegen->indent; i++) {
        fprintf(codegen->output, "    ");
    }
}

static const char* token_to_c_op(TokenType op) {
    switch (op) {
        case TOKEN_PLUS: return "+";
        case TOKEN_MINUS: return "-";
        case TOKEN_STAR: return "*";
        case TOKEN_SLASH: return "/";
        case TOKEN_PERCENT: return "%";
        case TOKEN_EQ: return "==";
        case TOKEN_NE: return "!=";
        case TOKEN_LT: return "<";
        case TOKEN_GT: return ">";
        case TOKEN_LE: return "<=";
        case TOKEN_GE: return ">=";
        case TOKEN_AND: return "&&";
        case TOKEN_OR: return "||";
        default: return "?";
    }
}

static void codegen_gen_expression(Codegen* codegen, ASTNode* node) {
    if (!node) return;
    
    switch (node->type) {
        case AST_NUMBER:
            fprintf(codegen->output, "%d", (int)node->literal.value);
            break;
            
        case AST_FLOAT:
            fprintf(codegen->output, "%f", node->literal.value);
            break;
            
        case AST_BOOL:
            fprintf(codegen->output, "%d", (int)node->literal.value);
            break;
            
        case AST_STRING:
            fprintf(codegen->output, "\"%s\"", node->string.value);
            break;
            
        case AST_NIL:
            fprintf(codegen->output, "NULL");
            break;
            
        case AST_IDENTIFIER:
            fprintf(codegen->output, "%s", node->identifier.name);
            break;
            
        case AST_BINARY: {
            fprintf(codegen->output, "(");
            codegen_gen_expression(codegen, node->binary.left);
            fprintf(codegen->output, " %s ", token_to_c_op(node->binary.operator));
            codegen_gen_expression(codegen, node->binary.right);
            fprintf(codegen->output, ")");
            break;
        }
            
        case AST_UNARY: {
            if (node->unary.operator == TOKEN_MINUS) {
                fprintf(codegen->output, "(-");
                codegen_gen_expression(codegen, node->unary.operand);
                fprintf(codegen->output, ")");
            } else if (node->unary.operator == TOKEN_BANG || node->unary.operator == TOKEN_NOT) {
                fprintf(codegen->output, "(!");
                codegen_gen_expression(codegen, node->unary.operand);
                fprintf(codegen->output, ")");
            } else {
                codegen_gen_expression(codegen, node->unary.operand);
            }
            break;
        }
            
        default:
            fprintf(codegen->output, "/* unknown expr */");
            break;
    }
}

static void codegen_gen_node(Codegen* codegen, ASTNode* node) {
    if (!node) return;
    
    switch (node->type) {
        case AST_LET:
            codegen_indent(codegen);
            fprintf(codegen->output, "AZC ");
            fprintf(codegen->output, "%s", node->let_stmt.name);
            fprintf(codegen->output, " = ");
            if (node->let_stmt.value) {
                codegen_gen_expression(codegen, node->let_stmt.value);
            } else {
                fprintf(codegen->output, "0");
            }
            fprintf(codegen->output, ";\n");
            break;
            
        case AST_FUNCTION:
            codegen_indent(codegen);
            fprintf(codegen->output, "void azc_%s(void) {\n", node->function.name);
            codegen->indent++;
            if (node->function.body) {
                codegen_gen_node(codegen, node->function.body);
            }
            codegen->indent--;
            codegen_indent(codegen);
            fprintf(codegen->output, "}\n");
            break;
            
        case AST_RETURN:
            codegen_indent(codegen);
            fprintf(codegen->output, "return");
            if (node->return_stmt.value) {
                fprintf(codegen->output, " ");
                codegen_gen_expression(codegen, node->return_stmt.value);
            }
            fprintf(codegen->output, ";\n");
            break;
            
        case AST_BINARY:
        case AST_UNARY:
        case AST_IDENTIFIER:
        case AST_NUMBER:
        case AST_FLOAT:
        case AST_STRING:
        case AST_BOOL:
        case AST_NIL:
            codegen_indent(codegen);
            codegen_gen_expression(codegen, node);
            fprintf(codegen->output, ";\n");
            break;
            
        default:
            codegen_indent(codegen);
            fprintf(codegen->output, "/* unsupported node type: %d */\n", node->type);
            break;
    }
}

void codegen_gen_program(Codegen* codegen, AST* ast) {
    fprintf(codegen->output, "/* AZC generated code */\n");
    fprintf(codegen->output, "#include <stdio.h>\n");
    fprintf(codegen->output, "#include <stdlib.h>\n");
    fprintf(codegen->output, "#include <stdbool.h>\n");
    fprintf(codegen->output, "\n");
    fprintf(codegen->output, "/* Runtime declarations */\n");
    fprintf(codegen->output, "#define AZC int\n");
    fprintf(codegen->output, "\n");
    
    for (int i = 0; i < ast->stmt_count; i++) {
        codegen_gen_node(codegen, ast->statements[i]);
    }
}

const char* codegen_get_error(Codegen* codegen) {
    return codegen->error_message;
}
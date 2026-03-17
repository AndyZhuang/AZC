#include "ast.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define INITIAL_STMT_CAPACITY 128

AST* ast_create(void) {
    AST* ast = (AST*)malloc(sizeof(AST));
    ast->statements = (ASTNode**)malloc(sizeof(ASTNode*) * INITIAL_STMT_CAPACITY);
    ast->stmt_count = 0;
    ast->stmt_capacity = INITIAL_STMT_CAPACITY;
    return ast;
}

void ast_destroy(AST* ast) {
    if (ast) {
        for (int i = 0; i < ast->stmt_count; i++) {
            free(ast->statements[i]);
        }
        free(ast->statements);
        free(ast);
    }
}

ASTNode* ast_add_node(AST* ast, ASTNode* node) {
    if (ast->stmt_count >= ast->stmt_capacity) {
        ast->stmt_capacity *= 2;
        ast->statements = (ASTNode**)realloc(ast->statements, 
            sizeof(ASTNode*) * ast->stmt_capacity);
    }
    ast->statements[ast->stmt_count++] = node;
    return node;
}

ASTNode* ast_make_node(AST* ast, ASTNodeType type) {
    ASTNode* node = (ASTNode*)calloc(1, sizeof(ASTNode));
    node->type = type;
    node->children = NULL;
    node->child_count = 0;
    node->inferred_type = NULL;
    return ast_add_node(ast, node);
}

static void print_indent(int indent) {
    for (int i = 0; i < indent; i++) printf("  ");
}

static const char* ast_type_name(ASTNodeType type) {
    switch (type) {
        case AST_PROGRAM: return "Program";
        case AST_LET: return "Let";
        case AST_LET_MUT: return "LetMut";
        case AST_FUNCTION: return "Function";
        case AST_IF: return "If";
        case AST_WHILE: return "While";
        case AST_FOR: return "For";
        case AST_RETURN: return "Return";
        case AST_BLOCK: return "Block";
        case AST_BINARY: return "Binary";
        case AST_UNARY: return "Unary";
        case AST_CALL: return "Call";
        case AST_MEMBER: return "Member";
        case AST_IDENTIFIER: return "Identifier";
        case AST_NUMBER: return "Number";
        case AST_FLOAT: return "Float";
        case AST_STRING: return "String";
        case AST_BOOL: return "Bool";
        case AST_NIL: return "Nil";
        default: return "Unknown";
    }
}

void ast_print(ASTNode* node, int indent) {
    if (!node) return;
    
    print_indent(indent);
    printf("%s", ast_type_name(node->type));
    
    switch (node->type) {
        case AST_LET:
        case AST_LET_MUT:
            printf(" (%s)", node->let_stmt.name);
            if (node->let_stmt.value) {
                printf(" = \n");
                ast_print(node->let_stmt.value, indent + 1);
                return;
            }
            break;
            
        case AST_FUNCTION:
            printf(" (%s)", node->function.name);
            break;
            
        case AST_BINARY:
            printf(" (%s)", 
                node->binary.operator == TOKEN_PLUS ? "+" :
                node->binary.operator == TOKEN_MINUS ? "-" :
                node->binary.operator == TOKEN_STAR ? "*" :
                node->binary.operator == TOKEN_SLASH ? "/" :
                "op");
            break;
            
        case AST_IDENTIFIER:
            printf(" %s", node->identifier.name);
            break;
            
        case AST_NUMBER:
            printf(" %d", (int)node->literal.value);
            break;
            
        case AST_FLOAT:
            printf(" %f", node->literal.value);
            break;
            
        case AST_STRING:
            printf(" \"%s\"", node->string.value);
            break;

        case AST_RETURN:
            if (node->return_stmt.value) {
                printf(" \n");
                ast_print(node->return_stmt.value, indent + 1);
                return;
            }
            break;
            
        default:
            break;
    }
    
    printf("\n");
    
    for (int i = 0; i < node->child_count; i++) {
        ast_print(node->children[i], indent + 1);
    }
}
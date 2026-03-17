#ifndef AZC_AST_H
#define AZC_AST_H

#include "lexer.h"
#include <stddef.h>

typedef enum {
    AST_PROGRAM,
    AST_LET,
    AST_LET_MUT,
    AST_FUNCTION,
    AST_IF,
    AST_WHILE,
    AST_FOR,
    AST_RETURN,
    AST_EXPRESSION_STMT,
    AST_BLOCK,
    
    // Expressions
    AST_BINARY,
    AST_UNARY,
    AST_CALL,
    AST_INDEX,
    AST_MEMBER,
    AST_IDENTIFIER,
    AST_NUMBER,
    AST_FLOAT,
    AST_STRING,
    AST_BOOL,
    AST_NIL,
    AST_ARRAY,
    AST_HASH,
    AST_LAMBDA,
    AST_MATCH,
    
    // Types
    AST_TYPE_INT,
    AST_TYPE_FLOAT,
    AST_TYPE_STRING,
    AST_TYPE_BOOL,
    AST_TYPE_VOID,
    AST_TYPE_ARRAY,
    AST_TYPE_POINTER,
    AST_TYPE_FUNCTION
} ASTNodeType;

typedef struct ASTNode {
    ASTNodeType type;
    int line;
    
    union {
        struct {
            char* name;
            struct ASTNode* value;
            struct ASTNode* type_annotation;
            bool mutable;
        } let_stmt;
        
        struct {
            char* name;
            struct ASTNode** params;
            int param_count;
            struct ASTNode* return_type;
            struct ASTNode* body;
        } function;
        
        struct {
            struct ASTNode* condition;
            struct ASTNode* then_branch;
            struct ASTNode* else_branch;
        } if_stmt;
        
        struct {
            struct ASTNode* condition;
            struct ASTNode* body;
        } while_stmt;
        
        struct {
            char* var;
            struct ASTNode* start;
            struct ASTNode* end;
            struct ASTNode* body;
        } for_stmt;

        struct {
            struct ASTNode* value;
        } return_stmt;
        
        struct {
            struct ASTNode* left;
            TokenType operator;
            struct ASTNode* right;
        } binary;
        
        struct {
            TokenType operator;
            struct ASTNode* operand;
        } unary;
        
        struct {
            struct ASTNode* callee;
            struct ASTNode** args;
            int arg_count;
        } call;
        
        struct {
            struct ASTNode* object;
            char* member;
        } member;
        
        struct {
            char* name;
        } identifier;
        
        struct {
            double value;
            bool is_float;
        } literal;
        
        struct {
            char* value;
        } string;
    };
    
    struct ASTNode** children;
    int child_count;
    
    char* inferred_type;
} ASTNode;

typedef struct {
    ASTNode** statements;
    int stmt_count;
    int stmt_capacity;
} AST;

AST* ast_create(void);
void ast_destroy(AST* ast);
ASTNode* ast_add_node(AST* ast, ASTNode* node);
ASTNode* ast_make_node(AST* ast, ASTNodeType type);
void ast_print(ASTNode* node, int indent);

#endif // AZC_AST_H
#define _POSIX_C_SOURCE 200809L

#include "parser.h"
#include "lexer.h"
#include "ast.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    Lexer* lexer;
    Token* tokens;
    size_t current;
    size_t token_count;
    bool had_error;
} ParserInternal;

static Token* peek(Parser* parser) {
    ParserInternal* p = (ParserInternal*)parser;
    if (p->current >= p->token_count) {
        return &p->tokens[p->token_count > 0 ? p->token_count - 1 : 0];
    }
    return &p->tokens[p->current];
}

static void advance(Parser* parser) {
    ParserInternal* p = (ParserInternal*)parser;
    if (p->current < p->token_count - 1) {
        p->current++;
    }
}

static bool is_at_end(Parser* parser) {
    ParserInternal* p = (ParserInternal*)parser;
    return p->current >= p->token_count - 1;
}

static bool check(Parser* parser, TokenType type) {
    if (is_at_end(parser)) return false;
    return peek(parser)->type == type;
}

Parser* parser_create(Lexer* lexer) {
    ParserInternal* p = (ParserInternal*)calloc(1, sizeof(ParserInternal));
    p->lexer = lexer;
    p->tokens = lexer->tokens;
    p->token_count = lexer->token_count;
    p->current = 0;
    p->had_error = false;
    return (Parser*)p;
}

void parser_destroy(Parser* parser) {
    if (parser) free(parser);
}

static ASTNode* parse_expression(Parser* parser);

static ASTNode* parse_primary(Parser* parser) {
    Token* tok = peek(parser);
    TokenType ttype = tok->type;
    
    if (ttype == TOKEN_NUMBER) {
        advance(parser);
        ASTNode* node = (ASTNode*)calloc(1, sizeof(ASTNode));
        node->type = AST_NUMBER;
        node->literal.value = tok->number;
        node->literal.is_float = false;
        return node;
    }
    
    if (ttype == TOKEN_STRING) {
        advance(parser);
        ASTNode* node = (ASTNode*)calloc(1, sizeof(ASTNode));
        node->type = AST_STRING;
        node->string.value = strdup(tok->lexeme ? tok->lexeme : "");
        return node;
    }
    
    if (ttype == TOKEN_IDENTIFIER) {
        advance(parser);
        ASTNode* node = (ASTNode*)calloc(1, sizeof(ASTNode));
        node->type = AST_IDENTIFIER;
        node->identifier.name = strdup(tok->lexeme ? tok->lexeme : "");
        return node;
    }
    
    if (ttype == TOKEN_TRUE || ttype == TOKEN_FALSE) {
        advance(parser);
        ASTNode* node = (ASTNode*)calloc(1, sizeof(ASTNode));
        node->type = AST_BOOL;
        node->literal.value = (ttype == TOKEN_TRUE) ? 1.0 : 0.0;
        return node;
    }
    
    return NULL;
}

static ASTNode* parse_term(Parser* parser) {
    ASTNode* left = parse_primary(parser);
    if (!left) return NULL;
    
    while (check(parser, TOKEN_PLUS) || check(parser, TOKEN_MINUS)) {
        Token* op = peek(parser);
        advance(parser);
        ASTNode* right = parse_primary(parser);
        if (!right) break;
        ASTNode* node = (ASTNode*)calloc(1, sizeof(ASTNode));
        node->type = AST_BINARY;
        node->binary.operator = op->type;
        node->binary.left = left;
        node->binary.right = right;
        left = node;
    }
    
    return left;
}

static ASTNode* parse_expression(Parser* parser) {
    return parse_term(parser);
}

AST* parser_parse(Parser* parser) {
    AST* ast = ast_create();
    
    while (!is_at_end(parser)) {
        Token* tok = peek(parser);
        
        if (tok->type == TOKEN_LET) {
            advance(parser);
            Token* name_tok = peek(parser);
            if (name_tok && name_tok->type == TOKEN_IDENTIFIER) {
                advance(parser);
                ASTNode* let_node = (ASTNode*)calloc(1, sizeof(ASTNode));
                let_node->type = AST_LET;
                let_node->let_stmt.name = strdup(name_tok->lexeme ? name_tok->lexeme : "");
                let_node->let_stmt.value = NULL;
                let_node->let_stmt.mutable = false;
                
                if (check(parser, TOKEN_ASSIGN)) {
                    advance(parser);
                    let_node->let_stmt.value = parse_expression(parser);
                }
                
                ast_add_node(ast, let_node);
            }
        } else {
            ASTNode* expr = parse_expression(parser);
            if (expr) {
                ast_add_node(ast, expr);
            }
        }
        
        if (check(parser, TOKEN_SEMICOLON)) {
            advance(parser);
        }
    }
    
    return ast;
}

bool parser_had_error(Parser* parser) {
    ParserInternal* p = (ParserInternal*)parser;
    return p->had_error;
}

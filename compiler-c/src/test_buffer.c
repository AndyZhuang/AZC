#include "lexer.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <assert.h>

#define INITIAL_TOKEN_CAPACITY 1024

Lexer* lexer_create(const char* source) {
    Lexer* lexer = (Lexer*)malloc(sizeof(Lexer));
    lexer->source = source;
    lexer->source_len = source ? strlen(source) : 0;
    lexer->position = 0;
    lexer->line = 1;
    lexer->column = 1;
    lexer->current_char = (source && source[0]) ? source[0] : '\0';
    lexer->tokens = (Token*)malloc(sizeof(Token) * INITIAL_TOKEN_CAPACITY);
    lexer->token_capacity = INITIAL_TOKEN_CAPACITY;
    lexer->token_count = 0;
    return lexer;
}

void lexer_destroy(Lexer* lexer) {
    if (lexer) { free(lexer->tokens); free(lexer); }
}

static void lexer_advance(Lexer* lexer) {
    if (lexer->current_char == '\n') { lexer->line++; lexer->column = 1; }
    else lexer->column++;
    lexer->position++;
    if (lexer->position < lexer->source_len) lexer->current_char = lexer->source[lexer->position];
    else lexer->current_char = '\0';
}

static void lexer_skip_whitespace(Lexer* lexer) {
    while (lexer->current_char != '\0' && isspace(lexer->current_char)) lexer_advance(lexer);
}

static Token* lexer_make_token(Lexer* lexer, TokenType type) {
    if (lexer->token_count >= lexer->token_capacity) {
        lexer->token_capacity *= 2;
        lexer->tokens = (Token*)realloc(lexer->tokens, sizeof(Token) * lexer->token_capacity);
    }
    Token* token = &lexer->tokens[lexer->token_count++];
    token->type = type; token->line = lexer->line; token->column = lexer->column;
    token->lexeme = NULL; token->number = 0; token->length = 1;
    return token;
}

static void lexer_read_number(Lexer* lexer, Token* token) {
    size_t start = lexer->position;
    while (isdigit(lexer->current_char)) lexer_advance(lexer);
    if (lexer->current_char == '.' && isdigit(lexer->source[lexer->position + 1])) {
        lexer_advance(lexer);
        while (isdigit(lexer->current_char)) lexer_advance(lexer);
        token->type = TOKEN_FLOAT;
        token->number = atof(lexer->source + start);
    } else {
        token->type = TOKEN_NUMBER;
        token->number = atoi(lexer->source + start);
    }
    token->length = lexer->position - start;
}

static TokenType do_ident(Lexer* lexer, char first_char) {
    char buffer[256];
    memset(buffer, 0, 256);
    int pos = 0;
    
    printf("  do_ident called, first_char='%c', current_char='%c'\n", first_char, lexer->current_char);
    buffer[pos++] = first_char;
    printf("  after buffer[pos++], pos=%d, buffer='%s'\n", pos, buffer);
    
    while (lexer->current_char != '\0' && (isalnum(lexer->current_char) || lexer->current_char == '_')) {
        printf("  in loop: current_char='%c', pos=%d\n", lexer->current_char, pos);
        if (pos < 255) buffer[pos++] = lexer->current_char;
        printf("  after adding, buffer='%s'\n", buffer);
        lexer_advance(lexer);
    }
    buffer[pos] = '\0';
    printf("  final: buffer='%s', pos=%d\n", buffer, pos);
    return TOKEN_IDENTIFIER;
}

Token* lexer_tokenize(Lexer* lexer) {
    while (lexer->current_char != '\0') {
        lexer_skip_whitespace(lexer);
        if (lexer->current_char == '\0') break;
        
        Token* token = lexer_make_token(lexer, TOKEN_EOF);
        
        if (isdigit(lexer->current_char)) {
            lexer_read_number(lexer, token);
        } else if (isalpha(lexer->current_char) || lexer->current_char == '_') {
            token->type = do_ident(lexer, lexer->current_char);
        } else {
            token->type = TOKEN_ERROR;
        }
        
        if (token->type != TOKEN_IDENTIFIER && token->type != TOKEN_STRING && 
            token->type != TOKEN_NUMBER && token->type != TOKEN_FLOAT && token->type != TOKEN_ERROR)
            lexer_advance(lexer);
    }
    lexer_make_token(lexer, TOKEN_EOF);
    return lexer->tokens;
}

int main() {
    const char* source = "let x = 10";
    printf("Source: '%s'\n", source);
    printf("First char: '%c'\n\n", source[0]);
    
    Lexer* lexer = lexer_create(source);
    lexer_tokenize(lexer);
    
    printf("\nTokens:\n");
    for (size_t i = 0; i < lexer->token_count; i++) {
        Token* t = &lexer->tokens[i];
        printf("  [%d:%d] type=%d\n", t->line, t->column, t->type);
    }
    lexer_destroy(lexer);
    return 0;
}

#define _POSIX_C_SOURCE 200809L

#include "lexer.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

#define INITIAL_TOKEN_CAPACITY 1024

static const char* KEYWORDS[] = {
    "let", "def", "if", "else", "while", "for", "in", "return",
    "true", "false", "nil", "self", "class", "end", "and", "or", "not"
};
static const int KEYWORD_COUNT = 17;

static TokenType get_keyword(const char* ident) {
    for (int i = 0; i < KEYWORD_COUNT; i++) {
        if (strcmp(ident, KEYWORDS[i]) == 0) {
            return TOKEN_LET + i;
        }
    }
    return TOKEN_IDENTIFIER;
}

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
    if (lexer) {
        for (size_t i = 0; i < lexer->token_count; i++) free(lexer->tokens[i].lexeme);
        free(lexer->tokens); free(lexer);
    }
}

static void lexer_advance(Lexer* lexer) {
    if (lexer->current_char == '\n') { lexer->line++; lexer->column = 1; }
    else lexer->column++;
    if (++lexer->position < lexer->source_len) lexer->current_char = lexer->source[lexer->position];
    else lexer->current_char = '\0';
}

static void skip_ws(Lexer* lexer) {
    while (lexer->current_char && isspace(lexer->current_char)) lexer_advance(lexer);
}

static Token* make_token(Lexer* lexer, TokenType type) {
    if (lexer->token_count >= lexer->token_capacity) {
        lexer->token_capacity *= 2;
        lexer->tokens = realloc(lexer->tokens, sizeof(Token) * lexer->token_capacity);
    }
    Token* t = &lexer->tokens[lexer->token_count++];
    t->type = type;
    t->lexeme = NULL;
    t->number = 0;
    t->length = 0;
    t->line = lexer->line;
    t->column = lexer->column;
    return t;
}

static void read_number(Lexer* lexer, Token* token) {
    size_t start = lexer->position;
    while (isdigit(lexer->current_char)) lexer_advance(lexer);
    bool is_float = false;
    if (lexer->current_char == '.' && isdigit(lexer->source[lexer->position + 1])) {
        is_float = true; lexer_advance(lexer);
        while (isdigit(lexer->current_char)) lexer_advance(lexer);
    }
    size_t len = lexer->position - start;
    char* num = malloc(len + 1);
    memcpy(num, lexer->source + start, len);
    num[len] = '\0';
    token->type = is_float ? TOKEN_FLOAT : TOKEN_NUMBER;
    token->number = is_float ? atof(num) : atoi(num);
    free(num);
    token->length = len;
}

static void read_string(Lexer* lexer, Token* token) {
    lexer_advance(lexer);
    size_t start = lexer->position;
    while (lexer->current_char && lexer->current_char != '"') {
        if (lexer->current_char == '\\') lexer_advance(lexer);
        lexer_advance(lexer);
    }
    size_t len = lexer->position - start;
    token->lexeme = malloc(len + 1);
    memcpy(token->lexeme, lexer->source + start, len);
    token->lexeme[len] = '\0';
    token->type = TOKEN_STRING;
    token->length = len + 2;
    if (lexer->current_char == '"') lexer_advance(lexer);
}

static void read_identifier(Lexer* lexer, Token* token) {
    size_t start = lexer->position;
    while (isalnum(lexer->current_char) || lexer->current_char == '_') lexer_advance(lexer);
    size_t len = lexer->position - start;
    token->lexeme = malloc(len + 1);
    memcpy(token->lexeme, lexer->source + start, len);
    token->lexeme[len] = '\0';
    token->type = get_keyword(token->lexeme);
    if (token->type != TOKEN_IDENTIFIER) { free(token->lexeme); token->lexeme = NULL; }
    token->length = len;
}

Token* lexer_tokenize(Lexer* lexer) {
    while (lexer->current_char) {
        skip_ws(lexer);
        if (!lexer->current_char) break;
        
        Token* token = make_token(lexer, TOKEN_EOF);
        
        switch (lexer->current_char) {
            case '(': token->type = TOKEN_LPAREN; break;
            case ')': token->type = TOKEN_RPAREN; break;
            case '{': token->type = TOKEN_LBRACE; break;
            case '}': token->type = TOKEN_RBRACE; break;
            case '[': token->type = TOKEN_LBRACKET; break;
            case ']': token->type = TOKEN_RBRACKET; break;
            case ',': token->type = TOKEN_COMMA; break;
            case ';': token->type = TOKEN_SEMICOLON; break;
            case ':': token->type = TOKEN_COLON; break;
            case '.': token->type = TOKEN_DOT; break;
            case '+': token->type = TOKEN_PLUS; break;
            case '-': token->type = TOKEN_MINUS; break;
            case '*': token->type = TOKEN_STAR; break;
            case '/': token->type = TOKEN_SLASH; break;
            case '%': token->type = TOKEN_PERCENT; break;
            case '=': token->type = (lexer->source[lexer->position + 1] == '=') ? TOKEN_EQ : TOKEN_ASSIGN;
                     if (token->type != TOKEN_ASSIGN) lexer_advance(lexer); break;
            case '!': token->type = (lexer->source[lexer->position + 1] == '=') ? TOKEN_NE : TOKEN_BANG;
                     if (token->type != TOKEN_BANG) lexer_advance(lexer); break;
            case '<': token->type = (lexer->source[lexer->position + 1] == '=') ? TOKEN_LE : TOKEN_LT;
                     if (token->type != TOKEN_LT) lexer_advance(lexer); break;
            case '>': token->type = (lexer->source[lexer->position + 1] == '=') ? TOKEN_GE : TOKEN_GT;
                     if (token->type != TOKEN_GT) lexer_advance(lexer); break;
            case '"': read_string(lexer, token); break;
            default:
                if (isdigit(lexer->current_char)) read_number(lexer, token);
                else if (isalpha(lexer->current_char) || lexer->current_char == '_') read_identifier(lexer, token);
                else token->type = TOKEN_ERROR;
                break;
        }
        
        if (token->type != TOKEN_IDENTIFIER && token->type != TOKEN_STRING && 
            token->type != TOKEN_NUMBER && token->type != TOKEN_FLOAT && token->type != TOKEN_ERROR)
            lexer_advance(lexer);
    }
    make_token(lexer, TOKEN_EOF);
    return lexer->tokens;
}

const char* token_type_name(TokenType type) {
    switch (type) {
        case TOKEN_EOF: return "EOF"; case TOKEN_IDENTIFIER: return "IDENTIFIER";
        case TOKEN_NUMBER: return "NUMBER"; case TOKEN_STRING: return "STRING";
        case TOKEN_LET: return "LET"; case TOKEN_TRUE: return "TRUE"; case TOKEN_FALSE: return "FALSE";
        case TOKEN_ASSIGN: return "ASSIGN"; case TOKEN_PLUS: return "PLUS"; case TOKEN_MINUS: return "MINUS";
        default: return "UNKNOWN";
    }
}

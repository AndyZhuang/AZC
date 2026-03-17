#define _POSIX_C_SOURCE 200809L

#include "lexer.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>
#include <assert.h>

#define INITIAL_TOKEN_CAPACITY 1024
#define MAX_IDENTIFIER_LEN 256

typedef struct {
    const char* name;
    TokenType type;
} KeywordEntry;

static const KeywordEntry KEYWORDS[] = {
    {"let", TOKEN_LET}, {"def", TOKEN_DEF}, {"if", TOKEN_IF}, {"else", TOKEN_ELSE},
    {"while", TOKEN_WHILE}, {"for", TOKEN_FOR}, {"in", TOKEN_IN}, {"return", TOKEN_RETURN},
    {"true", TOKEN_TRUE}, {"false", TOKEN_FALSE}, {"nil", TOKEN_NIL}, {"self", TOKEN_SELF},
    {"class", TOKEN_CLASS}, {"end", TOKEN_END}, {"and", TOKEN_AND}, {"or", TOKEN_OR},
    {"not", TOKEN_NOT}, {"struct", TOKEN_STRUCT}, {"enum", TOKEN_ENUM}, {"match", TOKEN_MATCH},
    {"when", TOKEN_WHEN}, {"pub", TOKEN_PUB}, {"impl", TOKEN_IMPL}, {"module", TOKEN_MODULE},
    {"use", TOKEN_USE},
};

#define KEYWORD_COUNT (sizeof(KEYWORDS)/sizeof(KEYWORDS[0]))

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

static void skip_ws_and_comments(Lexer* lexer) {
    while (lexer->current_char) {
        if (isspace(lexer->current_char)) lexer_advance(lexer);
        else if (lexer->current_char == '#') {
            while (lexer->current_char && lexer->current_char != '\n') lexer_advance(lexer);
        } else break;
    }
}

static Token* make_token(Lexer* lexer, TokenType type) {
    if (lexer->token_count >= lexer->token_capacity) {
        lexer->token_capacity *= 2;
        lexer->tokens = realloc(lexer->tokens, sizeof(Token) * lexer->token_capacity);
    }
    Token* t = &lexer->tokens[lexer->token_count++];
    *t = (Token){type, NULL, 0, 0, lexer->line, lexer->column};
    return t;
}

static TokenType check_keyword(const char* ident) {
    for (size_t i = 0; i < KEYWORD_COUNT; i++) {
        if (strcmp(ident, KEYWORDS[i].name) == 0) {
            return KEYWORDS[i].type;
        }
    }
    return TOKEN_IDENTIFIER;
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

static void read_identifier(Lexer* lexer, Token* token) {
    size_t start = lexer->position;
    while (isalnum(lexer->current_char) || lexer->current_char == '_') lexer_advance(lexer);
    size_t len = lexer->position - start;
    
    token->lexeme = malloc(len + 1);
    memcpy(token->lexeme, lexer->source + start, len);
    token->lexeme[len] = '\0';
    token->length = len;
    
    TokenType kw_type = check_keyword(token->lexeme);
    if (kw_type != TOKEN_IDENTIFIER) {
        free(token->lexeme);
        token->lexeme = NULL;
    }
    token->type = kw_type;
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
    token->length = len + 2;
    if (lexer->current_char == '"') lexer_advance(lexer);
}

Token* lexer_tokenize(Lexer* lexer) {
    while (lexer->current_char) {
        skip_ws_and_comments(lexer);
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
            case '.': 
                if (lexer->source[lexer->position + 1] == '.') {
                    token->type = TOKEN_DOT_DOT;
                    lexer_advance(lexer);
                } else token->type = TOKEN_DOT;
                break;
            case '+':
                if (lexer->source[lexer->position + 1] == '+') { token->type = TOKEN_PLUS_PLUS; lexer_advance(lexer); }
                else if (lexer->source[lexer->position + 1] == '=') { token->type = TOKEN_PLUS_ASSIGN; lexer_advance(lexer); }
                else token->type = TOKEN_PLUS;
                break;
            case '-':
                if (lexer->source[lexer->position + 1] == '-') { token->type = TOKEN_MINUS_MINUS; lexer_advance(lexer); }
                else if (lexer->source[lexer->position + 1] == '=') { token->type = TOKEN_MINUS_ASSIGN; lexer_advance(lexer); }
                else if (lexer->source[lexer->position + 1] == '>') { token->type = TOKEN_ARROW; lexer_advance(lexer); }
                else token->type = TOKEN_MINUS;
                break;
            case '*':
                if (lexer->source[lexer->position + 1] == '=') { token->type = TOKEN_STAR_ASSIGN; lexer_advance(lexer); }
                else token->type = TOKEN_STAR;
                if (token->type != TOKEN_STAR) lexer_advance(lexer);
                break;
            case '/':
                if (lexer->source[lexer->position + 1] == '=') { token->type = TOKEN_SLASH_ASSIGN; lexer_advance(lexer); }
                else token->type = TOKEN_SLASH;
                if (token->type != TOKEN_SLASH) lexer_advance(lexer);
                break;
            case '%': token->type = TOKEN_PERCENT; break;
            case '=':
                if (lexer->source[lexer->position + 1] == '=') { token->type = TOKEN_EQ; lexer_advance(lexer); }
                else if (lexer->source[lexer->position + 1] == '>') { token->type = TOKEN_FAT_ARROW; lexer_advance(lexer); }
                else token->type = TOKEN_ASSIGN;
                break;
            case '!':
                if (lexer->source[lexer->position + 1] == '=') { token->type = TOKEN_NE; lexer_advance(lexer); }
                else token->type = TOKEN_BANG;
                break;
            case '<':
                if (lexer->source[lexer->position + 1] == '=') { token->type = TOKEN_LE; lexer_advance(lexer); }
                else token->type = TOKEN_LT;
                break;
            case '>':
                if (lexer->source[lexer->position + 1] == '=') { token->type = TOKEN_GE; lexer_advance(lexer); }
                else token->type = TOKEN_GT;
                break;
            case '&':
                if (lexer->source[lexer->position + 1] == '&') { token->type = TOKEN_AND; lexer_advance(lexer); }
                else token->type = TOKEN_AMPERSAND;
                break;
            case '|':
                if (lexer->source[lexer->position + 1] == '|') { token->type = TOKEN_OR; lexer_advance(lexer); }
                else token->type = TOKEN_PIPE;
                break;
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

void lexer_print_tokens(Lexer* lexer) {
    for (size_t i = 0; i < lexer->token_count; i++) {
        Token* t = &lexer->tokens[i];
        printf("[%zu] %s", i, token_type_name(t->type));
        if (t->lexeme) printf(" = \"%s\"", t->lexeme);
        if (t->type == TOKEN_NUMBER || t->type == TOKEN_FLOAT) printf(" = %g", t->number);
        printf("\n");
    }
}

const char* token_type_name(TokenType type) {
    switch (type) {
        case TOKEN_EOF: return "EOF"; case TOKEN_IDENTIFIER: return "IDENTIFIER";
        case TOKEN_NUMBER: return "NUMBER"; case TOKEN_FLOAT: return "FLOAT";
        case TOKEN_STRING: return "STRING"; case TOKEN_LET: return "LET";
        case TOKEN_DEF: return "DEF"; case TOKEN_IF: return "IF";
        case TOKEN_ELSE: return "ELSE"; case TOKEN_WHILE: return "WHILE";
        case TOKEN_FOR: return "FOR"; case TOKEN_IN: return "IN";
        case TOKEN_RETURN: return "RETURN"; case TOKEN_TRUE: return "TRUE";
        case TOKEN_FALSE: return "FALSE"; case TOKEN_NIL: return "NIL";
        case TOKEN_CLASS: return "CLASS"; case TOKEN_END: return "END";
        case TOKEN_AND: return "AND"; case TOKEN_OR: return "OR";
        case TOKEN_NOT: return "NOT"; case TOKEN_PLUS: return "PLUS";
        case TOKEN_MINUS: return "MINUS"; case TOKEN_STAR: return "STAR";
        case TOKEN_SLASH: return "SLASH"; case TOKEN_ASSIGN: return "ASSIGN";
        case TOKEN_EQ: return "EQ"; case TOKEN_NE: return "NE";
        case TOKEN_LT: return "LT"; case TOKEN_GT: return "GT";
        case TOKEN_LE: return "LE"; case TOKEN_GE: return "GE";
        case TOKEN_LPAREN: return "LPAREN"; case TOKEN_RPAREN: return "RPAREN";
        case TOKEN_LBRACE: return "LBRACE"; case TOKEN_RBRACE: return "RBRACE";
        case TOKEN_COMMA: return "COMMA"; case TOKEN_DOT: return "DOT";
        case TOKEN_SEMICOLON: return "SEMICOLON"; case TOKEN_ERROR: return "ERROR";
        default: return "UNKNOWN";
    }
}

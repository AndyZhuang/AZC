#ifndef AZC_LEXER_H
#define AZC_LEXER_H

#include <stdbool.h>
#include <stddef.h>

typedef enum {
    TOKEN_EOF = 0,
    TOKEN_IDENTIFIER,
    TOKEN_NUMBER,
    TOKEN_FLOAT,
    TOKEN_STRING,
    TOKEN_CHAR,
    
    // Keywords
    TOKEN_LET,
    TOKEN_DEF,
    TOKEN_IF,
    TOKEN_ELSE,
    TOKEN_WHILE,
    TOKEN_FOR,
    TOKEN_IN,
    TOKEN_RETURN,
    TOKEN_TRUE,
    TOKEN_FALSE,
    TOKEN_NIL,
    TOKEN_SELF,
    TOKEN_CLASS,
    TOKEN_END,
    TOKEN_AND,
    TOKEN_OR,
    TOKEN_NOT,
    TOKEN_STRUCT,
    TOKEN_ENUM,
    TOKEN_MATCH,
    TOKEN_WHEN,
    TOKEN_PUB,
    TOKEN_IMPL,
    TOKEN_MODULE,
    TOKEN_USE,
    
    // Operators
    TOKEN_PLUS,
    TOKEN_MINUS,
    TOKEN_STAR,
    TOKEN_SLASH,
    TOKEN_PERCENT,
    TOKEN_AMPERSAND,
    TOKEN_PIPE,
    TOKEN_CARET,
    TOKEN_TILDE,
    TOKEN_BANG,
    TOKEN_QUESTION,
    TOKEN_COLON,
    TOKEN_SEMICOLON,
    TOKEN_COMMA,
    TOKEN_DOT,
    TOKEN_LPAREN,
    TOKEN_RPAREN,
    TOKEN_LBRACE,
    TOKEN_RBRACE,
    TOKEN_LBRACKET,
    TOKEN_RBRACKET,
    TOKEN_ARROW,
    TOKEN_FAT_ARROW,
    
    // Comparison
    TOKEN_EQ,
    TOKEN_NE,
    TOKEN_LT,
    TOKEN_GT,
    TOKEN_LE,
    TOKEN_GE,
    
    // Assignment
    TOKEN_ASSIGN,
    TOKEN_PLUS_ASSIGN,
    TOKEN_MINUS_ASSIGN,
    TOKEN_STAR_ASSIGN,
    TOKEN_SLASH_ASSIGN,
    TOKEN_PERCENT_ASSIGN,
    
    // Literals
    TOKEN_PLUS_PLUS,
    TOKEN_MINUS_MINUS,
    TOKEN_AMPERSAND_AMPERSAND,
    TOKEN_PIPE_PIPE,
    
    // Range
    TOKEN_DOT_DOT,
    TOKEN_DOT_DOT_EQ,
    
    // Unknown
    TOKEN_ERROR
} TokenType;

typedef struct {
    TokenType type;
    char* lexeme;
    double number;
    int length;
    int line;
    int column;
} Token;

typedef struct {
    const char* source;
    size_t source_len;
    size_t position;
    int line;
    int column;
    char current_char;
    Token* tokens;
    size_t token_count;
    size_t token_capacity;
} Lexer;

Lexer* lexer_create(const char* source);
void lexer_destroy(Lexer* lexer);
Token* lexer_tokenize(Lexer* lexer);
void lexer_print_tokens(Lexer* lexer);

const char* token_type_name(TokenType type);

#endif // AZC_LEXER_H
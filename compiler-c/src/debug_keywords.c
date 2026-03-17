#include <stdio.h>
#include <string.h>

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
    TOKEN_DO,
    TOKEN_BREAK,
    TOKEN_CONTINUE,
    TOKEN_UNSAFE,
    TOKEN_ASYNC,
    TOKEN_SEMICOLON
} TokenType;

typedef struct {
    const char* name;
    TokenType type;
} KeywordEntry;

#define KEYWORD_COUNT 31

static const KeywordEntry KEYWORDS[KEYWORD_COUNT] = {
    {"let", TOKEN_LET},
    {"def", TOKEN_DEF},
    {"if", TOKEN_IF},
    {"else", TOKEN_ELSE},
    {"while", TOKEN_WHILE},
    {"for", TOKEN_FOR},
    {"in", TOKEN_IN},
    {"return", TOKEN_RETURN},
    {"true", TOKEN_TRUE},
    {"false", TOKEN_FALSE},
    {"nil", TOKEN_NIL},
    {"self", TOKEN_SELF},
    {"class", TOKEN_CLASS},
    {"end", TOKEN_END},
    {"and", TOKEN_AND},
    {"or", TOKEN_OR},
    {"not", TOKEN_NOT},
    {"struct", TOKEN_STRUCT},
    {"enum", TOKEN_ENUM},
    {"match", TOKEN_MATCH},
    {"when", TOKEN_WHEN},
    {"pub", TOKEN_PUB},
    {"impl", TOKEN_IMPL},
    {"module", TOKEN_MODULE},
    {"use", TOKEN_USE},
    {"do", TOKEN_IF},
    {"break", TOKEN_BREAK},
    {"continue", TOKEN_CONTINUE},
    {"unsafe", TOKEN_UNSAFE},
    {"async", TOKEN_ASYNC},
    {"true", TOKEN_TRUE}
};

int main() {
    char buffer[] = "let";
    printf("Looking for: '%s'\n", buffer);
    printf("KEYWORD_COUNT = %d\n", KEYWORD_COUNT);
    for (int i = 0; i < KEYWORD_COUNT; i++) {
        int cmp = strcmp(buffer, KEYWORDS[i].name);
        printf("KEYWORDS[%d]: name='%s' (len=%zu), cmp=%d\n", i, KEYWORDS[i].name, strlen(KEYWORDS[i].name), cmp);
        if (cmp == 0) {
            printf("MATCH! Returning type %d\n", KEYWORDS[i].type);
            return 0;
        }
    }
    printf("No match found!\n");
    return 1;
}

#include <stdio.h>
#include <string.h>
#include "lexer.h"
#include "ast.h"

void test_keywords() {
    printf("=== Test: Keywords ===\n");
    const char* source = 
        "let x = 10\n"
        "def greet(name)\n"
        "    if x > 5\n"
        "        return true\n"
        "    end\n"
        "end\n"
        "while x < 100\n"
        "    x = x + 1\n"
        "end";
    
    Lexer* lexer = lexer_create(source);
    lexer_tokenize(lexer);
    
    printf("Source:\n%s\n\nTokens:\n", source);
    for (size_t i = 0; i < lexer->token_count && i < 25; i++) {
        Token* t = &lexer->tokens[i];
        printf("[%d:%d] %s", t->line, t->column, token_type_name(t->type));
        if (t->lexeme) printf(" \"%s\"", t->lexeme);
        if (t->type == TOKEN_NUMBER || t->type == TOKEN_FLOAT) printf(" %g", t->number);
        printf("\n");
    }
    printf("... (total %zu tokens)\n", lexer->token_count);
    
    lexer_destroy(lexer);
    printf("PASSED\n\n");
}

void test_strings() {
    printf("=== Test: Strings ===\n");
    const char* source = "let name = \"AZC\"";
    
    Lexer* lexer = lexer_create(source);
    lexer_tokenize(lexer);
    
    for (size_t i = 0; i < lexer->token_count; i++) {
        Token* t = &lexer->tokens[i];
        printf("[%d:%d] %s", t->line, t->column, token_type_name(t->type));
        if (t->lexeme) printf(" = \"%s\"", t->lexeme);
        printf("\n");
    }
    
    lexer_destroy(lexer);
    printf("PASSED\n\n");
}

int main() {
    printf("AZC C Compiler Test Suite v0.1\n");
    printf("================================\n\n");
    
    test_keywords();
    test_strings();
    
    printf("All tests completed!\n");
    return 0;
}
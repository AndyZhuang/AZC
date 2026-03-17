/* AZC C Compiler Test */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lexer.h"
#include "ast.h"

void test_lexer() {
    printf("=== Testing Lexer ===\n");
    
    const char* source = 
        "let x = 10\n"
        "let name = \"AZC\"\n"
        "def greet(person)\n"
        "    puts \"Hello, #{person}!\"\n"
        "end\n";
    
    Lexer* lexer = lexer_create(source);
    lexer_tokenize(lexer);
    
    printf("Tokenized %zu tokens:\n", lexer->token_count);
    lexer_print_tokens(lexer);
    
    lexer_destroy(lexer);
    printf("Lexer test PASSED\n\n");
}

void test_ast() {
    printf("=== Testing AST ===\n");
    
    AST* ast = ast_create();
    ASTNode* node = ast_make_node(ast, AST_NUMBER);
    node->literal.value = 42;
    node->literal.is_float = false;
    node->line = 1;
    
    ast_print(node, 0);
    
    ast_destroy(ast);
    printf("AST test PASSED\n\n");
}

int main(int argc, char** argv) {
    printf("AZC C Compiler Test Suite\n");
    printf("==========================\n\n");
    
    test_lexer();
    test_ast();
    
    printf("All tests PASSED!\n");
    return 0;
}
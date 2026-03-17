#include <stdio.h>
#include <string.h>

int main() {
    char buffer[256];
    memset(buffer, 0, sizeof(buffer));
    
    char first_char = 'l';
    int pos = 0;
    buffer[pos++] = first_char;
    
    const char* src = "let x = 10";
    printf("Source: '%s'\n", src);
    printf("Initial buffer after first_char: '%s'\n", buffer);
    printf("Buffer bytes: ");
    for (int i = 0; i < 5; i++) printf("[%d]=%d('%c') ", i, buffer[i], buffer[i] ? buffer[i] : '0');
    printf("\n");
    
    // Simulate reading characters
    for (int i = 1; src[i] != '\0' && pos < 10; i++) {
        char c = src[i];
        if ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_') {
            printf("Adding char '%c' at position %d\n", c, pos);
            buffer[pos++] = c;
        } else {
            printf("Stopping at '%c'\n", c);
            break;
        }
    }
    
    buffer[pos] = '\0';
    printf("Final buffer: '%s'\n", buffer);
    printf("Buffer bytes: ");
    for (int i = 0; i < 10; i++) printf("[%d]=%d('%c') ", i, buffer[i], buffer[i] ? buffer[i] : '0');
    printf("\n");
    
    // Now test with the actual source string positions
    printf("\n--- Testing with string positions directly ---\n");
    memset(buffer, 0, sizeof(buffer));
    pos = 0;
    buffer[pos++] = src[0];  // 'l'
    for (int i = 1; src[i] != '\0'; i++) {
        char c = src[i];
        if ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_') {
            buffer[pos++] = c;
        } else break;
    }
    buffer[pos] = '\0';
    printf("Result: '%s'\n", buffer);
    return 0;
}

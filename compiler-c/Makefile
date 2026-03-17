# AZC C Compiler Makefile

CC = gcc
CFLAGS = -Wall -Wextra -std=c11 -g -O2
LDFLAGS =

SRC = src
BUILD = build
RUNTIME = runtime

SOURCES = $(SRC)/main.c $(SRC)/lexer.c $(SRC)/ast.c $(SRC)/parser.c $(SRC)/codegen.c
OBJECTS = $(SOURCES:.c=.o)

TARGET = azc

.PHONY: all clean test

all: $(TARGET)

$(TARGET): $(OBJECTS)
	$(CC) $(CFLAGS) -o $@ $^ $(LDFLAGS)

%.o: %.c
	$(CC) $(CFLAGS) -c -o $@ $<

clean:
	rm -f $(OBJECTS) $(TARGET)

test: $(TARGET)
	./$(TARGET) ../tests/01_variables.azc

# Dependencies
src/lexer.o: src/lexer.h
src/ast.o: src/ast.h src/lexer.h
src/parser.o: src/parser.h src/ast.h src/lexer.h
src/codegen.o: src/codegen.h src/ast.h

# Bootstrap: Build runtime first
runtime:
	@echo "Building AZC runtime..."
	cd $(RUNTIME) && $(MAKE) -s || echo "Runtime build skipped"
#pragma once
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdio.h>

enum Filetype {
    Klass,
    Jar,
};

enum ParseError {
    FileDoesNotExist,
    MissingFileArgument,
    MissingClassPathArgument,
};

struct Cli {
    enum Filetype ftype;
    FILE * file;
    char * fpath;
    // args
    bool should_run;
    bool verbose;
    bool should_dump;
    char * classpath;

    uint8_t flags; 
};

struct ParseResult {
    bool is_valid;
    union {
        struct Cli cli;
        enum ParseError error;
    } value;
};

struct ParseResult parse_args(char**, int);
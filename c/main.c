#include "argsparser/argsparser.h"
#include <stdio.h>

int main(int argc, char **argv) {
    struct ParseResult res = parse_args(argv, argc);
    struct Cli cli;
    if (!res.is_valid) {
        switch (res.value.error) {
        case MissingFileArgument:
            printf("Missing argument: filename\n");
            break;
        case FileDoesNotExist:
            printf("Provided file {s} does not exist\n");
            break;
        case MissingClassPathArgument:
            printf("Missing argument: class path\n");
            break;
        }
        printf("Aborting due to previous error\n");
        return 1;
    } else {
        cli = res.value.cli;
    }

    printf("Hello world\n");
    return 0;
}
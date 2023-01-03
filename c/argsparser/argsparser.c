#include "argsparser.h"
#include "../jvm/settings.h"
#include <stddef.h>
#include <stdlib.h>
#include <stdint.h>
#include <stdbool.h>
#include <string.h>
#include <stdio.h>
 
struct ParseResult parse_args(char ** args, int argc) 
{
    bool should_run = false;
    bool should_dump = false;
    bool verbose = false;
    uint8_t flags = 0;
    char * fname = NULL;
    enum Filetype ftype = Klass;
    char * classpath = NULL;
    struct ParseResult result;
    size_t fname_len;
    FILE * file;
    struct Cli cli;

    for (int i = 0; i < argc; i++) {
        char * current_s = args[i];
        if (!strcmp(current_s, "-d") || !strcmp(current_s, "--dump")) {
            should_dump = true;
        } else if (!strcmp(current_s, "-r") || !strcmp(current_s, "--run")) {
            should_run = true;
        } else if (!strcmp(current_s, "-j") || !strcmp(current_s, "--jar")) {
            ftype = Jar;
        } else if (!strcmp(current_s, "-ac") || !strcmp(current_s, "--access-control")) {
            flags |= SHOULD_CONTROL_ACCESS;
        } else if (!strcmp(current_s, "-vf") || !strcmp(current_s, "--verify")) {
            flags |= SHOULD_VERIFY;
        } else if (!strcmp(current_s, "-db") || !strcmp(current_s, "--dump-backtrace")) {
            flags |= SHOULD_BACKTRACE;
        } else if (!strcmp(current_s, "-vb") || !strcmp(current_s, "--verbose")) {
            verbose = true;
        } else if (!strcmp(current_s, "-cp") || !strcmp(current_s, "--classpath")) {
            if (i + 1 == argc) {
                result.is_valid = false;
                result.value.error = MissingClassPathArgument;
                return result;
            }
            classpath = args[i + 1];
        } 
    }

    bool missing_file = argc == 0;
    if (!missing_file) {
        fname = args[argc - 1];
        fname_len = strlen(fname);
        if (ftype == Klass) {
            if (fname_len <= 6) {
                missing_file = true;
            } else {
                missing_file = !strcmp(fname + (fname_len - 6),".class");
            }
        } else if (fname_len <= 4) {
            missing_file = true;
        } else {
            missing_file = !strcmp(fname + (fname_len - 4),".jar");
        }
    }
    if (missing_file) {
        result.is_valid = false;
        result.value.error = MissingFileArgument;
        return result;
    }
    file = fopen(fname, "r");
    if (!file) {
        result.is_valid = false;
        result.value.error = FileDoesNotExist;
    } else {
        result.is_valid = true;
        cli.ftype = ftype;
        cli.file = file;
        cli.fpath = fname;
        cli.should_run = should_run;
        cli.should_dump = should_dump;
        cli.verbose = verbose;
        cli.classpath = classpath;
        cli.flags = flags;
        result.value.cli = cli;
    }
    return result;
}
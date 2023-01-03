#pragma once
#include <cstdint>
#include <stddef.h>
#include <stdint.h>

struct ElementValuePair;

struct ElementValuePairArray {
    struct ElementValuePair * data;
    size_t len;
};

struct Annotation {
    uint16_t type_index;
    ElementValuePairArray element_value_pairs;
};

struct Bound {
    uint8_t type_parameter_index;
    uint8_t bound_index;
};

struct LocalVar {
    uint16_t start_pc;
    uint16_t length;
    uint16_t index;
};

struct Argument {
    uint16_t offset;
    uint8_t type_argument_index;
};

struct TypeAnnotation {

};
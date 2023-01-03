#pragma once
#include <cstdint>
#include <stddef.h>
#include <stdint.h>
#include "annotations.h"

enum ElementValueDisc {
    ConstantValue,
    EnumConstantValue,
    ClassInfo,
    AnnotationValue,
    ArrayValue,
};

struct ElementValue;
struct ElementValueArray {
    struct ElementValue * data;
    size_t len;
};

enum ConstantDisc {
    Integer,
    Double,
    Float,
    Long,
    Utf8,
};

struct Constant {
    ConstantDisc type;
    uint16_t name_index;
};

struct Enum {
    uint16_t type_name_index;
    uint16_t const_name_index;
};

struct ElementValue {
    ElementValueDisc type;
    union value {
        struct Constant constant;
        struct Enum enum_val;
        uint16_t index;
        struct Annotation annotation;
        struct ElementValueArray array;
    };
};

struct ElementValuePair {
    uint16_t element_name_index;
    struct ElementValue value;
};

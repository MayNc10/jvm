#pragma once
#include <stdint.h>

// Just a list of flags.
const uint8_t SHOULD_VERIFY = 1 << 0; 
const uint8_t SHOULD_CONTROL_ACCESS = 1 << 1; 
const uint8_t SHOULD_BACKTRACE = 1 << 2; 
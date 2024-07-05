#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

const uint8_t *get_string(uint8_t end);

/**
 * Uppercases the input string
 *
 * # SAFETY
 * The input pointer needs to follow the same safety requirements
 * as Rust 'std::ffi::CStr::from_ptr'
 */
const char *to_uppercase(const char *input);

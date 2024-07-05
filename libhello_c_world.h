#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct RustString {
  const char *ptr;
} RustString;

const uint8_t *get_string(uint8_t end);

/**
 * Uppercases the input string
 *
 * # SAFETY
 * The input pointer needs to follow the same safety requirements
 * as Rust 'std::ffi::CStr::from_ptr'
 */
struct RustString to_uppercase(const char *input);

void free_rust_string(struct RustString string);

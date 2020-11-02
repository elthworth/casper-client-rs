#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Maximum length of error-string output in bytes.
 */
#define MAX_ERROR_LEN 255

/**
 * Maximum length of provided `response_buf` buffer.
 */
#define MAX_RESPONSE_BUFFER_LEN 1024

/**
 * Get the last error copied to the provided buffer (must be large enough, TODO: 255 chars?
 * MAX_ERROR_LEN)
 */
uintptr_t get_last_error(unsigned char *buf, uintptr_t len);

/**
 * Get auction info.
 *
 * See [super::get_auction_info](function.get_auction_info.html) for more details.
 */
bool get_auction_info(const char *maybe_rpc_id,
                      const char *node_address,
                      bool verbose,
                      unsigned char *response_buf,
                      uintptr_t response_buf_len);

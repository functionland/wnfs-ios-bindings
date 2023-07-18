#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct RustResult_c_char {
  bool ok;
  const char *err;
  const char *result;
} RustResult_c_char;

typedef struct RustResult_c_void {
  bool ok;
  const char *err;
  const void *result;
} RustResult_c_void;

typedef struct RustResult_u8 {
  bool ok;
  const char *err;
  const uint8_t *result;
} RustResult_u8;

typedef struct SwiftData {
  const char *err;
  const uint8_t *result_ptr;
  size_t result_count;
} SwiftData;

typedef struct BlockStoreInterface {
  void *userdata;
  const struct SwiftData *(*put_fn)(void *userdata,
                                    const uint8_t *cid,
                                    const size_t *cid_len,
                                    const uint8_t *bytes,
                                    const size_t *bytes_len);
  const struct SwiftData *(*get_fn)(void *userdata, const uint8_t *cid, const size_t *cid_len);
  void (*dealloc)(const struct SwiftData *swiftdata);
} BlockStoreInterface;

void rust_result_string_free(struct RustResult_c_char *ptr);

void rust_result_void_free(struct RustResult_c_void *ptr);

void rust_result_bytes_free(struct RustResult_u8 *ptr);

void cstring_free(char *ptr);

void cbytes_free(uint8_t *data, int32_t len, int32_t capacity);

struct RustResult_c_void *load_with_wnfs_key_native(struct BlockStoreInterface block_store_interface,
                                                    size_t wnfs_key_arr_len,
                                                    const uint8_t *wnfs_key_arr_pointer,
                                                    const char *cid);

struct RustResult_c_char *init_native(struct BlockStoreInterface block_store_interface,
                                      size_t wnfs_key_arr_len,
                                      const uint8_t *wnfs_key_arr_pointer);

struct RustResult_c_char *write_file_from_path_native(struct BlockStoreInterface block_store_interface,
                                                      const char *cid,
                                                      const char *path_segments,
                                                      const char *_filename);

struct RustResult_c_char *read_filestream_to_path_native(struct BlockStoreInterface block_store_interface,
                                                         const char *cid,
                                                         const char *path_segments,
                                                         const char *_filename);

struct RustResult_c_char *read_file_to_path_native(struct BlockStoreInterface block_store_interface,
                                                   const char *cid,
                                                   const char *path_segments,
                                                   const char *_filename);

struct RustResult_c_char *write_file_native(struct BlockStoreInterface block_store_interface,
                                            const char *cid,
                                            const char *path_segments,
                                            size_t content_arr_len,
                                            const uint8_t *content_arr_pointer);

struct RustResult_u8 *read_file_native(struct BlockStoreInterface block_store_interface,
                                       const char *cid,
                                       const char *path_segments,
                                       size_t *len,
                                       size_t *capacity);

struct RustResult_c_char *mkdir_native(struct BlockStoreInterface block_store_interface,
                                       const char *cid,
                                       const char *path_segments);

struct RustResult_c_char *mv_native(struct BlockStoreInterface block_store_interface,
                                    const char *cid,
                                    const char *source_path_segments,
                                    const char *target_path_segments);

struct RustResult_c_char *cp_native(struct BlockStoreInterface block_store_interface,
                                    const char *cid,
                                    const char *source_path_segments,
                                    const char *target_path_segments);

struct RustResult_c_char *rm_native(struct BlockStoreInterface block_store_interface,
                                    const char *cid,
                                    const char *path_segments);

struct RustResult_u8 *ls_native(struct BlockStoreInterface block_store_interface,
                                const char *cid,
                                const char *path_segments,
                                size_t *len,
                                size_t *capacity);

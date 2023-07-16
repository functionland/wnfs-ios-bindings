#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Status {
  bool ok;
  const char *err;
} Status;

typedef struct GenericResult {
  const struct Status *status;
} GenericResult;

typedef struct ConfigResult {
  const struct Status *status;
  const char *result;
} ConfigResult;

typedef struct BytesResult {
  const struct Status *status;
  uint8_t *result;
} BytesResult;

typedef struct StringResult {
  const struct Status *status;
  const char *result;
} StringResult;

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

void status_free(struct Status *ptr);

void result_free(struct GenericResult *ptr);

void config_result_free(struct ConfigResult *ptr);

void bytes_result_free(struct BytesResult *ptr);

void string_result_free(struct StringResult *ptr);

void cstring_free(char *ptr);

void cbytes_free(uint8_t *data, int32_t len, int32_t capacity);

struct GenericResult *load_with_wnfs_key_native(struct BlockStoreInterface block_store_interface,
                                                size_t wnfs_key_arr_len,
                                                const uint8_t *wnfs_key_arr_pointer,
                                                const char *cid);

struct ConfigResult *init_native(struct BlockStoreInterface block_store_interface,
                                 size_t wnfs_key_arr_len,
                                 const uint8_t *wnfs_key_arr_pointer);

struct ConfigResult *write_file_from_path_native(struct BlockStoreInterface block_store_interface,
                                                 const char *cid,
                                                 const char *path_segments,
                                                 const char *_filename);

struct StringResult *read_filestream_to_path_native(struct BlockStoreInterface block_store_interface,
                                                    const char *cid,
                                                    const char *path_segments,
                                                    const char *_filename);

struct StringResult *read_file_to_path_native(struct BlockStoreInterface block_store_interface,
                                              const char *cid,
                                              const char *path_segments,
                                              const char *_filename);

struct ConfigResult *write_file_native(struct BlockStoreInterface block_store_interface,
                                       const char *cid,
                                       const char *path_segments,
                                       size_t content_arr_len,
                                       const uint8_t *content_arr_pointer);

struct BytesResult *read_file_native(struct BlockStoreInterface block_store_interface,
                                     const char *cid,
                                     const char *path_segments,
                                     size_t *len,
                                     size_t *capacity);

struct ConfigResult *mkdir_native(struct BlockStoreInterface block_store_interface,
                                  const char *cid,
                                  const char *path_segments);

struct ConfigResult *mv_native(struct BlockStoreInterface block_store_interface,
                               const char *cid,
                               const char *source_path_segments,
                               const char *target_path_segments);

struct ConfigResult *cp_native(struct BlockStoreInterface block_store_interface,
                               const char *cid,
                               const char *source_path_segments,
                               const char *target_path_segments);

struct ConfigResult *rm_native(struct BlockStoreInterface block_store_interface,
                               const char *cid,
                               const char *path_segments);

struct BytesResult *ls_native(struct BlockStoreInterface block_store_interface,
                              const char *cid,
                              const char *path_segments,
                              size_t *len,
                              size_t *capacity);

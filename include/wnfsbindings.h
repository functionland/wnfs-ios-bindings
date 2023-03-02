#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct SwiftData {
  const uint8_t *ptr;
  size_t count;
} SwiftData;

typedef struct BlockStoreInterface {
  void *userdata;
  const struct SwiftData *(*put_fn)(void *userdata, const uint8_t *bytes, const size_t *bytes_len, int64_t codec);
  const struct SwiftData *(*get_fn)(void *userdata, const uint8_t *cid, const size_t *cid_len);
  void (*dealloc)(const struct SwiftData *swiftdata);
} BlockStoreInterface;

typedef struct Config {
  const char *cid;
  const char *private_ref;
} Config;

char *create_private_forest_native(struct BlockStoreInterface block_store_interface);

char *get_private_ref_native(struct BlockStoreInterface block_store_interface,
                             size_t wnfs_key_arr_len,
                             const uint8_t *wnfs_key_arr_pointer,
                             const char *cid);

struct Config *create_root_dir_native(struct BlockStoreInterface block_store_interface,
                                      size_t wnfs_key_arr_len,
                                      const uint8_t *wnfs_key_arr_pointer,
                                      const char *cid);

struct Config *write_file_from_path_native(struct BlockStoreInterface block_store_interface,
                                           const char *cid,
                                           const char *private_ref,
                                           const char *path_segments,
                                           const char *filename);

char *read_filestream_to_path_native(struct BlockStoreInterface block_store_interface,
                                     const char *cid,
                                     const char *private_ref,
                                     const char *path_segments,
                                     const char *filename);

char *read_file_to_path_native(struct BlockStoreInterface block_store_interface,
                               const char *cid,
                               const char *private_ref,
                               const char *path_segments,
                               const char *filename);

struct Config *write_file_native(struct BlockStoreInterface block_store_interface,
                                 const char *cid,
                                 const char *private_ref,
                                 const char *path_segments,
                                 size_t content_arr_len,
                                 const uint8_t *content_arr_pointer);

uint8_t *read_file_native(struct BlockStoreInterface block_store_interface,
                          const char *cid,
                          const char *private_ref,
                          const char *path_segments,
                          size_t *len,
                          size_t *capacity);

struct Config *mkdir_native(struct BlockStoreInterface block_store_interface,
                            const char *cid,
                            const char *private_ref,
                            const char *path_segments);

struct Config *mv_native(struct BlockStoreInterface block_store_interface,
                         const char *cid,
                         const char *private_ref,
                         const char *source_path_segments,
                         const char *target_path_segments);

struct Config *cp_native(struct BlockStoreInterface block_store_interface,
                         const char *cid,
                         const char *private_ref,
                         const char *source_path_segments,
                         const char *target_path_segments);

struct Config *rm_native(struct BlockStoreInterface block_store_interface,
                         const char *cid,
                         const char *private_ref,
                         const char *path_segments);

uint8_t *ls_native(struct BlockStoreInterface block_store_interface,
                   const char *cid,
                   const char *private_ref,
                   const char *path_segments,
                   size_t *len,
                   size_t *capacity);

uint8_t *alloc_bytes(size_t *size);

void config_free(struct Config *ptr);

void cstring_free(char *ptr);

void cbytes_free(uint8_t *data, int32_t len, int32_t capacity);

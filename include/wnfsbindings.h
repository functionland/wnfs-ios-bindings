#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct RustString {
  const char *str;
} RustString;

typedef struct RustResult_RustString {
  bool ok;
  struct RustString err;
  struct RustString result;
} RustResult_RustString;

typedef struct RustBytes {
  const uint8_t *data;
  size_t len;
  size_t cap;
} RustBytes;

typedef struct RustResult_RustBytes {
  bool ok;
  struct RustString err;
  struct RustBytes result;
} RustResult_RustBytes;

typedef struct RustVoid {
  void *result;
} RustVoid;

typedef struct RustResult_RustVoid {
  bool ok;
  struct RustString err;
  struct RustVoid result;
} RustResult_RustVoid;

typedef struct BlockStoreInterface {
  void *userdata;
  struct RustResult_RustVoid (*put_fn)(void *userdata, struct RustBytes cid, struct RustBytes bytes);
  struct RustResult_RustBytes (*get_fn)(void *userdata, struct RustBytes cid);
  void (*dealloc_after_get)(struct RustResult_RustBytes data);
  void (*dealloc_after_put)(struct RustResult_RustVoid data);
} BlockStoreInterface;

void rust_result_string_free(struct RustResult_RustString arg);

void rust_result_bytes_free(struct RustResult_RustBytes arg);

struct RustResult_RustVoid load_with_wnfs_key_native(struct BlockStoreInterface block_store_interface,
                                                     struct RustBytes wnfs_key,
                                                     struct RustString cid);

struct RustResult_RustString init_native(struct BlockStoreInterface block_store_interface,
                                         struct RustBytes wnfs_key);

struct RustResult_RustString write_file_from_path_native(struct BlockStoreInterface block_store_interface,
                                                         struct RustString cid,
                                                         struct RustString path_segments,
                                                         struct RustString _filename);

struct RustResult_RustString read_filestream_to_path_native(struct BlockStoreInterface block_store_interface,
                                                            struct RustString cid,
                                                            struct RustString path_segments,
                                                            struct RustString _filename);

struct RustResult_RustString read_file_to_path_native(struct BlockStoreInterface block_store_interface,
                                                      struct RustString cid,
                                                      struct RustString path_segments,
                                                      struct RustString _filename);

struct RustResult_RustString write_file_native(struct BlockStoreInterface block_store_interface,
                                               struct RustString cid,
                                               struct RustString path_segments,
                                               struct RustBytes _content);

struct RustResult_RustBytes read_file_native(struct BlockStoreInterface block_store_interface,
                                             struct RustString cid,
                                             struct RustString path_segments);

struct RustResult_RustString mkdir_native(struct BlockStoreInterface block_store_interface,
                                          struct RustString cid,
                                          struct RustString path_segments);

struct RustResult_RustString mv_native(struct BlockStoreInterface block_store_interface,
                                       struct RustString cid,
                                       struct RustString source_path_segments,
                                       struct RustString target_path_segments);

struct RustResult_RustString cp_native(struct BlockStoreInterface block_store_interface,
                                       struct RustString cid,
                                       struct RustString source_path_segments,
                                       struct RustString target_path_segments);

struct RustResult_RustString rm_native(struct BlockStoreInterface block_store_interface,
                                       struct RustString cid,
                                       struct RustString path_segments);

struct RustResult_RustBytes ls_native(struct BlockStoreInterface block_store_interface,
                                      struct RustString cid,
                                      struct RustString path_segments);

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Config {
  const char *cid;
  const char *private_ref;
} Config;

char *create_private_forest_native(const char *db_path);

char *get_private_ref_native(const char *db_path,
                             size_t wnfs_key_arr_size,
                             const uint8_t *wnfs_key_arr_pointer,
                             const char *cid);

struct Config *create_root_dir_native(const char *db_path,
                                      size_t wnfs_key_arr_size,
                                      const uint8_t *wnfs_key_arr_pointer,
                                      const char *cid);

struct Config *write_file_from_path_native(const char *db_path,
                                           const char *cid,
                                           const char *private_ref,
                                           const char *path_segments,
                                           const char *filename);

char *read_filestream_to_path_native(const char *db_path,
                                     const char *cid,
                                     const char *private_ref,
                                     const char *path_segments,
                                     const char *filename);

char *read_file_to_path_native(const char *db_path,
                               const char *cid,
                               const char *private_ref,
                               const char *path_segments,
                               const char *filename);

struct Config *write_file_native(const char *db_path,
                                 const char *cid,
                                 const char *private_ref,
                                 const char *path_segments,
                                 size_t content_arr_size,
                                 const uint8_t *content_arr_pointer);

uint8_t *read_file_native(const char *db_path,
                          const char *cid,
                          const char *private_ref,
                          const char *path_segments,
                          size_t *len,
                          size_t *capacity);

struct Config *mkdir_native(const char *db_path,
                            const char *cid,
                            const char *private_ref,
                            const char *path_segments);

struct Config *mv_native(const char *db_path,
                         const char *cid,
                         const char *private_ref,
                         const char *source_path_segments,
                         const char *target_path_segments);

struct Config *cp_native(const char *db_path,
                         const char *cid,
                         const char *private_ref,
                         const char *source_path_segments,
                         const char *target_path_segments);

struct Config *rm_native(const char *db_path,
                         const char *cid,
                         const char *private_ref,
                         const char *path_segments);

uint8_t *ls_native(const char *db_path,
                   const char *cid,
                   const char *private_ref,
                   const char *path_segments,
                   size_t *len,
                   size_t *capacity);

void config_free(struct Config *ptr);

void cstring_free(char *ptr);

void cbytes_free(uint8_t *data, int32_t len, int32_t capacity);

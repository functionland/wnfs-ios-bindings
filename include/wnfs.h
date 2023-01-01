#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Config {
  const char *cid;
  const char *private_ref;
} Config;

char *createPrivateForestNative(const char *db_path);

char *getPrivateRefNative(const char *db_path,
                          size_t wnfs_key_arr_size,
                          const uint8_t *wnfs_key_arr_pointer,
                          const char *cid);

struct Config *createRootDirNative(const char *db_path,
                                   size_t wnfs_key_arr_size,
                                   const uint8_t *wnfs_key_arr_pointer,
                                   const char *cid);

struct Config *writeFileFromPathNative(const char *db_path,
                                       const char *cid,
                                       const char *private_ref,
                                       const char *path_segments,
                                       const char *filename);

char *readFilestreamToPathNative(const char *db_path,
                                 const char *cid,
                                 const char *private_ref,
                                 const char *path_segments,
                                 const char *filename);

char *readFileToPathNative(const char *db_path,
                           const char *cid,
                           const char *private_ref,
                           const char *path_segments,
                           const char *filename);

struct Config *writeFileNative(const char *db_path,
                               const char *cid,
                               const char *private_ref,
                               const char *path_segments,
                               size_t content_arr_size,
                               const uint8_t *content_arr_pointer);

uint8_t *readFileNative(const char *db_path,
                        const char *cid,
                        const char *private_ref,
                        const char *path_segments,
                        int32_t *len,
                        int32_t *capacity);

struct Config *mkdirNative(const char *db_path,
                           const char *cid,
                           const char *private_ref,
                           const char *path_segments);

struct Config *mvNative(const char *db_path,
                        const char *cid,
                        const char *private_ref,
                        const char *source_path_segments,
                        const char *target_path_segments);

struct Config *cpNative(const char *db_path,
                        const char *cid,
                        const char *private_ref,
                        const char *source_path_segments,
                        const char *target_path_segments);

struct Config *rmNative(const char *db_path,
                        const char *cid,
                        const char *private_ref,
                        const char *path_segments);

uint8_t *lsNative(const char *db_path,
                  const char *cid,
                  const char *private_ref,
                  const char *path_segments,
                  int32_t *len,
                  int32_t *capacity);

void config_free(struct Config *ptr);

void cstring_free(char *ptr);

void cbytes_free(uint8_t *data, int32_t len, int32_t capacity);

#include "mjimap.h"
#include "config.h"
#include <string.h>

#include "gitmoji.h"

int mji_is_end(MjiMapEntry *entry) {
  return entry->desc == NULL && entry->name == NULL && entry->value == NULL;
}

void mji_list(MjiMapEntry *map) {
  while (!mji_is_end(map)) {
    printf("%s\t%s | %s\n", map->value, map->name, map->desc);

    map++;
  }
}

Error mji_find(MjiMapEntry *map, const char *name) {
  while (!mji_is_end(map)) {
    if (strcmp(map->name, name) == 0) {
      printf("%s%s%s", config.pre, map->value, config.post);
      return MJI_OK;
    }
    map++;
  }
  return MJI_NOT_FOUND;
}

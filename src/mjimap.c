#include "mjimap.h"
#include "config.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "gitmoji.h"

int mji_is_end(MjiMapEntry *entry) {
  return entry->desc == NULL && entry->name == NULL && entry->value == NULL;
}

void mji_list(MjiMapEntry *map) {
  char fmt[32];
  while (!mji_is_end(map)) {
    sprintf(fmt, "%%%lus\t%%s | %%s\n", 5 - strlen(map->value));
    printf(fmt, map->value, map->name, map->desc);

    map++;
  }
}

void mji_pre() { printf("%s", config.pre); }

void mji_post() { printf("%s", config.post); }

Error mji_find_or(MjiMapEntry *map, const char *name) {
  if (name[0] == MJI_SEPARATOR && strlen(name) == 1) {
    config.find_mji = TRUE;
    mji_post();
    return MJI_OK;
  }

  if (!config.find_mji) {
    printf(" %s", name);
    return MJI_OK;
  }

  config.find_mji = FALSE;

  return mji_find(map, name);
}

Error mji_find(MjiMapEntry *map, const char *name) {
  while (!mji_is_end(map)) {
    if (strcmp(map->name, name) == 0) {
      mji_pre();
      printf("%s", map->value);
      return MJI_OK;
    }
    map++;
  }
  return MJI_NOT_FOUND;
}

void mji_filter(FILE *input, FILE *output, char *filter) {
  unsigned int read = 0;
  char *line = NULL;
  unsigned long len = 0;

  while ((read = getline(&line, &len, input)) != -1) {
    if (strncmp(line, filter, strlen(filter)) == 0) {
      fprintf(output, "%s", line);
    }
  }

  free(line);
}

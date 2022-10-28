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

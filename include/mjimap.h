#ifndef MJIMAP_H_
#define MJIMAP_H_

#include "error.h"
#include <stdio.h>

#define MJI_MAP_END                                                            \
  { NULL, NULL, NULL }

typedef struct MjiMapEntry {
  const char *name;
  const char *value;
  const char *desc;
} MjiMapEntry;

extern const MjiMapEntry MJI_MAP[];

void mji_list(MjiMapEntry *map);
Error mji_find(MjiMapEntry *map, const char *name);

void mji_filter(FILE *input, FILE *output, char *filter);

#endif

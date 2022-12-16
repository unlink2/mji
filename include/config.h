#ifndef CONFIG_H_
#define CONFIG_H_

#include "error.h"
#include "types.h"

typedef struct Config {
  const char *pre;
  const char *post;

  bool find_mji;
  
  Error err;
} Config;

extern Config config;

Config config_init();

#endif

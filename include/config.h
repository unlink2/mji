#ifndef CONFIG_H_
#define CONFIG_H_

#include "error.h"

typedef struct Config {
  const char *pre;
  const char *post;
  Error err;
} Config;

extern Config config;

Config config_init();

#endif

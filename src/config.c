#include "config.h"
Config config;

Config config_init() {
  Config c = {"", "\n", TRUE, MJI_OK};
  return c;
}

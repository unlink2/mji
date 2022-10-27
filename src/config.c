#include "config.h"
Config config;

Config config_init() {
  Config c = {"", "\n", MJI_OK};
  return c;
}

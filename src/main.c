
/**
 * When built without test
 */
#include "mjimap.h"
#ifndef TEST

/// only use main if binary
#if TYPE == bin

#include "config.h"
#include <stdio.h>
#include <stdlib.h>
#include <argp.h>

const char *argp_program_version = "mji 0.1";
const char *argp_program_bug_address = "<lukas@krickl.dev>";
static char doc[] = "mji";
static char args_doc[] = "Quick access to gitmoji using a simple command";

static struct argp_option options[] = {
    {"list", 'l', NULL, 0, "List all mji entires"},
    {"post", 'p', "POSTFIX", 0, "Postfix for mji output"},
    {"pre", 'b', "PREFIX", 0, "Prefix for mji output"},
    {"filter", 'f', "FILTER", 0,
     "Filter input for strings starting with filter"},
    {0}};

static error_t parse_opt(int key, char *arg, struct argp_state *state) {
  Config *cfg = state->input;
  switch (key) {
  case 'l':
    mji_list((MjiMapEntry *)MJI_MAP);
    exit(0); // NOLINT
    break;
  case 'p':
    cfg->post = arg;
    break;
  case 'b':
    cfg->pre = arg;
    break;
  case 'f':
    mji_filter(stdin, stdout, arg);
    exit(0); // NOLINT
    break;
  case ARGP_KEY_ARG:
    /*
      if (state->arg_num >= 0) {
        // Too many arguments
        argp_usage(state); // NOLINT
      }
      // arguments->args[state->arg_num] = arg;
    */

    // all args are an emoji name, go find them in the map!
    cfg->err |= mji_find_or((MjiMapEntry *)MJI_MAP, arg);
    break;
  case ARGP_KEY_END:
    if (state->arg_num < 1) {
      /* Not enough arguments. */
      argp_usage(state); // NOLINT
    }
    break;
  default:
    return ARGP_ERR_UNKNOWN;
  }
  return 0;
}

static struct argp argp = {options, parse_opt, args_doc, doc};

int main(int argc, char **argv) {
  config = config_init();

  argp_parse(&argp, argc, argv, 0, 0, &config); // NOLINT
  mji_post();

  return config.err;
}

#endif
#endif

/**
 * When built with test
 */
#ifdef TEST

#include <scl/macros.h>

int main(int argc, char **argv) {
  const struct CMUnitTest tests[] = {NULL};
  return cmocka_run_group_tests(tests, NULL, NULL);
}

#endif

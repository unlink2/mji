
/**
 * When built without test
 */
#ifndef TEST

/// only use main if binary
#if TYPE == bin

#include <stdio.h>
#include <scl/scl.h>

int main(int argc, char **argv) {
  printf("Hello world!\n");
  printf("scl version %d\n", scl_version());
  return 0;
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

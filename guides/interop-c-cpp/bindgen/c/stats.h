#ifndef STATS_H
#define STATS_H

#include <stddef.h>

typedef struct {
    size_t count;
    double sum;
    double max;
} stats_t;

/* Fills *out from the first len values. Returns 0 on success, -1 if len is 0. */
int stats_compute(const double *values, size_t len, stats_t *out);

/* The result is owned by the library: do not free it. */
const char *stats_describe_status(int status);

#endif

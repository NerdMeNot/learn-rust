#include "stats.h"

int stats_compute(const double *values, size_t len, stats_t *out) {
    if (len == 0) return -1;
    out->count = len;
    out->sum = 0;
    out->max = values[0];
    for (size_t i = 0; i < len; i++) {
        out->sum += values[i];
        if (values[i] > out->max) out->max = values[i];
    }
    return 0;
}

const char *stats_describe_status(int status) {
    return status == 0 ? "ok" : "empty input";
}

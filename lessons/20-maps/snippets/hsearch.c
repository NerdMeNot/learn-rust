#include <search.h>
#include <stdio.h>

// POSIX hsearch: one global table, string keys, void* values.
int main(void) {
    hcreate(16);
    int ana = 30;
    ENTRY put = {"ana", &ana};
    hsearch(put, ENTER);

    ENTRY find = {"ana", NULL};
    ENTRY *hit = hsearch(find, FIND);
    printf("ana: %d\n", hit ? *(int *)hit->data : -1);

    ENTRY other = {"zed", NULL};
    printf("zed found: %s\n", hsearch(other, FIND) ? "yes" : "no");
    hdestroy();
    return 0;
}

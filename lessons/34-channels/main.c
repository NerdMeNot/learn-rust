#include <assert.h>
#include <pthread.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>

// C has no channels: a small queue guarded by a mutex and a condition variable
struct Channel {
    pthread_mutex_t lock;
    pthread_cond_t ready;
    char *items[8];      // a sketch: room for 8 messages in all, no wrap-around
    int head, tail;
    bool closed;
};

void send(struct Channel *ch, char *msg) {
    pthread_mutex_lock(&ch->lock);
    assert(ch->tail < 8);          // a real queue would wait here until there is room
    ch->items[ch->tail++] = msg;   // the receiver now owns msg
    pthread_cond_signal(&ch->ready);
    pthread_mutex_unlock(&ch->lock);
}

void close_channel(struct Channel *ch) {
    pthread_mutex_lock(&ch->lock);
    ch->closed = true;
    pthread_cond_broadcast(&ch->ready);   // wake every receiver
    pthread_mutex_unlock(&ch->lock);
}

// returns NULL once the channel is closed and empty
char *recv(struct Channel *ch) {
    pthread_mutex_lock(&ch->lock);
    while (ch->head == ch->tail && !ch->closed) {
        pthread_cond_wait(&ch->ready, &ch->lock);
    }
    char *msg = ch->head < ch->tail ? ch->items[ch->head++] : NULL;
    pthread_mutex_unlock(&ch->lock);
    return msg;
}

void *producer(void *arg) {
    struct Channel *ch = arg;
    for (int i = 1; i <= 3; i++) {
        char *msg = malloc(16);
        snprintf(msg, 16, "job %d", i);
        send(ch, msg);
    }
    close_channel(ch);
    return NULL;
}

int main(void) {
    struct Channel ch = {
        .lock = PTHREAD_MUTEX_INITIALIZER,
        .ready = PTHREAD_COND_INITIALIZER,
    };
    pthread_t t;
    pthread_create(&t, NULL, producer, &ch);

    char *msg;
    while ((msg = recv(&ch)) != NULL) {   // ends when the channel closes
        printf("got %s\n", msg);
        free(msg);
    }
    pthread_join(t, NULL);
    return 0;
}

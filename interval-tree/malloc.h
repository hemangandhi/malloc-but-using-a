#ifndef __INTERVAL_MALLOC_H
#define __INTERVAL_MALLOC_H

#ifndef HEAP_SIZE
#define HEAP_SIZE 10000
#endif

#ifndef size_t
typedef unsigned int size_t;
#endif

#ifndef NULL
#define NULL 0
#endif

void * malloc(size_t size);
void free();
void * realloc(void * original, size_t new_size);
void * calloc(size_t n_memb, size_t memb_size);

#endif

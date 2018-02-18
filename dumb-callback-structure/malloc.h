#ifndef __MALLOC_CALLBACK_INCL
#define __MALLOC_CALLBACK_INCL

#ifndef NULL
#define NULL 0
#endif

typedef void (* Callback)(void *, void *);
typedef size_t (* ReallocableCallback)(void *, void *);

typedef unsigned int size_t;

void malloc(size_t size, Callback onAlloc, void * args);
void calloc(size_t memb_size, size_t nmemb, Callback onAlloc, void * args);

void reallocableMalloc(size_t init_size, ReallocCallback onAllocs, void * args);

#endif

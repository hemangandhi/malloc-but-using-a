#include "malloc.h"
#include <alloca.h>

void malloc(size_t size, Callback onAlloc, void * args){
    onAlloc(alloca(size), args);
}

struct ZeroOutParams{
    size_t size;
    void * args;
    Callback ogCallBack;
};

static void zeroOut(void * alloced, void * params){
    struct ZeroOutParams * par = (struct ZeroOutParams *) params;

    int i;
    for(i = 0; i < par->size; i++){
        ((char *)alloced)[i] = 0;
    }
    par->ogCallBack(alloced, par->args);
}

void calloc(size_t memb_size, size_t nmemb, Callback onAlloc, void * args){
    struct ZeroOutParams par = {memb_size * nmemb, args, onAlloc};
    malloc(par.size, zeroOut, &par);
}

struct ReallocParams{
    void * ogArgs;
    ReallocCallback ogFn;
    size_t * nxtSize;
    void * upperFrame;
    size_t upperSize;
}

static void reallocCallback(void * alloced, void * arg){
    struct ReallocParams * par = (struct ReallocParams *) arg;
    *(par->nxtSize) = par->ogFn(alloced, par->ogArgs);
    int i;
    for(i = 0; i < par->upperSize; i++){
        ((char *)upperFrame)[i] = ((char *)alloced)[i];
    }
}

void reallocableMalloc(size_t init_size, ReallocCallback onAllocs, void * args){
    size_t rv = init_size;
    struct ReallocParams p = {args, onAllocs, &rv, NULL, rv};
    /* for(malloc(rv, reallocCallback, &p); rv > 0;  malloc(rv, reallocCallback, &p)); */
    do{
        void * upper;
        __asm__ volatile (
            "mov esp, %0\n"
            "sub esp, %1"
            : "=r" (upper)
            : "r" (rv)
        );

        p.upperFrame = upper;
        p.upperSize = rv;
        malloc(rv, reallocCallback, &p);

        __asm__ volatile (
            "add esp, %0"
            : "r" (rv)
        );
    }while(rv > 0);
}

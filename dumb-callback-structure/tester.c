#include "malloc.h"

typedef struct LLNode_ {
    struct LLNode_ * next;
    void * data;
} LLNode;

typedef void (* LLCallback) (LLNode *, void *);

struct AddParams{
    void * datum;
    LLNode * next;
    LLCallback og;
    void * ogArgs;
};

static void addCallback(void * alloced, void * arg){
    struct AddParams * par = (struct AddParams *) arg;
    LLNode * new = (LLNode *) alloced;
    new->data = par->datum;
    new->next = par->next;
    par->og(new, par->ogArgs);
}

void addToLL(LLNode * list, void * datum, LLCallback postAdd, void * postAddArgs){
    struct AddParams par = {datum, list, postAdd, postAddArgs};
    malloc(sizeof(LLNode), addCallback, &par);
}

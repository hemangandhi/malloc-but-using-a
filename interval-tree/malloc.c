#include "malloc.h"

static char MEM[HEAP_SIZE];

struct Node{
    void * mem;
    size_t size;
    size_t free_amt;
    struct Node * left, * right, * parent;
};

static struct Node * root = NULL;

void * malloc(size_t size){
    if(size < 0)
        return NULL;

    if(root == NULL){
        root = (struct Node *) MEM[HEAP_SIZE - sizeof(struct Node)];
        root->mem = MEM;
        root->size = HEAP_SIZE;
        root->free_amt = root->size - sizeof(struct Node);
        root->left = root->right = root->parent = NULL;
    }

    struct Node * find;
    for(find = root;
            find->left != NULL || find->right != NULL;
            find = (find->left != NULL &&
                       (find->right == NULL || find->left->free_amt > find->right->free_amt))?
                       find->left: find->right);

    //find now has the most space under it.
    if(find->free_amt < size + 2 * sizeof(struct Node)) return NULL;

    find->left = (struct Node *) find->mem;
    find->right = (struct Node *) (find->mem + sizeof(struct Node));

    find->left->mem = find->mem + 2*sizeof(struct Node);
    find->right->mem = find->left->mem + size;

    find->left->size = size;
    find->left->free_amt = 0;
    find->right->size = find->size - size - 2*sizeof(struct Node);
    find->right->free_amt = find->right->size;

    find->left->parent = find->right->parent = find;
    find->left->left = find->left->right = find->right->left = find->right->right = NULL;

    struct Node * t;
    for(t = find; t != NULL; t = t->parent) t->free_amt -= size + 2*sizeof(struct Node);

    return find->left->mem;
}

void * calloc(size_t n_memb, size_t memb_size){
    void * t = malloc(n_memb  * memb_size);
    if (t == NULL) return t;

    int i;
    for(i = 0; i < n_memb * memb_size; i++){
        * (char *) (t + i) = 0;
    }

    return t;
}

void free(void * ptr){
    struct Node * me = (struct Node *) (ptr - 2*sizeof(struct Node));

    if(me->parent->left != me || (me->parent->right) - me != sizeof(struct Node)){
        //corruption
        return;
    }

    struct Node * adj_free = me->parent->right;
    while(adj_free->free_amt < adj_free->size && adj_free->left != NULL) adj_free = adj_free->left;

    
}

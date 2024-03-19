#include<stdlib.h>
typedef struct I {
    int* x;
} I;

void drop(int *p) {
    free(p);
}

int main() {
    I i;
    int *p = malloc(sizeof(int));
    *p = 1;
    i.x = p;

    drop(p);

    *i.x = 5;
}



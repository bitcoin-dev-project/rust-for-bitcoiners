#include <stdlib.h>
#include <stdio.h>

int* f(int x) {
    int *p = malloc(sizeof(int));
    *p = x;
    return p;
}

int main() {
    int *p = f(5);
    printf("%d\n", *p);
}
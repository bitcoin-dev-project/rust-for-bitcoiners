#include <stdlib.h>
#include <stdio.h>

int* f(int x) {
    int *p = malloc(sizeof(int));
    *p = x;
    return p;
}

int* g() {
    int x=0;
    return &x;
}

int main() {
    int *p = f(5);
    int *q = g();
    printf("%d\n", *p);
    // *q = 7; // SEG Faulut
    int *t = malloc(sizeof(int) * 10);
    t[170000] = 9;
}
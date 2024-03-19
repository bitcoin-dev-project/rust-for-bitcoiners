#include<stdio.h>
int f(int a, int b) {
    /**
     * LOAD [a] r1
     * LOAD [b] r2
     * ADD r3 r1 r2
     * STORE r3 [return spot in the memory]
    */
    return a + b;
}

int main () {
    // 2 bits
    // -2 -> 10, -1 -> 11, 0-> 00, 1 -> 01
    // 2 
    // two x = 2; x = 10
    short x = 32768;
    int x = 6;
    int y = 5;
    int z = x + y; // z = 11
    printf("%hi", x);
    return 0;
}
/**
 * def f(a, b):
 *  return a + b
 * 
 * print(f(5,6))
*/
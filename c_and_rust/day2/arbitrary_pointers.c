#include<stdio.h>
int main() {
    int *x = (int *) 0x134323; // Referring a random memory, likely illegal
    printf("%d", *x); // Likely, SEG fault because OS with the help of hardware
                      // won't let a process access a memory which is not allocated to it
}
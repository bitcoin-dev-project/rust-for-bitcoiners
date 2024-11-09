#include <stdio.h>
#include <pthread.h>
#include <stdlib.h>

#define NUM_THREADS 5
#define NUM_INCREMENTS 1000000

int counter = 0; // Shared counter

// Function that each thread will execute
void* increment_counter(void* arg) {
    for (int i = 0; i < NUM_INCREMENTS; i++) {
        counter++;  // This operation is NOT atomic and causes data races
    }
    return NULL;
}

int main() {
    pthread_t threads[NUM_THREADS];

    // Create threads
    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_create(&threads[i], NULL, increment_counter, NULL) != 0) {
            perror("Failed to create thread");
            return 1;
        }
    }

    // Join threads
    for (int i = 0; i < NUM_THREADS; i++) {
        if (pthread_join(threads[i], NULL) != 0) {
            perror("Failed to join thread");
            return 1;
        }
    }

    // Print the final value of the counter
    printf("Final counter value: %d\n", counter);

    return 0;
}

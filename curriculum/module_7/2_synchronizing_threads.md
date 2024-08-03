# How to cordinate multiple threads

We learnt how to spawn threads and wait for them to finish. But sometimes we need to coordinate
threads to get the result that we want.

We need to learn how to share data among different threads.
Rust ensures memory and thread safety. So things are done differently in Rust.

NOTE: Every OS thread has it's own stack. If a process invokes 5 threads then all these threads
share the same address space which means any data accessible to a thread is accessible to another thread,
provided they belong to the same process. If a thread from process A tries to access the data belonging to
process B will result in segmentation fault, blue screen, process/app crash, system crash etc.,

## Can Rust threads share data stored in their stack to other threads?

The answer is NO! But why?
We saw that different threads in a process have different lifetimes.
Once a thread exits it's stack will be freed, which means it no longer belongs to that process.
So if thread A has a reference to some value in thread B's stack and if thread B exited but later
thread A tries to access that reference that would result in segmentation fault or undefined behaviour.
Rust compiler cannot guarante the runtime behaviour of the process so it prohibits this.

This is allowed in unsafe languages like C, C++.

## What is the safe way to share readable data among threads?

Definitely `Box` is not a candidate because it restricts a single owner.
`Rc` allows shared ownership, which allows multiple references to a value in heap.

### Whether Rc is thread safe?

NO! Because the reference counter is not meant to be used with multiple threads.
`Rc` is the efficient type to have shared ownership in a single threaded code.
[Arc](https://doc.rust-lang.org/std/sync/struct.Arc.html) type on the other hand has an *atomic reference counter* which synchronizes the update
of the counter variables among different CPU cores. Refer [atomic operations](https://en.wikipedia.org/wiki/Atomic_semantics).

`Arc` has the same semantics as `Rc` but can be safely shared with multiple threads.
Cehckout this example [sharing value using Arc](./demo/src/sharing_values.rs)

**What is move???**

The `move` keyword moves the variables from the current thread into the spawning thread.
In every loop `data_clone` variable is being moved into the ownership of the newly spawned thread.
Now each thread shares the ownership of the the `data` variable stored in the heap.
To explain further, `data` variable is owned by the main thread and the `data_clone` variable is
moved into the spawning thread.
So every time a thread exits the reference counter of the `Arc` type is decrementec atomically.


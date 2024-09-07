# Asynchronous programming in rust with tokio

In the last module we talked about the behaviour of threads.

## The program will only exit after all the futures are completed

When using tokio you don't have control to a main thread, instead you have a main future
from which you spawn or await on other futures. The main thread is controlled by the runtime.
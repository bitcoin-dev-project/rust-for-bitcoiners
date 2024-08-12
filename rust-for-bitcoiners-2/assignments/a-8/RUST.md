# Tips on writing with rust

1. Attend the lecture or watch the recording on how to use tokio.
1. make sure to await on the futures.
1. Divide the tasks into small helper functions for easier debug.
1. Test on regtest first with `-degug=net` option set, this will help you debug errors in your message serialization.
1. Check the endianness of the messages
1. Use `tracing::info or error` for logging. Don't use `dbg` or `println` as it won't give readable logs in multi threaded environment as we saw in last week.

## How to use TcpStream

It is like an `Iterator` except that not all the data are present in the beginning.
It is a `Future` in the essence that you have to keep polling using `read` method to look for data.
Use network magic bytes to mark the start of the message you received from the peer.
Use `bitcoin::consensus::{serialize, deserialize}` to send and receive messages in your rust code.


## Summary

I have added some code structure and pointers to make it easier for you.
You can add more functions and unit tests.

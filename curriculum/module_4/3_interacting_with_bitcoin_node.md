# Interacting with Bitcoin node using RPC

Through Bitcoin Core RPC Interface, we can interact with
a bitcoin node via command line using `bitcoin-cli` tool.
It is also possible to interact programmatically in rust by using `bitcoincore_rpc` crate.

### Capabilities of bitcoincore_rpc crate

This crate makes use of [jsonrpc](https://docs.rs/jsonrpc/0.18.0/jsonrpc/index.html) crate to interact with the bitcoin node.
RPC stands for Remote Procedure Call, which means the client interacts with server using the
procedure(function) semantics. The client will specify the function and the inputs required for them
in the message to the server and the server returns the result of the function in response.

Rust and C++ are statically typed programming languages, so the compiler itself can help in
validating types of inputs for the RPC stubs (similar to endpoints of REST APIs).

## Deep dive into bitcoincore_rpc crate

### [Client](https://docs.rs/bitcoincore-rpc/latest/bitcoincore_rpc/struct.Client.html) struct

It's a wrapper over jsonrpc module's [Client](https://docs.rs/jsonrpc/0.18.0/jsonrpc/client/struct.Client.html) type.
This struct has the necessary associated functions to interact with a bitcoin node.
There are two fundamental ways to create a value of this struct.

1. `new` method expects an `Auth` parameter which is essentially a wrapper over rpc username and password,
    it comes with defeault settings of `jsonrpc::client::Client` type, useful for most of the interactions
    with a bitcoin node.
1. `from_jsonrpc` method expects a value of `jsonrpc::client::Client` as input, this comes in handy when
    you require explicit control over how the client interacts with the node.

**When to use `from_jsonrpc`?**

If you examine the source code of `bitcoincore_rpc::Client`, you'll notice that it uses
[jsonrpc::client::Client::simple_http](https://docs.rs/jsonrpc/0.18.0/jsonrpc/client/struct.Client.html#method.simple_http) function to create the client with username and password details.

To have customized settings, one has to use [jsonrpc::client::Client::with_transpost](https://docs.rs/jsonrpc/0.18.0/jsonrpc/client/struct.Client.html#method.with_transport) function.
`with_transport` function expects as input types which implements [Transport](https://docs.rs/jsonrpc/0.18.0/jsonrpc/client/trait.Transport.html), trait defined in the *jsonrpc* crate.
jsonrpc crate re-exports simple_http crate, and it has a [Builder](https://docs.rs/jsonrpc/0.18.0/jsonrpc/http/simple_http/struct.Builder.html), which has methods
to create a value of type `SimpleHttpTransport` with custom timeouts, proxy and authentication settings.
The *jsonrpc* crate has `Transport` trait implementation for `SimpleHttpTransport` type, that is Rust
allows the programmers to extend the features of a type in a independent crate, rather than modifying the
initial source code.

One can also write a implementation of `Transport` trait for a new type and can make use of the functions
written in the *jsonrpc* crate.

In summary, some bitcoin core rpc calls takes longer time than usual to get the response.
If your program requires such calls then using `from_jsonrpc` function to create the `Client` is the way
to go.

## Significance of bitcoin crate

[bitcoin](https://docs.rs/bitcoin/latest/bitcoin/index.html) crate has a bunch of type definitions
for data-strucutres like block, transaction etc.,
*bitcoincore_rpc* crate re-uses *bitcoin* crate wherever possible.
*bitcoincore_rpc* crate uses *serde* crate to deserialize the json response into corresponding rust
types defined in *bitcoin* crate and other specific types in *bitcoincore_rpc* crate itself.

*bitcoin* crates [encode](https://docs.rs/bitcoin/latest/bitcoin/consensus/encode/index.html) module has the support to serialize a bitcoin type to consensus consistent byte of hex encoding and,
deseriazing from a hex or byte format to rust specific types defined in the crate.

## Conclusion

In this module you've learnt how to read and send message to a bitcoin node through bitcoin
core's rpc interface using rust.
Creating more involved messages like crafting a transaction, exporting data to a wallet etc.,
requires a dive deep into the *bitcoin* crate, which we will do in [module_5](../module_5/).
It is also possible to communicate with a bitcoin node through TCP, which gives us more flexibility.
Basics of it will be covered in [module_8](../module_8/).


## What is RPC?

In http, in URI you specify the resource, in the request body you specify some data.
REST API end point, you pass data to the endpoint.

RPC endpoints are treated like a function and you pass data like inputs.

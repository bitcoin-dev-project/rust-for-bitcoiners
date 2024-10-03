# Overview of [bitcoin](https://docs.rs/bitcoin/latest/bitcoin/) crate

This is a tutorial which will help you to read the documentation of the bitcoin crate better.

This is a widely used crate in rust projects which are trying to solve or implement new features
for bitcoin. This crate basically defines all the types required to respresent a bitcoin `Block` using
rust programming language, take a look at `Structs` and `Enums` listed in the home page of the docs.
It has a number of modules where these types are organised and relevant associated functions are defined
in a modular fashion.

Browse through the modules listed to get a better idea.

### Unifying Traits

Almost every very type defined in this module is used to represent the data in bitcoin consensus protocols.
So we need to be able to serialize them into that byte format or de-serialize the consensus conforming
data in byte format to the specific types which are defined in this crate.

The [Decodable](https://docs.rs/bitcoin/latest/bitcoin/consensus/trait.Decodable.html) and [Encodable](https://docs.rs/bitcoin/latest/bitcoin/consensus/trait.Encodable.html) traits seves that purpose.

So every type implements these traits, so we can serialize them into the corresponding byte format
or deserialize them from the byte format to the corresponding Rust type.

Armed with this information these are the following things you can do for example,

## Create a signed bitcoin transaction

#### Creating raw transaction

You have a bunch of UTXOs data for with respect to a wallet.
Investigate the [Transaction](https://docs.rs/bitcoin/latest/bitcoin/struct.Transaction.html) type, the only way to create this type is to provide the public fields indifidually.
Use the `Decodable` trait implementation of each of those individual type, get the corresponding Rust types
and you have the `Transaction` type.

#### sighash module

This module has [SigHashCache](https://docs.rs/bitcoin/latest/bitcoin/sighash/struct.SighashCache.html) type which can be created by providing the reference of a `Transaction`.
Then the necessary sighash based on the output script can be computed for segwit, taproot, legacy etc.,
Based on the hashing algorithm we get `TapSigHash`, `LegacySigHash` etc., which could be converted into
the `hashes::sha256d::Hash` type with a suitable function.

The `hashes` module is a re-export of [bitcoin_hashes]((https://docs.rs/bitcoin_hashes/0.14.0/bitcoin_hashes/sha256d/struct.Hash.html)).
Try to understand the design of the crate better by investing the `Hash` trait, every hashing type
implements this trait.

#### Signing the sighash

We need private keys to sign. This crate also re-exports [secp256k1](https://docs.rs/secp256k1/0.29.0/secp256k1/index.html) crate.
Use this to generate a new private key or create from existing private key data using suitable functions.

It also defines a [Secp256k1](https://docs.rs/secp256k1/0.29.0/secp256k1/struct.Secp256k1.html) type
has `sign_ecdsa` function which takes a message which is our sighash and corresponding private key
to give the signature.

This signature has to be inserted in the correct field of the Transaction to get the signed transaction,
which can be serialized into byte format and broadcasted to the bitcoin network.

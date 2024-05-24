Transaction:
    txid -> hash(List[TxIn] || List[TxOut])
    List[TxIn]
    List[TxOut]

TxIn:
    OutputPoint:
        Previous txid
        out_index
    (Signature, Script)

TxOut:
    Satoshis
    public address

Segwit Transaction:
    txid -> hash(List[TxIn] || List[TxOut])
    List[SegTxIn]
    List[TxOut]
    Witness data (List[(Signature, Script)])

SegTxIn:
    OutputPoint:
        Previous txid
        out_index
    NULL

Why Segwit was introduced?
* Scalability
* Transaction malleability

Digital signature is handled by ECDSA secp256k1
because the equations looks like y^2 = x^3 + 7.

new nodes will caluclate weight of a transaction like this

    3 * (txsize - witeness data size) + 1 * txsize

p2pk
p2pkh
p2sh is used to employ segwit initially and it is called native segwit


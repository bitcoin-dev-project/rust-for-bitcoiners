# Understanding bitcoin blockchain structure and storage capabilities

## What is a bitcoin node?

A computer which runs the bitcoin-core software.

## Where does the bitcoin blockchain is stored?

Every bitcoin node contains it's own copy of the entire bitcoin blockchain.
In most cases any two bitcoin nodes might differ in the last 6 blocks of the
bitcoin blockchain.

## What is a mempool?

A collection of unverified transactions.
Every bitcoin node has it's own version of mempool.

## What is a Satoshi?

1 BTC = 100 million satoshis = 1^8 satoshis.

## Bitcoin Miner rewards

A user who successfully mines the next block is rewared with certain amount of satoshis.
Initially the reward was 50 BTC.

## What is Genesis block?

Once there was Satoshi. He ran the bitcoin software first and mined the first block of
the blockchain. Initially there was no bitcoins, so the first block had only one transaction.
Which is the reward transaction, basically satoshi assigned himself 50 BTC to his public address.

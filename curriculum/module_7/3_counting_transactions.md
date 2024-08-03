# Counting the total number of bitcoin transactions

During the time of writing there was 855229 bitcoin blocks.
If you have a bitcoin node running you can invoke the number threads equal to your hardware threads,
each independently querying the bitcoin node to get the block data then count the number of transactions
in it and update them in a shared counter.

A simple strategy could be, if you have 16 hardware threads then spawn 16 threads.
Thread `i` should work on block heights whose remainder is `i` when divided by 16.

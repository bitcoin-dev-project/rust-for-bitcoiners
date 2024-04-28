hash=$(bitcoin-cli getblockhash 0)
bitcoin-cli getblock $hash 2 | jq .
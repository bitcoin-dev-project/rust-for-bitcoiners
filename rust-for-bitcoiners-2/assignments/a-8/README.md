# Bitcoin Protocol Development - Week 8: Connecting to P2P network

## Overview
In this challenge, you will interact with the Bitcoin P2P network. You will connect to a Bitcoin node, perform network handshakes, retrieve and parse block data, and extract relevant details from the parsed block.

## Objective
The objective of this challenge is to:
- **Establish a Connection to a Bitcoin Node:** You are required to discover and connect to a Bitcoin node. This will involve understanding the networking and communication protocols.
- **Perform Version and Verack Handshake:** You are required to perform the initial handshake with the node, including sending and receiving version and verack messages, to establish a valid connection.
- **Retrieve Block Data:** Using a known block hash obtained from a public blockchain explorer, you are required to request the block data from the connected node.
- **Extract and Output Block Details**:
  - **Block Header:** 80 byte block header
  - **Block Hash:** 32 byte block hash
  - **Total Fees Collected:** total fees collected by the miner
  - **Miner Info:** Information about the miner who mined the block

The output of your script should be a file named `out.txt` that follows a specific format.

Place your solution in the appropriate directory based on your chosen language:
- [bash](./bash/solution.sh)
- [javascript](./javascript/index.js)
- [python](./python/main.py)
- [rust](./rust/src/main.rs)

## Requirements
### Input
- You have to complete this challenge for block at the height of 840000.

### Output
Your script must generate an output file named `out.txt` with the following structure:
- First line: The block header.
- Second line: The block hash.
- Third line: Total fee collected in sats
- Fourth line: Miner Information

## Execution
To test your solution locally:
- Uncomment the line corresponding to your language in [run.sh](./run.sh).
- Execute [`local.sh`](./local.sh).

If your code works, you will see the test completed successfully.

## Evaluation Criteria
Your submission will be evaluated based on:
- **Autograder**: Your code must pass the autograder [test script](./test/sanity-checks.spec.ts).
- **Explainer Comments**: Include comments explaining each step of your code.
- **Code Quality**: Your code should be well-organized, commented, and adhere to best practices.

### Plagiarism Policy
Our plagiarism detection checker thoroughly identifies any instances of copying or cheating. Participants are required to publish their solutions in the designated repository, which is private and accessible only to the individual and the administrator. Solutions should not be shared publicly or with peers. In case of plagiarism, both parties involved will be directly disqualified to maintain fairness and integrity.

### AI Usage Disclaimer
You may use AI tools like ChatGPT to gather information and explore alternative approaches, but avoid relying solely on AI for complete solutions. Verify and validate any insights obtained and maintain a balance between AI assistance and independent problem-solving.

## Why These Restrictions?
These rules are designed to enhance your understanding of the technical aspects of Bitcoin. By completing this assignment, you gain practical experience with the technology that secures and maintains the trustlessness of Bitcoin. This challenge not only tests your ability to develop functional Bitcoin applications but also encourages deep engagement with the core elements of Bitcoin technology.
# NEAR Protocol <> Ethereum
===========================

This is my educational project to better understand how the Ethereum and NEAR Protocol works.
The essence of the project is to create a trustless bridge between two chains.

The bridge consists of four parts:
 - [Smart contracts in the NEAR](https://github.com/NutiNaguti/near-bridge)
 - [Smart contracts on the Ethereum](https://github.com/NutiNaguti/ethereum-bridge)
 - [Oracle](https://github.com/NutiNaguti/relayer)
 - [Frontend](https://github.com/NutiNaguti/crystal-bridge)

How it's work
-------------

1. The oracle receives block headers from Ethereum and takes only the bloom filter from there. Next, the oracle loads this data into the Lite Node contract in NEAR.
2. The user accesses the Ethereum smart contract and blocks his tokens there. At this moment, a new event is created in the smart contract function, which contains a hash of three values: block number, user address, and the number of tokens.
3. The user accesses the smart contract in NAP to get their tokens, and provides the hash generated in the event as proof of the locked tokens.
4. The smart contract checks for the event by 3 parameters in the bloom filter, and if the proof is valid, mint the specified number of tokens to the user's address.

Known issues and roadmap
------------------------

**! ! ! Yes, I understand that the Bloom filter is a probabilistic data structure and there is a chance of a false positive test result, but firstly, I considered the probability of such a response to be extremely small, and secondly, this is a training project and I strongly do not recommend using this principle in production ! ! !** 

 - [x] Build an Oracle
 - [ ] Implement sending tokens from Ethereum to NEAR
 - [ ] Implement sending tokens from NEAR to Ethereum

### Any suggestions and comments are welcome

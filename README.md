Night Bridge
==========================

<img src="https://www.svgrepo.com/show/225979/bridge-river.svg" align="left" width="150" heigth="150"/>

<div >
This is my educational project to better understand how the Ethereum and NEAR Protocol works.
The essence of the project is to create a trustless bridge between two chains.
</div>

<br>


**! ! ! Yes, I understand that the Bloom Filter is a probabilistic data structure and there is a chance of a false positive test result, but firstly, I considered the probability of such a response to be extremely small, and secondly, this is a training project and I strongly do not recommend using this principle in production ! ! !** 

<br/>

The bridge consists of four parts:
 - [Smart contracts in the NEAR](https://github.com/NutiNaguti/near-bridge)
 - [Smart contracts on the Ethereum](https://github.com/NutiNaguti/ethereum-bridge)
 - [Oracle](https://github.com/NutiNaguti/relayer)
 - [Frontend](https://github.com/NutiNaguti/crystal-bridge)
 
## Contracts 
 
Currently Ethereum and NEAR smart-contracts deployed in testnet (Goerly and Testnet).

| Contract | Address   | Description |
|    -     |     -     |       -     |
| FunCoin  | ETH: `0x918DD8e3F443C1a8535d0F6F266EC20E3a9329e2` <br/> NEAR: `dev-1669803669965-75235193778699` | ERC-20 token |
| Lite Node| NEAR: `dev-1669804361266-30686725939679` | Smart-contract storing Logs Bloom |
| Bridge   | ETH: `0x9431f9bba577B037D97ad6F7086a00eFB572c871` <br/> NEAR: `call dev-1669803549073-25511761548859` | Smart-contract validating the transer|


How it's work
-------------

1. The oracle receives block headers from Ethereum and takes only the bloom filter from there. Next, the oracle loads this data into the Lite Node contract in NEAR.
2. The user accesses the Ethereum smart contract and blocks his tokens there. At this moment, a new event is created in the smart contract function, which contains a hash of three values: block number, user address, and the number of tokens.
3. The user accesses the smart contract in NAP to get their tokens, and provides the hash generated in the event as proof of the locked tokens.
4. The smart contract checks for the event by three parameters in the bloom filter, and if the proof is valid, mint the specified number of tokens to the user's address.

Known issues and roadmap
------------------------

 - [ ] Build an Oracle
 - [x] Implement sending tokens from Ethereum to NEAR
 - [ ] Implement sending tokens from NEAR to Ethereum
 - [ ] Gas optimisations

### Any suggestions and comments are welcome

# Ethereum Basics

## Components that the Ethereum system is comprised of

### Accounts
Each account has a state associated with it and a 20-byte address. 

There are 2 types of accounts: 
1. Externally owned accounts 
These are controlled by private keys and have no code associated with them. An externally owned account can send messages to other externally owned accounts OR to tother contract accounts by creating and signing a transaction using its private key. 

2. Contract accounts
These are controlled by their contract code and have code associated with them. Contract accounts can't initiate new transactions on their own. Instead, contract accounts can only fire transactions in response to other transactions they have received (from an externally owned account or from another contract account).  

### Account state
The account state consists of four components

1. nonce
If the account is an externally owned account, this number represents the number of transactions sent from the account's address. 
If the account is a contract account, the nonce is the number of contracts created by the account.

2. balance
The number of Wei owned by this address. 

3. storageRoot
A hash of the root node of a Merkle Particia tree. It is empty by default. 

4. codeHash
The hash of the EVM code of this account. 

### World state
Ethereum's global state consists of a mapping between account addresses and the account states. This mapping is stored in a data structure known as a Merkle Patricia tree. 
A full archive node synchronizes the blockchain by downloading the full chain, from the genesis block to the current head block, executing all of the transactiosn contained within.

### Gas and payment
Every computation that occurs as a result of a transaction on the Ethereum network incurs a fee - there's no free lunch!
Not ony is gas used to pay for computation steps, it is also used to pay for storage usage. 


## Transaction and messages
Ethereum is a transaction-based state machine. 
In the most basic sense, a transaction is a cryptographically signed piece of instruction that is generated by an externally owned account, serialized, and then submitted to the blockchain. 

There are two types of transactions: **messeage calls** and **contract creations**(transactions that create new Ethereum contracts)

## Blocks
In Ethereum, a block consists of 
- the block header
- information about the set of transactios included in that block
- a set of other block headers for the current block's omers

### Ommers explained
An ommer is a block whose parent is equal to the current block's parent's parent. 

### Block header
- parentHash: a hash of the parent block's header (this is what makes the block set a "chain")
- ommersHash: a hash of the current block's list of ommers
- beneficiary: the account address that receives the fees for mining this block
- stateRoot: the hash of the root node of the state trie
- transactionsRoot
- receiptsRoot
- logsBloom: a Bloom filter (data structure) that consists of log information
- difficulty
- number
- gasLimit
- gasUsed
- timestamp
- extraData
- mixHash
- nonce

## Transaction Execution
All transactions must meet an initial set of requirements in order to be executed. 
- The transaction must be a properly formatted RLP. "RLP" stands for "Recursive Length Prefix" and is a data format used to encode nested array of binary data.
- Valid transaction signature
- Valid transaction nonce. 
- The transaction's gas limit must be equal to or greater than the intrinsic gas used by the transaction. The intrinsic gas includes:
1 a predefined cost of 21,000 gas for executing the transaction
2 a gas fee for data sent with the transaction (4 gas for every byte of data or code that equals zero, and 68 gas for every non-zero byte of data or code)
3 if the transactio is a cotract-creating transaction, an additional 32,000 gas
4 gas cost of each operation performed by transaction


### Contract creation
The transaction is not allowed to use up more gas than the remaining gas. If it does, the execution will hit an out-of-gas (OOG) exception and exit. If the transaction exits due to an out-of-gas exception, then the state is reverted to the point immediately prior to transaction. The sender is not refunded the gas that was spent before running out.

### Message calls

## Execution model
The part of the protocol that actually handles processing the transactions is Ethereum's own virtual machine, known as the Ethereum Virtual Machine (EVM)



## What is an uncle/ommer block?

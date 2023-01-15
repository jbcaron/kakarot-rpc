

## JSON-RPC API Methods

Based on this specification: [ethereum/execution-apis](https://github.com/ethereum/execution-apis)

### Method Implementation State
- ❌ -> TODO
- ⚠️ -> Logic created, to be verified
- ⏳ -> Logic verified, being implemented
- ✅ -> Implemented
- 🟡 -> Not respecting the specification

### Contribute
The template for the method file can be found [here](docs/contributing/method_template.md) copy it to the new method file and edit it corresponding to the method you're implementing.
All methods should be documented in `docs/methods/{method}.md`

| Name                                                                                            | Description                                                                                                                                                                                        | State |
| ----------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----- |
| [eth_chainId](docs/methods/eth_chainId)                                                         | Returns the chain ID of the current network.                                                                                                                                                       | ⚠️     |
| [eth_syncing](docs/methods/eth_syncing)                                                         | Returns an object with data about the sync status or false.version.                                                                                                                                | ❌     |
| [eth_coinbase](docs/methods/eth_coinbase)                                                       | Returns the client coinbase address.                                                                                                                                                               | ❌     |
| [eth_mining](docs/methods/eth_mining)                                                           | Returns true if client is actively mining new blocks.                                                                                                                                              | ❌     |
| [eth_hashrate](docs/methods/eth_hashrate)                                                       | Returns the number of hashes per second that the node is mining with.                                                                                                                              | ❌     |
| [eth_gasPrice](docs/methods/eth_gasPrice)                                                       | Returns the current price per gas in wei.                                                                                                                                                          | ❌     |
| [eth_accounts](docs/methods/eth_accounts)                                                       | Returns a list of addresses owned by client.                                                                                                                                                       | ❌     |
| [eth_blockNumber](docs/methods/eth_blockNumber)                                                 | Returns the number of most recent block.                                                                                                                                                           | ❌     |
| [eth_getBalance](docs/methods/eth_getBalances)                                                  | Returns the balance of the account of given address.                                                                                                                                               | ❌     |
| [eth_getStorageAt](docs/methods/eth_getStorageAt)                                               | Returns the value from a storage position at a given address.                                                                                                                                      | ❌     |
| [eth_getTransactionCount](docs/methods/eth_getTransactionCount)                                 | Returns the number of transactions sent from an address.                                                                                                                                           | ❌     |
| [eth_getBlockTransactionCountByHash](docs/methods/eth_getBlockTransactionCountByHash)           | Returns the number of transactions in a block from a block matching the given block hash.                                                                                                          | ❌     |
| [eth_getBlockTransactionCountByNumber](docs/methods/eth_getBlockTransactionCountByNumber)       | Returns the number of transactions in a block matching the given block number.                                                                                                                     | ❌     |
| [eth_getUncleCountByBlockHash](docs/methods/eth_getUncleCountByBlockHashs)                      | Returns the number of uncles in a block from a block matching the given block hash.                                                                                                                | ❌     |
| [eth_getUncleCountByBlockNumber](docs/methods/eth_getUncleCountByBlockNumber)                   | Returns the number of uncles in a block from a block matching the given block number.                                                                                                              | ❌     |
| [eth_getCode](docs/methods/eth_getCode)                                                         | Returns code at a given address.                                                                                                                                                                   | ❌     |
| [eth_sign](docs/methods/eth_sign)                                                               | The sign method calculates an Ethereum specific signature with: sign(keccak256("\x19Ethereum Signed Message:\n" + len(message) + message))).                                                       | ❌     |
| [eth_signTransaction](docs/methods/eth_signTransaction)                                         | Signs a transaction that can be submitted to the network at a later time using with eth_sendRawTransaction.                                                                                        | ❌     |
| [eth_sendTransaction](docs/methods/eth_sendTransaction)                                         | Creates new message call transaction or a contract creation, if the data field contains code.                                                                                                      | ❌     |
| [eth_sendRawTransaction](docs/methods/eth_sendRawTransaction)                                   | Creates new message call transaction or a contract creation for signed transactions.                                                                                                               | ❌     |
| [eth_call](docs/methods/eth_call)                                                               | Executes a new message call immediately without creating a transaction on the block chain.                                                                                                         | ❌     |
| [eth_estimateGas](docs/methods/eth_estimateGas)                                                 | Generates and returns an estimate of how much gas is necessary to allow the transaction to complete.                                                                                               | ❌     |
| [eth_getBlockByHash](docs/methods/eth_getBlockByHash)                                           | Returns information about a block by hash.                                                                                                                                                         | ❌     |
| [eth_getBlockByNumber](docs/methods/eth_getBlockByNumber)                                       | Returns information about a block by block number.                                                                                                                                                 | ❌     |
| [eth_getTransactionByHash](docs/methods/eth_getTransactionByHash)                               | Returns the information about a transaction requested by transaction hash.                                                                                                                         | ❌     |
| [eth_getTransactionByBlockHashAndIndex](docs/methods/eth_getTransactionByBlockHashAndIndex)     | Returns information about a transaction by block hash and transaction index position.                                                                                                              | ❌     |
| [eth_getTransactionByBlockNumberAndIndex](docs/methods/eth_getTransactionByBlockNumberAndIndex) | Returns information about a transaction by block number and transaction index position.                                                                                                            | ❌     |
| [eth_getTransactionReceipt](docs/methods/eth_getTransactionReceipt)                             | Returns the receipt of a transaction by transaction hash.                                                                                                                                          | ❌     |
| [eth_newFilter](docs/methods/eth_newFilter)                                                     | Creates a filter object, based on filter options, to notify when the state changes (logs). To check if the state has changed, call eth_getFilterChanges.                                           | ❌     |
| [eth_newBlockFilter](docs/methods/eth_newBlockFilter)                                           | Creates a filter in the node, to notify when a new block arrives. To check if the state has changed, call eth_getFilterChanges.                                                                    | ❌     |
| [eth_newPendingTransactionFilter](docs/methods/eth_newPendingTransactionFilter)                 | Creates a filter in the node, to notify when new pending transactions arrive. To check if the state has changed, call eth_getFilterChanges.                                                        | ❌     |
| [eth_uninstallFilter](docs/methods/eth_uninstallFilter)                                         | Uninstalls a filter with given id. Should always be called when watch is no longer needed. Additionally Filters timeout when they aren't requested with eth_getFilterChanges for a period of time. | ❌     |
| [eth_getFilterChanges](docs/methods/eth_getFilterChanges)                                       | Polling method for a filter, which returns an array of logs which occurred since last poll.                                                                                                        | ❌     |
| [eth_getFilterLogs](docs/methods/eth_getFilterLogs)                                             | Returns an array of all logs matching filter with given id.                                                                                                                                        | ❌     |
| [eth_getLogs](docs/methods/eth_getLogs)                                                         | Returns an array of all logs matching a given filter object.                                                                                                                                       | ❌     |
| [eth_getWork](docs/methods/eth_getWork)                                                         | Returns the hash of the current block, the seedHash, and the boundary condition to be met ("target").                                                                                              | ❌     |
| [eth_submitWork](docs/methods/eth_submitWork)                                                   | Used for submitting a proof-of-work solution.                                                                                                                                                      | ❌     |
| [eth_createAccessList](docs/methods/eth_createAccessList)                                       | Generates an access list for a transaction.                                                                                                                                                        | ❌     |
| [eth_maxPriorityFeePerGas](docs/methods/eth_maxPriorityFeePerGas)                               | Returns the current maxPriorityFeePerGas per gas in wei.                                                                                                                                           | ❌     |
| [eth_feeHistory](docs/methods/eth_feeHistory)                                                   | Returns transaction base fee per gas and effective priority fee per gas for the requested/supported block range.                                                                                   | ❌     |
| [eth_feeHistory](docs/methods/eth_feeHistory)                                                   | Returns transaction base fee per gas and effective priority fee per gas for the requested/supported block range.                                                                                   | ❌     |
| [eth_getProof](docs/methods/eth_getProof)                                                       | Returns the merkle proof for a given account and optionally some storage keys.                                                                                                                     | ❌     |
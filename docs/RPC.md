# RPC document

## RPC list
- [chain_getBestNumber](#chain_getBestNumber)
- [chain_getFinalizedNumber](#chain_getFinalizedNumber)

## chain_getBestNumber

Get the best number of the chain

### Parameters
 - `shard_num`
 
```asm
params: [
   0
]
```

### Returns
`block_number`

### Example
```
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"chain_getBestNumber","params":[0],"id":1}' localhost:10055 -H 'Content-Type: application/json'

// Result
{
  "jsonrpc": "2.0",
  "result": 71,
  "id": 1
}

```

## chain_getFinalizedNumber

Get the finalized number of the chain

### Parameters
 - `shard_num`
 
```asm
params: [
   0
]
```

### Returns
`finalized_number`

### Example
```
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"chain_getFinalizedNumber","params":[0],"id":1}' localhost:10055 -H 'Content-Type: application/json'

// Result
{
  "jsonrpc": "2.0",
  "result": 65,
  "id": 1
}

```

# RPC document

## RPC list
- [chain_getBestNumber](#chain_getBestNumber)
- [chain_getFinalizedNumber](#chain_getFinalizedNumber)
- [chain_getHeaderByNumber](#chain_getHeaderByNumber)


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

## chain_getHeaderByNumber

Get the header by block number

### Parameters
 - `shard_num`
 - `number`
 
```asm
params: [
    0,
    65
]
```

### Returns
`header`
 - `blockHash`
 - `extrinsicsRoot`
 - `number`
 - `parentHash`
 - `stateRoot`

### Example
```
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"chain_getHeaderByNumber","params":[0, 65],"id":1}' localhost:10055 -H 'Content-Type: application/json'

// Result
{
  "jsonrpc": "2.0",
  "result": {
    "blockHash": "0xb4d35a4b1c5af319ab5922bbcfb946905248c4b174682abb61b5c090cdda16d5",
    "extrinsicsRoot": "0x242916f3dad7735e350f2541d3bf1e4c747635b61fda11443dbaa81aa48a2d70",
    "number": 65,
    "parentHash": "0x68ab5ba6d5e1992cc65f66c36a21fc10ea9513a173737c4a02063d6be666029f",
    "stateRoot": "0xa27664b888bed276ba67e0ee1cca674976ab4549992491ceb09cd36af12979b4"
  },
  "id": 1
}

```

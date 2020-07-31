# RPC document

## RPC list
- [chain_getBestNumber](#chain_getBestNumber)
- [chain_getFinalizedNumber](#chain_getFinalizedNumber)
- [chain_getHeaderByNumber](#chain_getHeaderByNumber)
- [chain_getBlockByNumber](#chain_getBlockByNumber)
- [chain_getBlockByHash](#chain_getBlockByHash)

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
 - `block_hash`
 - `extrinsics_root`
 - `number`
 - `parent_hash`
 - `state_root`

### Example
```
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"chain_getHeaderByNumber","params":[0, 65],"id":1}' localhost:10055 -H 'Content-Type: application/json'

// Result
{
  "jsonrpc": "2.0",
  "result": {
    "block_hash": "0xb4d35a4b1c5af319ab5922bbcfb946905248c4b174682abb61b5c090cdda16d5",
    "extrinsics_root": "0x242916f3dad7735e350f2541d3bf1e4c747635b61fda11443dbaa81aa48a2d70",
    "number": 65,
    "parent_hash": "0x68ab5ba6d5e1992cc65f66c36a21fc10ea9513a173737c4a02063d6be666029f",
    "state_root": "0xa27664b888bed276ba67e0ee1cca674976ab4549992491ceb09cd36af12979b4"
  },
  "id": 1
}

```

## chain_getBlockByNumber

Get the block by block number

### Parameters
 - `shard_num`
 - `number`
 
```asm
params: [
    0,
    98
]
```

### Returns
`block`
 - `header`
 - `extrinsics`: Array of `extrinsic`
 
`header`
 - `block_hash`
 - `extrinsics_root`
 - `number`
 - `parent_hash`
 - `state_root`
 
`extrinsic`
 - `call`
 - `hash`
 - `index`
 - `signature`
 - `success`
 
`call`
 - `module`
 - `method`
 - `params`
 
`signature`
 - `era`
 - `nonce`
 - `sender`
 - `signature`

### Example
```
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"chain_getBlockByNumber","params":[0, 121],"id":1}' localhost:10055 -H 'Content-Type: application/json'

// Result
{
  "jsonrpc": "2.0",
  "result": {
    "extrinsics": [
      {
        "call": {
          "method": 0,
          "module": 0,
          "params": {
            "now": 1596125229
          }
        },
        "hash": "0xd5fdce0a3cf253abaa856c21cdf64a57d33df35b163b004e25c57d7469d66c98",
        "index": 0,
        "signature": null,
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 2,
          "params": {
            "info": {
              "coinbase": "0x1033e0576822a6a836f612a193036042050e286da4561f5cc5d8ee560c64dc54",
              "reward_condition": "Normal"
            }
          }
        },
        "hash": "0x7d741e7c792e5aa8aad2f1d5793b3f275c62d0a8dfaac23296728451c7fb41b0",
        "index": 1,
        "signature": null,
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 5,
          "params": {
            "info": {
              "count": 4,
              "num": 0,
              "scale_out": null
            }
          }
        },
        "hash": "0x103f752f00d4c217e7c299f4e7de89e041ca25b6ea394d3fa47cfc1157537b00",
        "index": 2,
        "signature": null,
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 6,
          "params": {
            "info": "0x58c35a5978bc6b24e6c4b2957335cd834212af9e5f7117b8f03e81075809dde0"
          }
        },
        "hash": "0x78c2005bd842aaf5fd91e43ffc992c5cfbed7da3ef35b3caae36e06599c93ba6",
        "index": 3,
        "signature": null,
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 7,
          "params": {
            "hint": 114
          }
        },
        "hash": "0x013130cce060f5ee06ec96c2cd9362e47305c31eca26aaf9c3b7b3858cdd9440",
        "index": 4,
        "signature": null,
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 4,
          "params": {
            "dest": "0xff94d988b42d96dcbd6605ff47f19c6ab35f626eb1bc8bbd28f59a74997a253a3d",
            "value": 100000000
          }
        },
        "hash": "0x9a3d2d9aac88964da0d3efc36ae9de85f728f0cba6043bd84d6573b3735e5c7f",
        "index": 5,
        "signature": {
          "era": {
            "Mortal": [
              64,
              56
            ]
          },
          "nonce": 0,
          "sender": "0xff1033e0576822a6a836f612a193036042050e286da4561f5cc5d8ee560c64dc54",
          "signature": "0x3cba8ec56b55217f962cdb458f0b9b7cc303f8692e0480d7b7ff717618253034e5f632e8fe280837745970a1f0a17b7a7fd104b0d81f5bb8491bfd5c4b422e0f"
        },
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 9,
          "params": {
            "hash": "0xf61c24cee7371133a2295686e6d3c4f0bf3499674d9e0ed1b0d26d5e211bebf5",
            "number": 89,
            "parent": "0xe818f6fdf28768f4070fe337245249a584f84dea0ef2a90f2918042625254362",
            "relay_type": "Balance",
            "tx": "0x310281ff66122af0ca54d09bd572c01d1b9df1aaa1a139d414c2e9ef3a519a307c85fb192e5fca8b8793c5e413d1f8d89eb00d99f9b881de3863378bb2a8f4f49c3fb628b1bdfa4070297edbbde830c281ed9996c9c6b706b47f82ef94c0346d9878a40c0075010400ff1033e0576822a6a836f612a193036042050e286da4561f5cc5d8ee560c64dc540284d717"
          }
        },
        "hash": "0x3d916abc0dedc04e352289ff667ee83811ae9b5fc86da9eb1c793bf9d80e0d17",
        "index": 6,
        "signature": null,
        "success": true
      }
    ],
    "header": {
      "block_hash": "0x5a24c47b94b05a13b94e8b9561724fb49eb2725e4d951ad791d0282d20f0667b",
      "extrinsics_root": "0x66844dd11a095dcd09788e823056111844379e1af31d1e53768deed8580c5a1c",
      "number": 121,
      "parent_hash": "0x6f08c4822a8c277156a257d1c8118ac38ce5ad4c353efc27f0efe224507dbd46",
      "state_root": "0xc3e8c83d90e57f5574829faf79f1bba566350f2251189088a51ec6880d7e7fc6"
    }
  },
  "id": 1
}

```

## chain_getBlockByHash

Get the block by block hash

### Parameters
 - `shard_num`
 - `hash`
 
```asm
params: [
    0,
    '0xeed3ef98e847cc817b8e682fe11e63fd86050d6699e484eb75ff28fe9580bd7a'
]
```

### Returns
reference `chain_getBlockByNumber`

### Example
```
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"chain_getBlockByHash","params":[0, "0xeed3ef98e847cc817b8e682fe11e63fd86050d6699e484eb75ff28fe9580bd7a"],"id":1}' localhost:10055 -H 'Content-Type: application/json'

// Result
{
  "jsonrpc": "2.0",
  "result": {
    "extrinsics": [
      {
        "call": {
          "method": 0,
          "module": 0,
          "params": {
            "now": 1596125229
          }
        },
        "hash": "0xd5fdce0a3cf253abaa856c21cdf64a57d33df35b163b004e25c57d7469d66c98",
        "index": 0,
        "signature": null,
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 2,
          "params": {
            "info": {
              "coinbase": "0x1033e0576822a6a836f612a193036042050e286da4561f5cc5d8ee560c64dc54",
              "reward_condition": "Normal"
            }
          }
        },
        "hash": "0x7d741e7c792e5aa8aad2f1d5793b3f275c62d0a8dfaac23296728451c7fb41b0",
        "index": 1,
        "signature": null,
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 5,
          "params": {
            "info": {
              "count": 4,
              "num": 0,
              "scale_out": null
            }
          }
        },
        "hash": "0x103f752f00d4c217e7c299f4e7de89e041ca25b6ea394d3fa47cfc1157537b00",
        "index": 2,
        "signature": null,
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 6,
          "params": {
            "info": "0x58c35a5978bc6b24e6c4b2957335cd834212af9e5f7117b8f03e81075809dde0"
          }
        },
        "hash": "0x78c2005bd842aaf5fd91e43ffc992c5cfbed7da3ef35b3caae36e06599c93ba6",
        "index": 3,
        "signature": null,
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 7,
          "params": {
            "hint": 114
          }
        },
        "hash": "0x013130cce060f5ee06ec96c2cd9362e47305c31eca26aaf9c3b7b3858cdd9440",
        "index": 4,
        "signature": null,
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 4,
          "params": {
            "dest": "0xff94d988b42d96dcbd6605ff47f19c6ab35f626eb1bc8bbd28f59a74997a253a3d",
            "value": 100000000
          }
        },
        "hash": "0x9a3d2d9aac88964da0d3efc36ae9de85f728f0cba6043bd84d6573b3735e5c7f",
        "index": 5,
        "signature": {
          "era": {
            "Mortal": [
              64,
              56
            ]
          },
          "nonce": 0,
          "sender": "0xff1033e0576822a6a836f612a193036042050e286da4561f5cc5d8ee560c64dc54",
          "signature": "0x3cba8ec56b55217f962cdb458f0b9b7cc303f8692e0480d7b7ff717618253034e5f632e8fe280837745970a1f0a17b7a7fd104b0d81f5bb8491bfd5c4b422e0f"
        },
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 9,
          "params": {
            "hash": "0xf61c24cee7371133a2295686e6d3c4f0bf3499674d9e0ed1b0d26d5e211bebf5",
            "number": 89,
            "parent": "0xe818f6fdf28768f4070fe337245249a584f84dea0ef2a90f2918042625254362",
            "relay_type": "Balance",
            "tx": "0x310281ff66122af0ca54d09bd572c01d1b9df1aaa1a139d414c2e9ef3a519a307c85fb192e5fca8b8793c5e413d1f8d89eb00d99f9b881de3863378bb2a8f4f49c3fb628b1bdfa4070297edbbde830c281ed9996c9c6b706b47f82ef94c0346d9878a40c0075010400ff1033e0576822a6a836f612a193036042050e286da4561f5cc5d8ee560c64dc540284d717"
          }
        },
        "hash": "0x3d916abc0dedc04e352289ff667ee83811ae9b5fc86da9eb1c793bf9d80e0d17",
        "index": 6,
        "signature": null,
        "success": true
      }
    ],
    "header": {
      "block_hash": "0x5a24c47b94b05a13b94e8b9561724fb49eb2725e4d951ad791d0282d20f0667b",
      "extrinsics_root": "0x66844dd11a095dcd09788e823056111844379e1af31d1e53768deed8580c5a1c",
      "number": 121,
      "parent_hash": "0x6f08c4822a8c277156a257d1c8118ac38ce5ad4c353efc27f0efe224507dbd46",
      "state_root": "0xc3e8c83d90e57f5574829faf79f1bba566350f2251189088a51ec6880d7e7fc6"
    }
  },
  "id": 1
}

```

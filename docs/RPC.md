# RPC document

## RPC list
- [chain_getBestNumber](#chain_getBestNumber)
- [chain_getFinalizedNumber](#chain_getFinalizedNumber)
- [chain_getHeaderByNumber](#chain_getHeaderByNumber)
- [chain_getHeaderByHash](#chain_getHeaderByHash)
- [chain_getBlockByNumber](#chain_getBlockByNumber)
- [chain_getBlockByHash](#chain_getBlockByHash)
- [chain_getExtrinsicByHash](#chain_getExtrinsicByHash)
- [chain_getExtrinsicByRaw](#chain_getExtrinsicByRaw)
- [chain_getExtrinsicByOriginHash](#chain_getExtrinsicByOriginHash)
- [state_getNonce](#state_getNonce)
- [author_submitExtrinsic](#author_submitExtrinsic)

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
    "block_hash": "0x2ba6352cace11de7b9bb37f3afb72cad3f2c20e21a77f107f3bd17f763a6e807",
    "extrinsics_root": "0xd01be7376faa2133af91c920de770d5ab01e7524b347d25a655a6ea16a9f6c9c",
    "number": 65,
    "parent_hash": "0x713b7bffd160970863eb454ab912486ad0da6222862da1ea48f87e65edad265c",
    "state_root": "0xb85dd1f68e3d1c979840c53f03640938dc6da844a64d9057905865560bbbd3e1"
  },
  "id": 1
}

```

## chain_getHeaderByHash

Get the header by block hash

### Parameters
 - `shard_num`
 - `hash`
 
```asm
params: [
    0,
    "0xeed3ef98e847cc817b8e682fe11e63fd86050d6699e484eb75ff28fe9580bd7a"
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
curl -X POST --data '{"jsonrpc":"2.0","method":"chain_getHeaderByHash","params":[0, "0xeed3ef98e847cc817b8e682fe11e63fd86050d6699e484eb75ff28fe9580bd7a"],"id":1}' localhost:10055 -H 'Content-Type: application/json'

// Result
{
  "jsonrpc": "2.0",
  "result": {
    "block_hash": "0xeed3ef98e847cc817b8e682fe11e63fd86050d6699e484eb75ff28fe9580bd7a",
    "extrinsics_root": "0x30f1d15224e9e286737b8604f80bf92734dff43a2fb866651fecff6a7c131afd",
    "number": 98,
    "parent_hash": "0x3c8f6a9a88c8b04e4699c8c45a7729b1aa5ce23ac177fbb963b6e0064440e5f5",
    "state_root": "0x40b868ceb9a1c833e0f685b0fd250f371ec4da335b05189ef52caf459e2ac9a8"
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
 - `sender_address`
 - `sender_shard_num`
 - `signature`

### Example
```
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"chain_getBlockByNumber","params":[0, 394],"id":1}' localhost:10055 -H 'Content-Type: application/json'

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
            "now": 1596874295
          }
        },
        "hash": "0x033460364355b535bc1af8d55bede9eeb7fde511fe033e4b379156f32ff15e34",
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
              "coinbase": "0x44a5fbc7fff3cb26358e6070fbe7e171f978a5bf6de3dcc7ba359bb783916d44",
              "reward_condition": "Normal"
            }
          }
        },
        "hash": "0x274e0944bd0fa0626b6beefd60da6692b7e7340e606865d50ee1c43c2053315b",
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
            "info": "0xc260e48a949ae9fdcfc3386d82b59fa3cb5c9532323cdb48273bf1d1d6f635d9"
          }
        },
        "hash": "0x6fef63024a5783cc998dec92ef2b781e97a261a964bc13667e9ff811ca7e11fe",
        "index": 3,
        "signature": null,
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 7,
          "params": {
            "hint": 387
          }
        },
        "hash": "0x4ce997a6efd74701a5e39205e10307c46fe33366b1ae6a53c307ec2a67ad68ec",
        "index": 4,
        "signature": null,
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 4,
          "params": {
            "dest": "0xffc49bc1483a1669d65b19274445cb86604b7eca1d8e8d062269c8c6796a45b625",
            "dest_address": "yee1cjduzjp6ze5avkceyazytjuxvp9hajsa36xsvgnferr8j6j9kcjsnuzkdc",
            "dest_shard_num": 1,
            "value": 10000000000
          }
        },
        "hash": "0xeca31494ab0fd0dfbf5927f398e4ca3b9766c4d674f9d64bd4344c67e56b2e9b",
        "index": 5,
        "signature": {
          "era": {
            "Mortal": [
              64,
              9
            ]
          },
          "nonce": 1,
          "sender": "0xff36b116bcdeff6bf63539cea3cafdd90bb53d6df043b2ef791d234c92ca5de804",
          "sender_address": "yee1x6c3d0x7la4lvdfee63u4lwepw6n6m0sgwew77gaydxf9jjaaqzqzzu8dj",
          "sender_shard_num": 0,
          "signature": "0x80ba2ca34dfe11d120a8c610534887312c79e5c247da9b4f31ea7495a4376f6a9512d3f0b771c923142c46dc33ef6f924f86b8f7bcd1749eb2e15aa388bddb09"
        },
        "success": true
      }
    ],
    "header": {
      "block_hash": "0xc34449ad91dfa044c4d314b1b22762189bb3ad4a8577a9050e90e443f3550afc",
      "extrinsics_root": "0x549cf14e3874c61be194dd27e3e930891daf2a1dd4cd60ed7fc39a0923c4a261",
      "number": 394,
      "parent_hash": "0xf78eb90a94e881b488b8c3a81905d0424e2c55834a819164c7afba2594f43318",
      "state_root": "0x488c4a38429adbe510bf2e37230244f795638b4964d2bfcee6649e38b3343a21"
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
curl -X POST --data '{"jsonrpc":"2.0","method":"chain_getBlockByHash","params":[0, "0xc34449ad91dfa044c4d314b1b22762189bb3ad4a8577a9050e90e443f3550afc"],"id":1}' localhost:10055 -H 'Content-Type: application/json'

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
            "now": 1596874295
          }
        },
        "hash": "0x033460364355b535bc1af8d55bede9eeb7fde511fe033e4b379156f32ff15e34",
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
              "coinbase": "0x44a5fbc7fff3cb26358e6070fbe7e171f978a5bf6de3dcc7ba359bb783916d44",
              "reward_condition": "Normal"
            }
          }
        },
        "hash": "0x274e0944bd0fa0626b6beefd60da6692b7e7340e606865d50ee1c43c2053315b",
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
            "info": "0xc260e48a949ae9fdcfc3386d82b59fa3cb5c9532323cdb48273bf1d1d6f635d9"
          }
        },
        "hash": "0x6fef63024a5783cc998dec92ef2b781e97a261a964bc13667e9ff811ca7e11fe",
        "index": 3,
        "signature": null,
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 7,
          "params": {
            "hint": 387
          }
        },
        "hash": "0x4ce997a6efd74701a5e39205e10307c46fe33366b1ae6a53c307ec2a67ad68ec",
        "index": 4,
        "signature": null,
        "success": true
      },
      {
        "call": {
          "method": 0,
          "module": 4,
          "params": {
            "dest": "0xffc49bc1483a1669d65b19274445cb86604b7eca1d8e8d062269c8c6796a45b625",
            "dest_address": "yee1cjduzjp6ze5avkceyazytjuxvp9hajsa36xsvgnferr8j6j9kcjsnuzkdc",
            "dest_shard_num": 1,
            "value": 10000000000
          }
        },
        "hash": "0xeca31494ab0fd0dfbf5927f398e4ca3b9766c4d674f9d64bd4344c67e56b2e9b",
        "index": 5,
        "signature": {
          "era": {
            "Mortal": [
              64,
              9
            ]
          },
          "nonce": 1,
          "sender": "0xff36b116bcdeff6bf63539cea3cafdd90bb53d6df043b2ef791d234c92ca5de804",
          "sender_address": "yee1x6c3d0x7la4lvdfee63u4lwepw6n6m0sgwew77gaydxf9jjaaqzqzzu8dj",
          "sender_shard_num": 0,
          "signature": "0x80ba2ca34dfe11d120a8c610534887312c79e5c247da9b4f31ea7495a4376f6a9512d3f0b771c923142c46dc33ef6f924f86b8f7bcd1749eb2e15aa388bddb09"
        },
        "success": true
      }
    ],
    "header": {
      "block_hash": "0xc34449ad91dfa044c4d314b1b22762189bb3ad4a8577a9050e90e443f3550afc",
      "extrinsics_root": "0x549cf14e3874c61be194dd27e3e930891daf2a1dd4cd60ed7fc39a0923c4a261",
      "number": 394,
      "parent_hash": "0xf78eb90a94e881b488b8c3a81905d0424e2c55834a819164c7afba2594f43318",
      "state_root": "0x488c4a38429adbe510bf2e37230244f795638b4964d2bfcee6649e38b3343a21"
    }
  },
  "id": 1
}

```

## chain_getExtrinsicByHash

Get the extrinsic by block number and extrinsic hash

### Parameters
 - `shard_num`
 - `block_number`
 - `extrinsic_hash`
 
```asm
params: [
    0,
    121,
    "0x9a3d2d9aac88964da0d3efc36ae9de85f728f0cba6043bd84d6573b3735e5c7f",
]
```

### Returns 
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
 - `sender_address`
 - `sender_shard_num`
 - `signature`

### Example
```
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"chain_getExtrinsicByHash","params":[0, 394, "0xeca31494ab0fd0dfbf5927f398e4ca3b9766c4d674f9d64bd4344c67e56b2e9b"],"id":1}' localhost:10055 -H 'Content-Type: application/json'

// Result
{
  "jsonrpc": "2.0",
  "result": {
    "call": {
      "method": 0,
      "module": 4,
      "params": {
        "dest": "0xffc49bc1483a1669d65b19274445cb86604b7eca1d8e8d062269c8c6796a45b625",
        "dest_address": "yee1cjduzjp6ze5avkceyazytjuxvp9hajsa36xsvgnferr8j6j9kcjsnuzkdc",
        "dest_shard_num": 1,
        "value": 10000000000
      }
    },
    "hash": "0xeca31494ab0fd0dfbf5927f398e4ca3b9766c4d674f9d64bd4344c67e56b2e9b",
    "index": 5,
    "signature": {
      "era": {
        "Mortal": [
          64,
          9
        ]
      },
      "nonce": 1,
      "sender": "0xff36b116bcdeff6bf63539cea3cafdd90bb53d6df043b2ef791d234c92ca5de804",
      "sender_address": "yee1x6c3d0x7la4lvdfee63u4lwepw6n6m0sgwew77gaydxf9jjaaqzqzzu8dj",
      "sender_shard_num": 0,
      "signature": "0x80ba2ca34dfe11d120a8c610534887312c79e5c247da9b4f31ea7495a4376f6a9512d3f0b771c923142c46dc33ef6f924f86b8f7bcd1749eb2e15aa388bddb09"
    },
    "success": true
  },
  "id": 1
}

```

## chain_getExtrinsicByRaw

Get the extrinsic by block number and extrinsic raw

### Parameters
 - `shard_num`
 - `block_number`
 - `extrinsic_raw`
 
```asm
params: [
    0,
    121,
    "0x310281ff1033e0576822a6a836f612a193036042050e286da4561f5cc5d8ee560c64dc543cba8ec56b55217f962cdb458f0b9b7cc303f8692e0480d7b7ff717618253034e5f632e8fe280837745970a1f0a17b7a7fd104b0d81f5bb8491bfd5c4b422e0f0085030400ff94d988b42d96dcbd6605ff47f19c6ab35f626eb1bc8bbd28f59a74997a253a3d0284d717",
]
```

### Returns 
reference `chain_getExtrinsicByHash`


### Example
```
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"chain_getExtrinsicByHash","params":[0, 394, "0x390281ff36b116bcdeff6bf63539cea3cafdd90bb53d6df043b2ef791d234c92ca5de80480ba2ca34dfe11d120a8c610534887312c79e5c247da9b4f31ea7495a4376f6a9512d3f0b771c923142c46dc33ef6f924f86b8f7bcd1749eb2e15aa388bddb090495000400ffc49bc1483a1669d65b19274445cb86604b7eca1d8e8d062269c8c6796a45b6250700e40b5402"],"id":1}' localhost:10055 -H 'Content-Type: application/json'

// Result
{
  "jsonrpc": "2.0",
  "result": {
    "call": {
      "method": 0,
      "module": 4,
      "params": {
        "dest": "0xffc49bc1483a1669d65b19274445cb86604b7eca1d8e8d062269c8c6796a45b625",
        "dest_address": "yee1cjduzjp6ze5avkceyazytjuxvp9hajsa36xsvgnferr8j6j9kcjsnuzkdc",
        "dest_shard_num": 1,
        "value": 10000000000
      }
    },
    "hash": "0xeca31494ab0fd0dfbf5927f398e4ca3b9766c4d674f9d64bd4344c67e56b2e9b",
    "index": 5,
    "signature": {
      "era": {
        "Mortal": [
          64,
          9
        ]
      },
      "nonce": 1,
      "sender": "0xff36b116bcdeff6bf63539cea3cafdd90bb53d6df043b2ef791d234c92ca5de804",
      "sender_address": "yee1x6c3d0x7la4lvdfee63u4lwepw6n6m0sgwew77gaydxf9jjaaqzqzzu8dj",
      "sender_shard_num": 0,
      "signature": "0x80ba2ca34dfe11d120a8c610534887312c79e5c247da9b4f31ea7495a4376f6a9512d3f0b771c923142c46dc33ef6f924f86b8f7bcd1749eb2e15aa388bddb09"
    },
    "success": true
  },
  "id": 1
}

```

## chain_getExtrinsicByOriginHash

Get the relay extrinsic by block number range and origin extrinsic hash

### Parameters
 - `shard_num`
 - `from_block_number`: inclusive
 - `to_block_number`: inclusive
 - `origin_hash`
 
```asm
params: [
    1,
    400,
    500,
    "0xeca31494ab0fd0dfbf5927f398e4ca3b9766c4d674f9d64bd4344c67e56b2e9b",
]
```

### Returns 
reference `chain_getExtrinsicByHash`


### Example
```
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"chain_getExtrinsicByOriginHash","params":[1, 400, 500, "0xeca31494ab0fd0dfbf5927f398e4ca3b9766c4d674f9d64bd4344c67e56b2e9b"],"id":1}' localhost:10055 -H 'Content-Type: application/json'

// Result
{
  "jsonrpc": "2.0",
  "result": {
    "block_number": 401,
    "call": {
      "method": 0,
      "module": 9,
      "params": {
        "hash": "0xc34449ad91dfa044c4d314b1b22762189bb3ad4a8577a9050e90e443f3550afc",
        "number": 394,
        "parent": "0xf78eb90a94e881b488b8c3a81905d0424e2c55834a819164c7afba2594f43318",
        "relay_type": "Balance",
        "tx": "0x390281ff36b116bcdeff6bf63539cea3cafdd90bb53d6df043b2ef791d234c92ca5de80480ba2ca34dfe11d120a8c610534887312c79e5c247da9b4f31ea7495a4376f6a9512d3f0b771c923142c46dc33ef6f924f86b8f7bcd1749eb2e15aa388bddb090495000400ffc49bc1483a1669d65b19274445cb86604b7eca1d8e8d062269c8c6796a45b6250700e40b5402"
      }
    },
    "hash": "0x13a51a1eb9be4b8349493fb795b63045cfa6f27470a0dbcc8d28d63995179e67",
    "index": 5,
    "signature": null,
    "success": true
  },
  "id": 1
}

```


## state_getNonce

Get the nonce of the address

### Parameters
 - `address`
 - `block_number`: Optional
 
```asm
params: [
    "yee1zqe7q4mgy2n2sdhkz2sexqmqggzsu2rd53tp7hx9mrh9vrrym32qzlkq6f",
    121,
]
```

### Returns 
`nonce`


### Example
```
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"state_getNonce","params":["yee1zqe7q4mgy2n2sdhkz2sexqmqggzsu2rd53tp7hx9mrh9vrrym32qzlkq6f", 121],"id":1}' localhost:10055 -H 'Content-Type: application/json'

// Result
{
  "jsonrpc": "2.0",
  "result": 1,
  "id": 1
}

```

## author_submitExtrinsic

Submit extrinsic

### Parameters
 - `raw`
 
```asm
params: [
    "0x310281ff1033e0576822a6a836f612a193036042050e286da4561f5cc5d8ee560c64dc5440b2595b4c269c29377b658abe62303d59f975d0914f205d5fe8c7f24974a56007b979bf73e76211c3c3826293def93b882ef03a0a48e479693c2cbcb0425c0704b5030400ff94d988b42d96dcbd6605ff47f19c6ab35f626eb1bc8bbd28f59a74997a253a3d0284d717",
]
```

### Returns 
`hash`


### Example
```
// Request
curl -X POST --data '{"jsonrpc":"2.0","method":"author_submitExtrinsic","params":["0x310281ff1033e0576822a6a836f612a193036042050e286da4561f5cc5d8ee560c64dc5440b2595b4c269c29377b658abe62303d59f975d0914f205d5fe8c7f24974a56007b979bf73e76211c3c3826293def93b882ef03a0a48e479693c2cbcb0425c0704b5030400ff94d988b42d96dcbd6605ff47f19c6ab35f626eb1bc8bbd28f59a74997a253a3d0284d717"],"id":1}' localhost:10055 -H 'Content-Type: application/json'

// Result
{
  "jsonrpc": "2.0",
  "result": "0x4298dee6d0f9a84b28b14a42eada3d8f1e912efce359798f85c17f09f1cdcd79",
  "id": 1
}

```

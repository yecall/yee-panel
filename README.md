# Yee panel

Yee panel is a friendly RPC middleware

## Build

1. Install rust

1. Build
```
cargo build --release
```

## Deploy

1. Start yeeroot nodes of all the shard

1. Prepare yee panel base path

<base_path>/conf/config.toml: 
```
[shards]
[shards.0]
rpc = ["http://127.0.0.1:9033"]

[shards.1]
rpc = ["http://127.0.0.1:9133"]

[shards.2]
rpc = ["http://127.0.0.1:9233"]

[shards.3]
rpc = ["http://127.0.0.1:9333"]

```

1. Start yee-panel
```
./yee-panel --base-path=<base_path>
```

## Document

[RPC document](./docs/RPC.md)

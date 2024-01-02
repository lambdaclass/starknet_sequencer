# Starknet Sequencer

### How to run

```bash
cargo run
```

Will start the RPC server on port `1234`

You can then send requests to the socket accordingly:

```bash

$ curl -X POST localhost:1234 -H "Content-Type: application/json" --data '{"jsonrpc":"2.0", "method":"starknet_specVersion", "params":[], "id":1}'

{"jsonrpc":"2.0","result":"0.6.0","id":1}%
```
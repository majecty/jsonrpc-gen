# jsonrpc-gen
Generate jsonrpc's format with CLI

## Install

Use `cargo install`

## How to use

Type in the shell like `jsonrpc-gen <method-name> <arg>`
Then `{"jsonrpc": "2.0", "id":"1", "method": "<method-name>", "params": <arg> }` will be printed

```sh
$ jsonrpc-gen hi '[1,3]'
{"jsonrpc": "2.0", "id":"1", "method": "hi", "params": [1,3] }
```

# Substrate-Node-CLI
A simple rust cli to to read chain state and submit transactions to frameless substrate node.

## Usage

### To build:
```bash
cargo build
```
### NodeInfo:

This command covers all rpc call that not requires arguments. Pass the name of the method to select them. for example:
```bash
./target/debug/node-cli node-info --method rpc_methods

./target/debug/node-cli node-info --method system_health

./target/debug/node-cli node-info --method system_chainType

./target/debug/node-cli node-info --method system_chain

./target/debug/node-cli node-info --method system_peers
```

### Setting and Reading Value

This command sets and reads an u128 number as a way of testing. The storage key is hardcoded. To be able to call the SetValue(u128) call. For example:
```bash
./target/debug/node-cli setting-value --number 10

./target/debug/node-cli reading-value
```

### Read State Balance

Command to ask for the balance or u128 number stored in a text key the user provide. This command can be used for debugging purposes. So to read balances after mining or transfering. Is necessary to know the key in text format.

```bash
./target/debug/node-cli read-state-balance --text-key VALUE_KEY
```

### Mint

In development. Currently it has the public key of a user hardcoded and mints tokens to that account.

```bash
./target/debug/node-cli mint
```

## COMMENTS ABOUT THE ASSIGMENT

I spent half of the time of the assigment working on the CLI. Encoding and decoding; and getting UI and Node agree are not trivial tasks.

Making the CLI recieve decimal numbers and text is the main part of the work develped until now.

I would like the Teachers to check encodind and decoding part in specific.

I think that this CLI is simple enough to make visible the important part of the interaction between user client and the RPC node. So, with some improvement can be used as pedagogical tool.

## Futher development

- Local storage of the pair keys after login of the user.
- Usage of accountid instead of raw public key pairs.

## References

* [rust-cli-template](https://github.com/noahgift/rust-docker-cli)

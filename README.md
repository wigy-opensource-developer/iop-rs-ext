# Internet-of-People Rust SDK Extensions

## Delegates

A simple tool listing all balances and nonces of genesis delegates on the IOP
testnet. 10 queries run in parallel, output is not ordered.

```bash
$ cargo run --bin delegates
#04: balance: 118578139198000, nonce: 1
#06: balance: 118579926853000, nonce: 1
...
```

## Transfer

An example how to send in a transfer transaction on the testnet.

```bash
$ cargo run --bin transfer
txid: cc15d51cece60cff503dfdc3741b889ed9509e25946450a9ef14d5edad9778d9
```

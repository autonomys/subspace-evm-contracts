# CLI commands

Listed down are some commonly used commands for the CLI other than build, deploy, test in this case.

> - Mostly chain oriented rather than smart contracts.
> - Before running any command, ensure the deployer public key and RPC URL are set as environment variables using `$ source .env`.

## Get balance

```sh
cast balance $DEPLOYER_PUBLIC_KEY --rpc-url $NOVA_RPC_URL
```

## Get nonce

```sh
cast nonce $DEPLOYER_PUBLIC_KEY --rpc-url $NOVA_RPC_URL
```

## Send transaction

Calling setter function of deployed Load contract:

```sh
cast send $LOAD "setArray(uint256)" 1200 --private-key $DEPLOYER_PRIVATE_KEY --rpc-url $NOVA_RPC_URL
```

## Get contract state

```sh
$ cast call $COUNTER "number()" --rpc-url $NOVA_RPC_URL
0x00000000000000000000000000000000000000000000000000000000000038eb
```

> This value is in hex, convert it to decimal to get the actual value.

```sh
$ cast --to-dec 38EB -i 16
14571
```

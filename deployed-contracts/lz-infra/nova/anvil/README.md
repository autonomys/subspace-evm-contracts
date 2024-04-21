Follow the instructions to setup for your Anvil node:

In Terminal-1:

```sh
anvil --fork-url $NOVA_RPC_URL
```

---

In Terminal-2:

1. deploy LZ Infra suite by running:

```sh
forge script ./script/LzInfra.s.sol:LzInfraScript --private-key ${DEPLOYER_PRIVATE_KEY} --rpc-url http://127.0.0.1:8545 --broadcast --legacy
```

2. Run Autobridge script for sending tokens from Nova:

```sh
forge script ./script/auto_bridge.s.sol:AutoBridgeScript --private-key $DEPLOYER_PRIVATE_KEY --rpc-url http://127.0.0.1:8545 --broadcast --legacy -vvvv
```

View a sample [log](./autobridge-run-log.sh).

3. Start the AutoBridge DVN script on another terminal:

For debug:

```sh
forge script ./script/autobridge_dvn.s.sol:AutoBridgeDVNScript --private-key $DEPLOYER_PRIVATE_KEY --rpc-url $SEPOLIA_RPC_URL --broadcast --legacy -vvvv
```

For production:

> Go to layerzero-demo/demos.

```sh
bun auto-bridge:dvn
```

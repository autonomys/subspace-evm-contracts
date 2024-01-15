# Subspace EVM Contracts

This repository contains the relevant EVM Contracts that are deployed on Subspace Nova. This could perhaps be considered as a one-stop shop for all the relevant contracts, libraries deployed on Subspace Nova.

> Currently, the contracts are deployed on the `gemini-3g` Nova chain with chain ID `1002`.

## Usage

### Smart Contracts

#### Build

```sh
forge build
```

#### Test

```sh
forge test
```

#### Format

```sh
forge fmt
```

#### Gas Snapshots

```sh
forge snapshot
```

> - `1 TSSC = 1e9 Gwei`
> - `Block gas limit = 60,000,000`

Example of converting gas usage to TSSC:

```sh
gas price = 3.00000002 Gwei
So, 27,177,138 gas would cost:
27,177,138 * 3.00000002 Gwei = 81,531,414.54354276 Gwei * 10^-9 = 0.08153141454354276 TSSC
```

#### Deploy

```sh
forge script script/Counter.s.sol:CounterScript --rpc-url <your_rpc_url> --private-key <your_private_key>
```

### Generating Rust bindings to the contracts

> Before following this, make sure you have the `forge` CLI installed.

Rust bindings to the contracts can be generated via `forge bind`, which requires
first building your contracts:

```sh
# Build the contracts
$ forge build --root ./bindings
# Generate the bindings to the contracts
$ forge bind --bindings-path ./bindings --root ./ --crate-name subspace-evm-bindings
```

Any follow-on calls to `forge bind` will check that the generated bindings match
the ones under the build files. If you want to re-generate your bindings, pass
the `--overwrite` flag to your `forge bind` command.

### Nova

View the usage [here](./cli/README.md).

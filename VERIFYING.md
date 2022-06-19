# Verifying the Monolith Prop Smart Contract

The source code for the Monolith prop smart contract is available in the [cw-monolith-prop repo](https://github.com/RAMPAGEXP/cw-monolith-prop).

To verify the code deployed on chain you will need `docker` and `rxpd` installed on your machine.

### Compile contracts locally

Clone the repo and enter the directory:
```bash
git clone https://github.com/RAMPAGEXP/cw-monolith-prop
cd cw-monolith-prop
git checkout v0.0.1-alpha
```

Compile the contracts:
```bash
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/rust-optimizer:0.12.6
```

This should output info about the contract sha256 hash:
```bash
Optimizing cw_monolith_prop.wasm ...
Creating hashes ...
3481e613b412706204124ce081ac9cba18eb044d5c78c922e55e162e411d4b47  cw_monolith_prop.wasm
Info: sccache stats after build
```

#### More info on verifying smart contracts
- https://docs.cosmwasm.com/docs/1.0/smart-contracts/verify
- https://secdao.medium.com/smart-contract-verification-75f9a7e7f23

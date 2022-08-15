## Description

| Chain       | Contract                                                        |
| ----------- | --------------------------------------------------------------- |
| malaga-420  | wasm13srsm7zrvnyf77atxq8vvtj4tm8nuqfqlfut7me4fml00zxjqd3qmz3dnt |
| osmo-test-4 | osmo1arvynavhgcn7ssrxgtymxnvs2djkawlutw4zmercyhjw8jdd40wq76hl7n |

## Deployment

### Osmoisis chain:

1. Setup

```
cosmwast-std version: 1.0.0
osmosisd version: 11.0.0
```

2. Deploy a smart contract:

Add environment

```sh
OSM_RPC="https://rpc-test.osmosis.zone:443"
OSM_CHAIN_ID="osmo-test-4"
OSM_NODE=(--node $OSM_RPC)
```

Create new wallet

```sh
osmosisd keys add osmosisd_wallet
```

It shows an address - our wallet and `osmosisd_wallet` is an alias of the wallet.
We need claim `uosmo` free in [testnet](https://faucet.osmosis.zone/#/)

After claim we have a lot of tokens :)), store code and instantiate in the chain:

```sh
cd ver_1.0.0/cw_template/artifacts

RES=$(osmosisd tx wasm store cw_template.wasm $OSM_NODE --chain-id $OSM_CHAIN_ID --from osmosisd_wallet --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 -y --output json -b block)

CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[0].value')

osmosisd tx wasm instantiate $CODE_ID '{}' $OSM_NODE --chain-id $OSM_CHAIN_ID --from osmosisd_wallet --label "cw_counter" --gas-prices 0.025uosmo --gas auto --gas-adjustment 1.3 -b block -y --no-admin

```

### Malaga chain:

# substrate-overlay-token

A new SRML-based Substrate node, ready for hacking.


## initialize

```
$ ./scripts/build.sh
$ cargo build --release
$ ./target/release/substrate-overlay-token --dev
```

### start as parent

```
$ ./target/release/substrate-overlay-token purge-chain --chain parent --base-path /tmp/parent -y
$ ./target/release/substrate-overlay-token --chain parent --charlie --validator --base-path /tmp/parent --ws-port 9944
```

### start as child

```
$ ./target/release/substrate-overlay-token purge-chain --chain child --base-path /tmp/child -y
$ ./target/release/substrate-overlay-token --chain child --dave --validator --base-path /tmp/child --ws-port 9945
```

## token details

states
- init: bool
- root: bool
- owner: AccountId
- name: Bytes
- ticker: Bytes
- totalSupply: TokenBalance
- localSupply: TokenBalance
- balanceOf(AccountId): TokenBalance
- parentSupply: TokenBalance
- childSupplies(ChildChainId): TokenBalance

functions
- init()
- set_parent(parent_supply)
- transfer(receiver, value)
- mint(value)
- burn(value)
- sendToParent(value)
- sendToChild(child, value)
- receiveFromParent(value)
- receiveFromChild(child, value)

additional type definitions

```
{
  "TokenBalance": "u128",
  "ChildChainId": "u32"
}
```

## Using docker-compose

```
$ cp command.sh ../
$ cd ../
$ git clone substrate-overlay-token-ui
$ git clone substrate-overlay-token-bridge
```

You can run

```
$ sh command.sh init {YOUR_DOMAIN}
$ sh command.sh run
$ sh command.sh stop
```

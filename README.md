# substrate-overlay-token

A new SRML-based Substrate node, ready for hacking.

## document

https://github.com/dmm-bclabs/overlay-token

## requirements

You should install substrate on your local environment as follow.

https://github.com/paritytech/substrate#on-mac-and-ubuntu

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
$ git clone https://github.com/dmm-bclabs/substrate-overlay-token-ui.git
$ git clone https://github.com/dmm-bclabs/substrate-overlay-token-bridge.git
```

You can run

```
$ sh command.sh init {YOUR_DOMAIN}
$ sh command.sh build
$ sh command.sh run
$ sh command.sh stop
```

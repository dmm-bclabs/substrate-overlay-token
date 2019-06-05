# substrate-overlay-token

A new SRML-based Substrate node, ready for hacking.


## initialize

```
$ ./scripts/build.sh
$ cargo build --release
$ ./target/release/substrate-overlay-token --dev
```

## token details

states
- init: bool
- owner: AccountId
- name: Bytes
- ticker: Bytes
- totalSupply: TokenBalance
- localSupply: TokenBalance
- balanceOf(AccountId): TokenBalance
- parentSupply: TokenBalance
- childSupplies(ChildChainId): TokenBalance

functions
- init(total_supply)
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

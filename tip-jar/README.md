# TipJar Stylus Contract

## Environment Variables

```bash
export PRIVATE_KEY=your_private_key # for testing: 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520659
export USER_1=0x00A2895816e64F152FF81c8A931DC1bd9F5c3ce3 # User 1 Address
```

## Deploying Contract

```bash
cargo stylus deploy --endpoint='http://localhost:8547' --private-key="$PRIVATE_KEY" --no-verify
```

> Update ENV with Contract Address: `export CONTRACT_ADDRESS=contract_address_after_deploy`

## Get Balance

```bash
cast call --rpc-url 'http://localhost:8547' --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "getBalance(address)(uint256)" $USER_1

# 0
```

## Tip User

```bash
cast send --rpc-url 'http://localhost:8547' --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "tip(address,uint256)(uint256)" $USER_1 1000000000 --value 1000000000
```

## Get Balance

```bash
cast call --rpc-url 'http://localhost:8547' --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "getBalance(address)(uint256)" $USER_1

# 1000000000
```

## Get Ether Balance

```bash
cast balance --rpc-url 'http://localhost:8547' $CONTRACT_ADDRESS
# 1000000000
cast balance --rpc-url 'http://localhost:8547' $USER_1
# 0
```

## Withdraw Tip

```bash
cast send --rpc-url 'http://localhost:8547' --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "withdraw(address)(uint256)" $USER_1
```

## Get Balance

```bash
cast call --rpc-url 'http://localhost:8547' --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "getBalance(address)(uint256)" $USER_1

# 0
```

## Get Ether Balance after Withdraw

```bash
cast balance --rpc-url 'http://localhost:8547' $CONTRACT_ADDRESS
# 0
cast balance --rpc-url 'http://localhost:8547' $USER_1
# 1000000000
```

---

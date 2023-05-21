# Anchor TODO App

## Local Run

### Run Solana Local Validator

```sh
solana-test-validator
```

### Run Client

```sh
cargo run -p app
```

## Cost Of Deployment

```sh
stat -f "%z bytes" ./target/deploy/todo.so
# 208K -> 213464 bytes
```
rent = `2n + 45` = 2 * 213464 + 45 = 426973

```sh
solana rent 426973
# Rent-exempt minimum: 2.97262296 SOL
```

### Testing

```sh
anchor test --skip-local-validator
```
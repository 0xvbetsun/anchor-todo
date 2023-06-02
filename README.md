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
anchor test --skip-local-validator --skip-lint --skip-build --skip-deploy
```

## Troubleshooting

```
==================================================================================
Recover the intermediate account's ephemeral keypair file with
`solana-keygen recover` and the following 12-word seed phrase:
==================================================================================
valley flat great hockey share token excess clever benefit traffic avocado athlete
==================================================================================
To resume a deploy, pass the recovered keypair as
the [BUFFER_SIGNER] to `solana program deploy` or `solana program write-buffer'.
Or to recover the account's lamports, pass it as the
[BUFFER_ACCOUNT_ADDRESS] argument to `solana program drain`.
==================================================================================
```
[for more info](https://docs.solana.com/cli/deploy-a-program)

## Resources

- [Solana RPC Doc](https://docs.solana.com/ru/api/http)
- [Conventional Commits Emoji](https://gist.github.com/parmentf/359667bf23e08a1bd8241fbf47ecdef0)
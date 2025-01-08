## anchor-template

1. First, install the necessary dependencies:

```
yarn install

# Install Anchor CLI if you haven't already, currently only compatible with 0.29.0
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install 0.29.0
avm use 0.29.0
```

2. Generate a program ID:

```
solana-keygen new -o program-id.json
```

3. Update the program ID in your files:

+ lib.rs
+ Anchor.toml
+ scripts/client.ts

4. Build the program:

```
anchor build
```

5. Deploy the program:

For localnet:
```
# Start local validator if not running
solana-test-validator

# Deploy to localnet
anchor deploy
```

For devnet:
```
# Switch to devnet
solana config set --url devnet

# Airdrop some SOL for deployment
solana airdrop 2

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

For mainnet:
```
# Switch to mainnet
solana config set --url mainnet-beta

# Deploy to mainnet
anchor deploy --provider.cluster mainnet-beta
```

6. Verify the deployment:

```
solana program show --programs
```

## trouble shooting

1. If you get BPF compilation errors:

```
cargo clean
anchor clean
anchor build
```

2. If you get "Program already exists" error:

```
# anchor deploy --program-id program-id.json
anchor deploy --program-name xxx --program-keypair program-id.json
```

3. If you need to upgrade the program:

```
anchor upgrade target/deploy/xxx.so --program-id <PROGRAM_ID>
```

4. To verify program logs:

```
solana logs <PROGRAM_ID>
```

5. To run tests:

```
# anchor test --skip-deploy # skip deploy if on devnet/mainnet
anchor test
```

Remember to:

- Keep your program ID keypair safe
- Test thoroughly on devnet before mainnet deployment
- Ensure you have enough SOL for deployment
- Verify program authority and upgrade authority after deployment

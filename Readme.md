# Using Kurtosis on Ubuntu:

https://docs.kurtosistech.com/


1. Instqalling apt package
```
echo "deb [trusted=yes] https://apt.fury.io/kurtosis-tech/ /" | sudo tee /etc/apt/sources.list.d/kurtosis.list
sudo apt update
sudo apt install kurtosis-cli
```

2. Configure Kurtosis
You’ll therefore want the first call to the kurtosis CLI in your CI job to be either:
`kurtosis config init send-metrics`

If you’d like to help us make the product better for you or
`kurtosis config init dont-send-metrics`

# Controlling Kurtosis enclaves:
`kurtosis enclave ls`
`kurtosis enclave inspect [enclave-name]`
`kurtosis enclave stop [enclave-name]`
`kurtosis clean`

# Running curtosis module in docker:
`kurtosis module exec kurtosistech/near-kurtosis-module:0.5.25`

# Near local Alias

After kurtosis is started - configure a BASH terminal with environment variables and a local_near
`source ./config-terminal.sh`
Adjust environment variables inside file as required by kurtosis startup.

**IMPORTANT!** Make sure to adjust `near` commands inside `package.json` project files to use `locnear` command instead.

# Accounts

## Top-level payment accounts need to be created using the wallet.
For example to create a user account we have to create a new account using wallet.
Then we can extract private key using Chrome Dev tools. And pool the public key using `near keys` command.

## Account Setup

### Getting Public Key from Private Key
```
locnear repl
nearAPI.KeyPair.fromString("ed25519:4SSMoxgetTffkgCgoQ9Hwx1o9cVk4eAVANUURPXPhtJZzmwycgQGMo3HrSX4wkmoZ2ymF8u5pgdv9JsTzyBgFzM4").publicKey.toString()
-- Returns Public Key
.exit
```
### Getting account keypair from Wallet into Local Env
1. Register account inside wallet
2. Once finished - select account and open Chomw DevTools. Go to Application section.
3. Open LocalStorage area and copy the PrivateKey of an appropriate Wallet address.
4. Use Getting Public Key from Private key instructions above to extract Public Key from Private Key.
5. Verify public key using `locnear keys <accountname.test.near>`
6. Create a json file in: `$HOME/.near-credentials/localnet/` name it `accountname.test.near.json`
7. Set proper permissions using `chown 600 *.json`

# Smart Contract Deployment

## Creating Sub-Accounts
It is possible to create sub accounts once we set up a main account.
`locnear create-account cont1.a.test.near --masterAccount a.test.near --initialBalance 5`
Unfortunately it seems that assigning a custom key right from the start is impossible.
This means that we can later create another key and assign it to the new account as needed.
```
locnear generate-key ...
locnear add-key ...
```
However importing them after into the wallet is still impossible.


## Building Contract without NPM Steps:

1. `cd contract`
2. `rustup target add wasm32-unknown-unknown`
3. `cargo build --all --target wasm32-unknown-unknown --release`

## Deploy contract
1. `cd contract`
2. `locnear delete hello.bob.test.near bob.test.near`
3. `locnear create-account hello.bob.test.near --masterAccount a.test.near --initialBalance 20`
4. `locnear deploy --wasm-file ./target/wasm32-unknown-unknown/release/hello_near.wasm --accountId hello.a.test.near`

# Calling Smart contract with CLI
`locnear call <contractName> <methodName> [args]`
Foe example to call contract with Arguments:
`locnear call hello.a.test.near set_greeting '{"message":"goaway"}' --accountId  b.test.near`
or
`locnear call hello.a.test.near set_greeting '{}' --accountId  b.test.near`


# Importing account with Private Key fails.
http://127.0.0.1:8334/auto-import-secret-key#YOUR_ACCOUNT_ID/YOUR_PRIVATE_KEY



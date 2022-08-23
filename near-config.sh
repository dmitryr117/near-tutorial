#!/bin/bash

# prepend execution with `source` eg:
# source ./config-terminal.sh

echo -n "Did you switch to Node environment and prepend source? y/n: "
read nodeon
if [ "$nodeon" != "y" ]
then
  echo "Set Node env. cat ~/envs.md"
  exit
fi
echo -n "What is .neartosis private key directory?: "
read neartosis_dir
echo $neartosis_dir

export NEAR_ENV="local"
export NEAR_CLI_LOCALNET_NETWORK_ID="localnet"
export NEAR_NODE_URL="http://127.0.0.1:8332"
export NEAR_CLI_LOCALNET_KEY_PATH="${HOME}/.neartosis/${neartosis_dir}/validator-key.json"
export NEAR_WALLET_URL="http://127.0.0.1:8334"
export NEAR_HELPER_URL="http://127.0.0.1:8330"
export NEAR_HELPER_ACCOUNT="test.near"
export NEAR_EXPLORER_URL="http://127.0.0.1:8331"

#lias locnear="NEAR_ENV=\"${NEAR_ENV}\" NEAR_CLI_LOCALNET_NETWORK_ID=\"${NEAR_CLI_LOCALNET_NETWORK_ID}\" NEAR_NODE_URL=\"${NEAR_NODE_URL}\" NEAR_CLI_LOCALNET_KEY_PATH=\"${NEAR_CLI_LOCALNET_KEY_PATH}\" NEAR_WALLET_URL=\"${NEAR_WALLET_URL}\" NEAR_HELPER_URL=\"${NEAR_HELPER_URL}\" NEAR_HELPER_ACCOUNT=\"${NEAR_HELPER_ACCOUNT}\" NEAR_EXPLORER_URL=\"${NEAR_EXPLORER_URL}\" near"

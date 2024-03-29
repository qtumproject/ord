# Overview
This document provides a brief overview of the ordinals wallet for Qtum (`qord`), and how to setup the development environment for it.

## What is `qord`?
`qord` is a wallet for Qtum that is designed to be used to inscribe sat in the Qtum blockchain. It is a fork of the bitcoin's [ordinals wallet](https://github.com/ordinals/ord) that has been modified to work with Qtum.

# Quick start guide

## Building `qord`


Clone the qord repository
  
  ```bash
git clone https://github.com/qtumproject/ord.git
```

`qord` can be built using the following command:

```bash
cd ord
cargo build --release
```

## Ordinals on a regtest qtum node

### 1. Running a qtum node on regtest

`qord` requires a qtum node to be running on regtest. The easiest way to do this is with docker:

```bash
docker run --name qtum_regtest -d -p 13777:13777 qtum/qtum:latest qtumd -regtest -txindex -addrindex=1 -rpcbind=0.0.0.0:13777 -rpcallowip=0.0.0.0/0 -logevents -rpcuser=qtum -rpcpassword=qtum -deprecatedrpc=accounts -printtoconsole -debug=1
```

### 2. Alias qord
It is useful to alias `qord` so it runs with the correct arguments:

```bash
alias qord='RUST_LOG=debug <path to cloned repo>/ord/target/release/qord --regtest --bitcoin-rpc-user qtum --bitcoin-rpc-pass qtum  --rpc-url 127.0.0.1:13777'
```

### 3. Creating a `qord` wallet

```bash
qord wallet create
```

You should see the following output:

```bash
[2023-10-08T21:06:32Z INFO  ord::options] Connecting to Bitcoin Core at 127.0.0.1:13777/wallet/ord
[2023-10-08T21:06:32Z DEBUG bitcoincore_rpc] JSON-RPC request: getblockchaininfo []
[2023-10-08T21:06:32Z DEBUG bitcoincore_rpc] JSON-RPC request: getnetworkinfo []
[2023-10-08T21:06:32Z DEBUG bitcoincore_rpc] JSON-RPC request: getnetworkinfo []
[2023-10-08T21:06:32Z DEBUG bitcoincore_rpc] JSON-RPC request: createwallet ["ord",false,true]
[2023-10-08T21:06:32Z DEBUG bitcoincore_rpc] JSON-RPC request: importdescriptors [[{"active":true,"desc":"tr([24735198/86'/1'/0']tprv8gXojALAAYg9ojGa9AJwbwsojkg4tTuR6cKcACY4BSmVnw8mQrJJiLXQCSazmr71asiQLXGnRopLGRpUiozfdXeeakwwafXp1te2av1B8Zk/0/*)#hcxp0gyd","internal":false,"timestamp":"now"}]]
[2023-10-08T21:06:32Z DEBUG bitcoincore_rpc] JSON-RPC request: importdescriptors [[{"active":true,"desc":"tr([24735198/86'/1'/0']tprv8gXojALAAYg9ojGa9AJwbwsojkg4tTuR6cKcACY4BSmVnw8mQrJJiLXQCSazmr71asiQLXGnRopLGRpUiozfdXeeakwwafXp1te2av1B8Zk/1/*)#xvrqja54","internal":true,"timestamp":"now"}]]
{
  "mnemonic": "public ball lizard list shock galaxy all letter help shoulder opinion immune",
  "passphrase": ""
}
```

### 4. Create an address for the `qord` wallet

```bash
❯ qord wallet receive
[2023-10-08T21:07:03Z INFO  ord::options] Connecting to Bitcoin Core at 127.0.0.1:13777/wallet/ord
[2023-10-08T21:07:03Z DEBUG bitcoincore_rpc] JSON-RPC request: getblockchaininfo []
[2023-10-08T21:07:03Z DEBUG bitcoincore_rpc] JSON-RPC request: getnetworkinfo []
[2023-10-08T21:07:03Z DEBUG bitcoincore_rpc] JSON-RPC request: getnetworkinfo []
[2023-10-08T21:07:03Z DEBUG bitcoincore_rpc] JSON-RPC request: listwallets []
[2023-10-08T21:07:03Z DEBUG bitcoincore_rpc] JSON-RPC request: listdescriptors []
[2023-10-08T21:07:03Z DEBUG bitcoincore_rpc] JSON-RPC request: getnewaddress [null,"bech32m"]
{
  "address": "qcrt1pn5gu79mlxaswessrhw3n5ldd70x0g6r2z0z4l5eqaxdgw9gvfn5s97fevr"
}
```

### 5. Use the qtum cli to fund the qord wallet address

Open another terminal to use with `regtest` and perform the following steps:

```bash
# login into the qtum regtest container
docker exec -it qtum_regtest bash

# alias the qtum cli
root@875eeb0757a8:/# alias q='/usr/local/bin/qtum-cli  -rpcuser=qtum -rpcpassword=qtum -regtest -rpcport=13777 --rpcconnect=127.0.0.1'

# generate 2001 blocks to fund the qord wallet
root@875eeb0757a8:/#  q generatetoaddress 2001  qcrt1pn5gu79mlxaswessrhw3n5ldd70x0g6r2z0z4l5eqaxdgw9gvfn5s97fevr
```

### 6. Create a file to inscribe into a satoshi



```bash
echo "Hello Qtum ordinal!" > /tmp/hello.txt
```

### 7. Inscribing the file into a satoshi

```bash
### generate the transactions to inscribe the file into a satoshi
qord wallet inscribe --fee-rate 61601 --file /tmp/hello.txt 
```
Back on the `regtest` terminal, use qtum-cli again to mine the transactions:

```bash
root@875eeb0757a8:/#  q generatetoaddress 1  qcrt1pn5gu79mlxaswessrhw3n5ldd70x0g6r2z0z4l5eqaxdgw9gvfn5s97fevr
```

### 8. Visualize the inscribed satoshi using the `qord` explorer

```bash
qord server
```

### 9. Open the browser to visualize the inscribed satoshi

Navigate to http://localhost:8000/ to see the inscribed satoshi on block 2002:

![Alt text](image.png)

## Ordinals on qtum testnet

### 1. Running a qtum node on testnet

`qord` requires a qtum node to be running on testnet. The easiest way to do this is with docker:

```bash
docker run --name qtum_testnet -d -p 3889:3889 qtum/qtum:latest qtumd -testnet -txindex -addrindex=1 -rpcbind=0.0.0.0:3889 -rpcallowip=0.0.0.0/0 -logevents -rpcuser=qtum -rpcpassword=qtum -deprecatedrpc=accounts -printtoconsole
```

Wait for the node to sync with the testnet blockchain.

### 2. Alias qord
It is useful to alias `qord` so it runs with the correct arguments:

```bash
alias qord='RUST_LOG=debug  <path to cloned repo>/ord/target/release/qord --testnet --bitcoin-rpc-user qtum --bitcoin-rpc-pass qtum  --rpc-url 127.0.0.1:3889'
```

### 3. Creating a `qord` wallet

```bash
❯ qord wallet create
{
  "mnemonic": "unknown document skull person few fat exotic salute belt drive ritual beef",
  "passphrase": ""
}
```

### 4. Create an address for the `qord` wallet

```bash
❯ qord wallet receive
{
  "address": "tq1pka5cz4hmz4elp279hma55eclk396thvp7jnp5yhym0v20t65rymsrd9lmh"
} 
  ```
### 5. Fund the `qord` wallet address

In order to make an inscription, you need to fund the `qord` wallet address. You can do this by sending testnet QTUM coins to the address generated in the previous step.

### 6. Create a file to inscribe into a satoshi

```bash
echo "Hello Qtum testnet ordinal!" > /tmp/hello.txt
```

### 7. Inscribing the file into a satoshi

```bash
qord wallet inscribe --fee-rate 61601  --file /tmp/hello.txt
```


## Restarting the `qord` server

If you follow the steps above to make an inscription, you might get errors related to missing transaction outputs or indexing errors.

The quickest way to fix this is to delete the database and reindex the blockchain.

By default, the database is stored in the following locations depending on your operating system:

|Platform | Value                                            | Example                                      |
| ------- | ------------------------------------------------ | -------------------------------------------- |
| Linux   | `$XDG_DATA_HOME`/ord or `$HOME`/.local/share/ord | /home/alice/.local/share/ord                 |
| macOS   | `$HOME`/Library/Application Support/ord          | /Users/Alice/Library/Application Support/ord |
| Windows | `{FOLDERID_RoamingAppData}`\ord                  | C:\Users\Alice\AppData\Roaming\ord           |

So to delete the database and reindex on MacOS you would have to run the following
commands in the terminal:

```bash
rm ~/Library/Application Support/ord/index.redb
ord index run
```


# Hello, World! Solana Smart Contract

This project demonstrates a basic smart contract written in Rust with Anchor. It comes with a smart contract and a test for the contract.

---

## Setup the Project

Before you can run the project test cases you will need to install dependencies and setup your keypair.

### Install Rust
First make sure you have the Rust programming language installed, you can follow the installation guide [here](https://doc.rust-lang.org/book/ch01-01-installation.html?utm_source=buildspace.so&utm_medium=buildspace_project): 

Verify that Rust and Cargo were installed correctly by running the following commands.
```shell
rustup --version
rustc --version
cargo --version
```

---

### Install Solana and Anchor
Follow the installation steps [here](https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool?utm_source=buildspace.so&utm_medium=buildspace_project) to install the Solana CLI. This is a necessary step to test contracts locally.

Verify that solana was installed correctly:
```shell
solana --version
```

Run some config commands:
```shell
solana config set --url localhost
solana config get
```

Install Anchor, a popular development framework for Solana. To learn more about Anchor, check out [The Anchor Book](https://book.anchor-lang.com/).

```shell
cargo install --git https://github.com/project-serum/anchor anchor-cli --locked
```
Verify the installation
```shell
anchor --version
```

Install the remaining npm dependencies
```shell
npm install
```

---

### Setup a keypair to communicate with the contract
Generate a new keypair
```shell
solana-keygen new -o target/deploy/hello-world-solana-keypair.json
```
Verify that the wallet property in `Anchor.toml` is pointing to the keypair we just created
```shell
wallet = "target/deploy/hello-world-solana-keypair.json"
```
---

## Some Common Commands

### List Anchor Commands
```shell
anchor
```

### Build Contracts
```shell
anchor build
```

### Run test cases
```shell
anchor test
```

### Deploy the smart contract
```shell
anchor deploy <project_path>/target/deploy/hello_world_solana.so
```
When deploying, you can override the project address by passing `--program-id`

---

## Next Steps

### Add additional functionality to the contract
 - Let users like/dislike messages
 - Emit an event when a message is liked/disliked
 - Let users reply to messages

### Setup a frontend for users to interact with your contract
 - Explore options like `@solana/web3.js` to make frontend web3 development a breeze
    
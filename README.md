# DeployDAO Migrator

[![Crates.io](https://img.shields.io/crates/v/migrator)](https://crates.io/crates/migrator)
[![License](https://img.shields.io/crates/l/migrator)](https://github.com/DeployDAO/migrator/blob/master/LICENSE.txt)
[![Build Status](https://img.shields.io/github/workflow/status/DeployDAO/migrator/Rust/master)](https://github.com/DeployDAO/migrator/actions/workflows/rust.yml?query=branch%3Amaster)
[![Contributors](https://img.shields.io/github/contributors/DeployDAO/migrator)](https://github.com/DeployDAO/migrator/graphs/contributors)

**WARNING: This code is a work in progress. Please do not use it as is.**

A program for deploying and upgrading programs.

## About

The Migrator:

- Performs program deploys and upgrades
- Decouples program deploys/upgrades into 3 roles: the proposer, the approver, and the deployer
- Leaves an on-chain audit trail of program upgrades and deploys

## Usage

There are two forms of intended usage: "self-hosted" and the DeployDAO.

### Self-hosted

This may be used for development or if you want to maintain full control over your own smart contract deployment.

1. Reserve a program ID. This allows for the program to be deployed at the same address across multiple chains.
2. Create a new migrator, with the approver set to your own address, a multisig, or a DAO.
3. Upload the bytecode of the program to a buffer via `solana program write-buffer`. Ideally this bytecode is generated in a [verifiable manner](https://anchor.projectserum.com/).
4. Create a proposal to deploy the program.
5. Approve your proposal.
6. Anyone must supply the migrator account with enough SOL to cover the program deployment.
7. Anyone may deploy the new migration, until the migration expires.

### DeployDAO

_note: this is subject to change_

The DeployDAO is a decentralized autonomous organization that elects multisig holders to approve program upgrades and deploys.

To deploy a program, one should:

1. Reserve a program ID. This allows for the program to be deployed at the same address across multiple chains.
2. Create a new migrator, with the approver set to the DeployDAO address.
3. Upload the bytecode of the program to a buffer via `solana program write-buffer`. Ideally this bytecode is generated in a [verifiable manner](https://anchor.projectserum.com/).
4. Create a proposal to deploy the program.
5. Contact the DeployDAO requesting for your program to be approved for deployment.
6. If the DeployDAO likes your code, they may approve the deployment.
7. Anyone in the community must supply the migrator account with enough SOL to cover the program deployment.
8. Anyone in the community may deploy the new migration, until the migration expires.

Upgrading is done very similarly.

## License

The DeployDAO program and SDK is distributed under the GPL v3.0 license.

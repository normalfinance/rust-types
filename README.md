# Normal Finance Rust Types

[![Crates.io](https://img.shields.io/crates/v/normal-rust-types.svg)](https://crates.io/crates/normal-rust-types)
[![Documentation](https://docs.rs/normal-rust-types/badge.svg)](https://docs.rs/normal-rust-types)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE.md)

A comprehensive Rust types library for the Normal Finance ecosystem, providing shared data structures, events, storage types, and error definitions for Soroban smart contracts.

## Overview

Normal Finance is a decentralized finance platform built on Stellar's Soroban smart contract platform. This crate provides the foundational types used across all Normal Finance smart contracts, including:

- **Index Fund Management**: Types for creating and managing tokenized index funds
- **Automated Market Maker (AMM)**: Pool management, liquidity provision, and trading types
- **Insurance Fund**: Risk management and insurance claim structures
- **Oracle Integration**: Price feed and oracle registry types
- **Access Control**: Permission and role management types

## Features

- **Soroban SDK Integration**: Built specifically for Soroban smart contracts using `soroban-sdk` v22.0.1
- **Comprehensive Error Handling**: Standardized error types across all contract interactions
- **Event Definitions**: Structured event types for contract logging and monitoring
- **Storage Types**: Optimized data structures for Soroban contract storage
- **Type Safety**: Strongly-typed interfaces preventing common smart contract vulnerabilities

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
normal-rust-types = "0.1.3"
```

## Usage

### Basic Import

```rust
use normal_rust_types::*;
```

### Specific Module Imports

```rust
// Import specific modules as needed
use normal_rust_types::{
    errors::{PoolError, TokenError},
    types::{Pool, SwapParams, IndexInfo},
    events::{SwapEvent, PoolCreatedEvent},
    storage::{PoolStorage, AccessControlStorage}
};
```

### Working with Pool Types

```rust
use normal_rust_types::{Pool, PoolTier, PoolStatus, SwapParams};
use soroban_sdk::{Env, Address, Symbol};

// Create a new pool configuration
let pool = Pool {
    token_b: token_address,
    base_asset: Symbol::new(&env, "USDC"),
    quote_asset: Symbol::new(&env, "XLM"),
    tier: PoolTier::Tier1,
    status: PoolStatus::Active,
    // ... other fields
};

// Define swap parameters
let swap_params = SwapParams {
    token_in: usdc_address,
    token_out: xlm_address,
    amount_in: 1000_0000000, // 1000 USDC (7 decimals)
    amount_out_min: 2500_0000000, // Minimum 2500 XLM expected
    to: user_address,
    // ... other fields
};
```

### Working with Index Types

```rust
use normal_rust_types::{IndexInfo, Component};

// Define index components
let components = vec![
    Component {
        asset: Symbol::new(&env, "BTC"),
        weight: 4000, // 40% allocation
    },
    Component {
        asset: Symbol::new(&env, "ETH"),
        weight: 3000, // 30% allocation
    },
    Component {
        asset: Symbol::new(&env, "XLM"),
        weight: 3000, // 30% allocation
    },
];

// Create index info
let index_info = IndexInfo {
    address: index_contract_address,
    token_address: index_token_address,
    total_shares: 0,
    base_nav: 1_0000000, // Starting NAV of 1.0
    is_public: true,
    components,
    // ... other fields
};
```

### Error Handling

```rust
use normal_rust_types::{PoolError, TokenError, ValidationError};

// Handle specific error types
match contract_result {
    Err(PoolError::InsufficientLiquidity) => {
        // Handle insufficient liquidity
    },
    Err(TokenError::InsufficientBalance) => {
        // Handle insufficient balance
    },
    Err(ValidationError::InvalidAmount) => {
        // Handle invalid amount
    },
    Ok(result) => {
        // Process successful result
    }
}
```

## Module Structure

### Types (`types/`)

Core data structures used across Normal Finance contracts:

- **`amm_config`**: AMM configuration and settings
- **`config`**: General configuration types
- **`enums`**: Common enumerations (PoolTier, PoolStatus, etc.)
- **`fees`**: Fee calculation and structure types
- **`index`**: Index fund related types (IndexInfo, Component, etc.)
- **`insurance_fund`**: Insurance and risk management types
- **`oracle`**: Price oracle and registry types
- **`pool`**: Liquidity pool types and structures
- **`rebalance`**: Portfolio rebalancing types
- **`rewards`**: Incentive and reward distribution types
- **`trading`**: Trading and swap related types

### Errors (`errors/`)

Comprehensive error definitions for all contract operations:

- **`access_control_error`**: Permission and role errors
- **`buffer_error`**: Buffer management errors
- **`index_error`**: Index fund operation errors
- **`insurance_fund_error`**: Insurance claim and management errors
- **`liquidity_calculator_error`**: Liquidity calculation errors
- **`math_error`**: Mathematical operation errors
- **`oracle_error`**: Price feed and oracle errors
- **`pool_error`**: Pool operation and validation errors
- **`pool_router_error`**: Pool routing errors
- **`storage_error`**: Data storage and retrieval errors
- **`swap_error`**: Trading and swap errors
- **`token_error`**: Token operation errors
- **`upgrade_error`**: Contract upgrade errors
- **`validation_error`**: Input validation errors

### Events (`events/`)

Structured event types for contract logging:

- **`access_control`**: Permission change events
- **`amm_config`**: AMM configuration events
- **`amm_pool`**: Pool lifecycle events
- **`config`**: Configuration update events
- **`factory`**: Contract deployment events
- **`fees`**: Fee collection and update events
- **`index`**: Index fund events
- **`swap`**: Trading and swap events
- **`upgrade`**: Contract upgrade events

### Storage (`storage/`)

Optimized storage structures for Soroban contracts:

- **`access_control`**: Role and permission storage
- **`amm_pool`**: Pool state storage
- **`buffer`**: Buffer management storage
- **`factory`**: Factory contract storage
- **`index`**: Index fund storage
- **`insurance_fund`**: Insurance fund storage
- **`swap_utility`**: Swap utility storage
- **`token`**: Token storage structures
- **`token_share`**: Token share storage
- **`upgrade`**: Upgrade management storage

## Contributing

We welcome contributions to improve and extend the Normal Finance type system. Please ensure that:

1. All new types include proper documentation
2. Error types include descriptive messages
3. Changes maintain backward compatibility when possible
4. Tests are included for new functionality

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE.md](LICENSE.md) file for details.

## Authors

- **Jay Malve** - _Initial work_ - [jaymalveus@gmail.com](mailto:jaymalveus@gmail.com)
- **Joshua Blew** - _Initial work_ - [joshua@normalfinance.io](mailto:joshua@normalfinance.io)

## Links

- [Normal Finance](https://normalfinance.io)
- [Soroban Documentation](https://soroban.stellar.org/)
- [Stellar Network](https://stellar.org/)

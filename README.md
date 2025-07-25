# Constellation Protocol

A decentralized digital asset bridge that creates a unified constellation of representations across Web2 and Web3 ecosystems. Built on the Internet Computer (IC), Constellation Protocol enables seamless integration between traditional web services, blockchain networks, and digital assets, functioning as a comprehensive oracle and bridge system.

## Overview

Constellation Protocol is a comprehensive bridge and oracle system that unifies digital assets across multiple ecosystems:

- **Multi-Ecosystem Bridge** - Connects Web2 APIs, Ethereum, Internet Computer, and other blockchain networks
- **Oracle Functionality** - Provides verified data feeds and asset representations from traditional web services
- **Unified Asset Registry** - Creates a single source of truth for digital assets across all connected networks
- **Cross-Chain Authentication** - Seamless identity management using SIWE (Sign-In with Ethereum)
- **ICRC-3 Ledger** - Immutable asset tracking and certification across the constellation
- **Modern Web Interface** - React-based frontend for managing cross-ecosystem assets

## Architecture

### Canisters

#### Galaxy Canister 
- Manages asset deployment and mapping across all connected ecosystems
- Handles cross-chain operations and canister lifecycle
- Integrates with Ethereum using `ic-alloy` for signing and transactions
- Supports multiple Ethereum networks (Base Sepolia testnet by default)
- Orchestrates Web2 API integrations and data verification

#### Constellation Canister
- Provides HTTP endpoints for asset management across ecosystems
- Implements ICRC-3 ledger functionality for unified asset tracking
- Handles data ingestion, metadata, and asset storage from Web2 sources
- Manages authentication and authorization across all connected networks
- Serves as oracle endpoint for external data verification

#### EVM RPC Canister (`7hfb6-caaaa-aaaar-qadga-cai`)
- Remote canister for Ethereum RPC interactions
- Enables cross-chain communication between IC and Ethereum
- Provides standardized interface for blockchain data access
- https://github.com/dfinity/evm-rpc-canister

#### SIWE Provider Canister
- Handles Sign-In with Ethereum authentication
- Manages user sessions and cross-chain identity
- https://github.com/kristoferlund/ic-siwe

### Frontend:

The frontend is customizable to the project, but a default is to come that provides a unified interface for managing assets across all connected ecosystems with:
- **React** with TypeScript for modern web development
- **RainbowKit** for multi-chain wallet connections
- **Wagmi** for Ethereum and Web3 interactions
- **IC Agent** for Internet Computer interactions
- **Lit Protocol** for encryption and access control
- **Web2 API Integration** for traditional service connectivity

## Prerequisites

- [DFX](https://internetcomputer.org/docs/current/developer-docs/setup/install/) (Internet Computer SDK)
- [Node.js](https://nodejs.org/) (v18 or higher)
- [Rust](https://rustup.rs/) (for canister development)
- [Git](https://git-scm.com/)

## Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd Constellation_Protocol
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Start local Internet Computer replica**
   ```bash
   dfx start --background
   ```

4. **Deploy the project**
   ```bash
   chmod +x deploy.sh
   ./deploy.sh
   ```

## Development

### Building

```bash
# Build all packages and canisters
npm run build

# Build specific workspace
npm run build --workspace=@project/shared
```

### Development Mode

```bash
# Start development servers
npm run dev
```

### Canister Development

```bash
# Generate candid files
dfx generate

# Deploy specific canister
dfx deploy galaxy

# Update canister settings
dfx canister update-settings --add-controller <principal-id> <canister-id>
```

## Configuration

### Environment Variables

The project uses several configuration files:
- `dfx.json` - DFX configuration and canister definitions
- `canister_ids.json` - Generated canister IDs (auto-updated)
- `packages/shared/src/config/canister-ids.ts` - Frontend canister configuration

### Network Configuration

The project is configured for:
- **Base Sepolia testnet** (default)
- **Local development** network
- **Internet Computer mainnet**
- **Web2 API endpoints** (configurable)
- **Additional blockchain networks** (extensible)

Network settings can be modified in `canisters/galaxy/src/lib.rs`.

## Deployment

### Local Deployment

```bash
./deploy.sh
```

### Mainnet Deployment

1. Update network configuration in `dfx.json`
2. Set appropriate canister IDs
3. Run deployment script with mainnet flag

## API Reference

### Galaxy Canister

- `deploy_asset` - Deploy new asset across connected ecosystems
- `initialize_icrc3_chain` - Launches a connstellation canister
- `mint_token` - Mint tokens on supported blockchain networks
- `transfer_ownership` - Transfer asset ownership across networks
- `get_asset_mapping` - Retrieve unified asset mappings
- `verify_web2_data` - Verify and ingest data from Web2 sources

### Constellation Canister

- `add_block` - Add new block to ICRC-3 ledger for asset tracking
- `add_image` - Upload and store digital assets
- `http_request` - HTTP endpoints for cross-ecosystem asset management
- `oracle_endpoint` - Provide verified data feeds to external systems

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## TODO

### Core Protocol
- **Solana Bridge** - Native Solana asset integration & smart contract interoperability
- **ICRC-7 Migration** - Upgrade to advanced metadata & multi-asset capabilities
- **Custom Contracts** - Upload & manage custom EVM contracts beyond ERC-1155

###  Infrastructure
- **Decentralized Index** - Verifiable asset discovery across canisters & chains
- **Multi-galaxy** - Collaborative asset management across organizations

###  Frontend
- **Modular UI Framework** - Reusable React interface for all asset types & roles

###  Security
- **vetKeys Integration** - Advanced encryption & seamless Internet Identity

###  Governance
- **DAO Toolkits** - Token-based governance for asset permissions & royalties
- **Open Standards** - Contribute to ICRC/W3C federated asset protocols


## Known Issues
### Storage & Memory
- **WASM Storage Limitation** - Canister WASM modules not currently stored in stable memory, requiring redeployment on canister upgrades
- **Asset Metadata Size** - Large metadata objects may exceed canister message size limits


## License

This project is licensed under the **Creative Commons Attribution-NonCommercial 4.0 International License (CC BY-NC 4.0)**.

### What this means:
- ✅ **You can**: Use, modify, and distribute this code for non-commercial purposes
- ✅ **You can**: Build upon this work for personal, educational, or research projects
- ❌ **You cannot**: Use this code for commercial purposes without explicit permission
- ❌ **You cannot**: Sell or monetize products/services built with this code

### For Commercial Use:
If you're interested in using this code for commercial purposes, please contact us to discuss licensing options.

### Attribution:
When using this code, please include:
- A link to this repository
- Attribution to the original authors
- A notice that the work is licensed under CC BY-NC 4.0

For full license details, see: https://creativecommons.org/licenses/by-nc/4.0/

## Support

For questions and support, please [create an issue](link-to-issues) or contact the development team.
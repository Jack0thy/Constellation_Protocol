#!/bin/bash

# Generate the dfx.json files
dfx generate

# Update canister IDs in the frontend config
npm run update-canister-ids

# Build the frontend packages
npm run build

# Deploy the canisters
dfx canister create ic_siwe_provider 
dfx deploy

# Add the controller to the constellation canister
dfx canister update-settings --add-controller gdh4h-lyaaa-aaaal-ar2ba-cai constellation
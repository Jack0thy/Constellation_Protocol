#!/usr/bin/env node

const fs = require('fs');
const path = require('path');

// Read the canister IDs from dfx
function readCanisterIds() {
  try {
    const canisterIdsPath = path.join(__dirname, '..', '.dfx', 'local', 'canister_ids.json');
    const canisterIdsContent = fs.readFileSync(canisterIdsPath, 'utf8');
    return JSON.parse(canisterIdsContent);
  } catch (error) {
    console.error('Error reading canister IDs:', error.message);
    return null;
  }
}

// Update the canister-ids.ts files and Rust config
function updateCanisterIdsConfig(canisterIds) {
  try {
    // Update frontend config
    const frontendConfigPath = path.join(__dirname, '..', 'canisters', 'constellation', 'src', 'config.rs');
    // Update shared package config
    const sharedConfigPath = path.join(__dirname, '..', 'packages', 'shared', 'src', 'config', 'canister-ids.ts');
    // Update Rust backend config
    const rustConfigPath = path.join(__dirname, '..', 'canisters', 'constellation', 'src', 'config.rs');
    
    // Create the new config content
    const configContent = `// This file contains canister IDs for different environments
// It should be updated during the build process to reflect the current canister IDs

export const CANISTER_IDS = {
  // Local development canister IDs (updated by build process)
  local: {
    constellation: "${canisterIds.constellation?.local || 'NOT_FOUND'}",
    galaxy: "${canisterIds.galaxy?.local || 'NOT_FOUND'}",
    evm_rpc: "${canisterIds.evm_rpc?.local || 'NOT_FOUND'}",
    ic_siwe_provider: "${canisterIds.ic_siwe_provider?.local || 'NOT_FOUND'}"   
  },
  // Production canister IDs (mainnet)
  production: {
    constellation: "bd3sg-teaaa-aaaaa-qaaba-cai",
    galaxy: "gdh4h-lyaaa-aaaal-ar2ba-cai",
    evm_rpc: "7hfb6-caaaa-aaaar-qadga-cai",
    ic_siwe_provider: "uxrrr-q7777-77774-qaaaq-cai",
  }
};

// Helper function to get the current environment
export const getEnvironment = (): 'local' | 'production' => {
  const hostname = window.location.hostname;
  
  if (hostname === 'localhost' || hostname.endsWith('.localhost')) {
    return 'local';
  }
  
  return 'production';
};

// Helper function to get canister ID for a specific canister
export const getCanisterId = (canisterName: keyof typeof CANISTER_IDS.local): string => {
  const environment = getEnvironment();
  return CANISTER_IDS[environment][canisterName];
};
`;

    // Create Rust config content
    const rustConfigContent = `use candid::Principal;

// This file contains canister IDs for different environments
// It should be updated during the build process to reflect the current canister IDs

pub struct CanisterIds {
    pub ic_siwe_provider: Principal,
    pub galaxy: Principal,
}

impl CanisterIds {
    pub fn local() -> Self {
        Self {
            ic_siwe_provider: Principal::from_text("${canisterIds.ic_siwe_provider?.local || 'NOT_FOUND'}")
                .expect("Invalid ic_siwe_provider principal"),
            galaxy: Principal::from_text("${canisterIds.galaxy?.local || 'NOT_FOUND'}")
                .expect("Invalid galaxy principal"),
        }
    }

    pub fn production() -> Self {
        Self {
            ic_siwe_provider: Principal::from_text("cpmcr-yeaaa-aaaaa-qaala-cai")
                .expect("Invalid ic_siwe_provider principal"),
            galaxy: Principal::from_text("gdh4h-lyaaa-aaaal-ar2ba-cai")
                .expect("Invalid galaxy principal"),
        }
    }

    // For now, we'll use local IDs for development
    // In the future, this could be determined by environment variables or other means
    pub fn current() -> Self {
        Self::local()
    }
}
`;

    // Write to all config files
    fs.writeFileSync(rustConfigPath, rustConfigContent);
    fs.writeFileSync(sharedConfigPath, configContent);
    fs.writeFileSync(rustConfigPath, rustConfigContent);
    console.log('✅ Updated canister-ids.ts files and Rust config with current local canister IDs');
    
    // Log the updated IDs for verification
    console.log('Current local canister IDs:');
    Object.entries(canisterIds).forEach(([name, ids]) => {
      if (ids.local) {
        console.log(`  ${name}: ${ids.local}`);
      }
    });
    
  } catch (error) {
    console.error('Error updating canister IDs config:', error.message);
  }
}

// Main execution
const canisterIds = readCanisterIds();
if (canisterIds) {
  updateCanisterIdsConfig(canisterIds);
} else {
  console.error('❌ Failed to read canister IDs from dfx');
  process.exit(1);
} 
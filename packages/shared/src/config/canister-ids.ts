// This file contains canister IDs for different environments
// It should be updated during the build process to reflect the current canister IDs

export const CANISTER_IDS = {
  // Local development canister IDs (updated by build process)
  local: {
    constellation: "uxrrr-q7777-77774-qaaaq-cai",
    galaxy: "gdh4h-lyaaa-aaaal-ar2ba-cai",
    evm_rpc: "7hfb6-caaaa-aaaar-qadga-cai",
    ic_siwe_provider: "u6s2n-gx777-77774-qaaba-cai"   
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

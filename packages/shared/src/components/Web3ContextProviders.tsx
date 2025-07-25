// Import required dependencies
import React from 'react';
import '@rainbow-me/rainbowkit/styles.css';
import { getDefaultConfig, RainbowKitProvider, darkTheme, Theme } from '@rainbow-me/rainbowkit';
import { WagmiProvider } from 'wagmi';
import { base } from 'wagmi/chains';
import { QueryClientProvider, QueryClient } from "@tanstack/react-query";
import { SiweIdentityProvider } from 'ic-use-siwe-identity';
import { idlFactory } from "@declarations/ic_siwe_provider/index";
import { _SERVICE } from "@declarations/ic_siwe_provider/ic_siwe_provider.did";
import { getCanisterId } from '../config/canister-ids';
// import { IC_SIWE_PROVIDER_CANISTER_ID } from '../constants';

// Custom museum theme extending the dark theme with specific styling overrides
const museumTheme = {
  ...darkTheme(),
  colors: {
    ...darkTheme().colors,
    accentColor: '#2c3e50',              // Dark blue-gray accent
    accentColorForeground: '#ffffff',     // White text on accent
    connectButtonBackground: '#ffffff',    // White button background
    connectButtonInnerBackground: '#f8f9fa', // Light gray inner button
    connectButtonText: '#2c3e50',         // Dark text on button
    modalBackground: '#ffffff',           // White modal background
    modalText: '#2c3e50',                // Dark text in modal
    modalTextSecondary: '#6c757d',       // Secondary gray text
  },
  fonts: {
    body: 'Times New Roman, serif',      // Classic museum-style font
  },
  radii: {
    ...darkTheme().radii,
    connectButton: '4px',                // Subtle rounded corners for buttons
    modal: '8px',                        // Slightly more rounded corners for modals
  },
} as Theme;

// Configure Wagmi/RainbowKit with base network settings
const config = getDefaultConfig({
  appName: 'Digital Collections Portal',
  projectId: 'YOUR_PROJECT_ID',          // TODO: Replace with actual project ID
  chains: [base],
  ssr: true,
});

// Initialize React Query client for data fetching
const queryClient = new QueryClient();

// Props interface for the Web3ContextProviders component
interface ContextProvidersProps {
  children: React.ReactNode;
}

// Main component that wraps the application with necessary Web3 context providers
export const Web3ContextProviders: React.FC<ContextProvidersProps> = ({ children }) => {
  return (
    <WagmiProvider config={config}>
      <QueryClientProvider client={queryClient}>
        <RainbowKitProvider theme={museumTheme}>
          <SiweIdentityProvider<_SERVICE>
            idlFactory={idlFactory}
            canisterId={getCanisterId('ic_siwe_provider')}
          >
            {children}
          </SiweIdentityProvider>
        </RainbowKitProvider>
      </QueryClientProvider>
    </WagmiProvider>
  );
};
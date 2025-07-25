// Import the dynamic configuration from shared config
import { getCanisterId } from '../config/canister-ids';

// This function is deprecated. Use the getCanisterId function from the config instead.
// Keeping this for backward compatibility but it should be removed in future versions.
function getCanisterIdFromSubdomain() {
    if (import.meta) {
      // Development environment
      console.log('Development environment is true');
      // Use the dynamic config instead of hardcoded ID
      return getCanisterId('constellation');
    }
  
    const hostname = window.location.hostname;
    console.log('hostname', hostname);
    const subdomain = hostname.split('.')[0];
    console.log('subdomain', subdomain);
    return subdomain;
  };
  
  export { getCanisterIdFromSubdomain };
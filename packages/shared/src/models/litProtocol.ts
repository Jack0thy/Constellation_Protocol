import { LIT_ABILITY, LIT_NETWORK } from "@lit-protocol/constants";
import { AccessControlConditions, EncryptToJsonPayload } from "@lit-protocol/types";
import { ethers } from "ethers";
import { 
    LitAccessControlConditionResource,
    createSiweMessageWithRecaps,
    generateAuthSig
 } from "@lit-protocol/auth-helpers";
import { encryptToJson, decryptFromJson } from "@lit-protocol/encryption";
import { LitNodeClient } from '@lit-protocol/lit-node-client';
 
export class Lit {
    litNodeClient: any;
    chain: string;

    constructor(chain: string) {
        this.chain = chain;
    }

    async init() {      
        this.litNodeClient = new LitNodeClient({
            alertWhenUnauthorized: false,
            litNetwork: LIT_NETWORK.DatilTest,
            debug: true,
        });
        await this.litNodeClient.connect();
        console.log("Connected to Lit");
    }

    async disconnect() {
        await this.litNodeClient.disconnect();
        console.log("Disconnected from Lit");
    }

    async getSessionSignatures(){
        // Connect to the wallet
        const provider = new ethers.providers.Web3Provider(window.ethereum);
        await provider.send("eth_requestAccounts", []);
        const signer = provider.getSigner();
        const walletAddress = await signer.getAddress();
        console.log("Connected account:", walletAddress);
     
        // Get the latest blockhash
        const latestBlockhash = await this.litNodeClient.getLatestBlockhash();
     
        // Define the authNeededCallback function
        const authNeededCallback = async(params: any) => {
          if (!params.uri) {
            throw new Error("uri is required");
          }
          if (!params.expiration) {
            throw new Error("expiration is required");
          }
     
          if (!params.resourceAbilityRequests) {
            throw new Error("resourceAbilityRequests is required");
          }
      
          // Create the SIWE message
          const toSign = await createSiweMessageWithRecaps({
            uri: params.uri,
            expiration: params.expiration,
            resources: params.resourceAbilityRequests,
            walletAddress: walletAddress,
            nonce: latestBlockhash,
            litNodeClient: this.litNodeClient,
          });
     
          // Generate the authSig
          const authSig = await generateAuthSig({
            signer: signer,
            toSign,
          });
     
          return authSig;
        }
     
        // Define the Lit resource
        const litResource = new LitAccessControlConditionResource('*');
   
        // Get the session signatures
        const sessionSigs = await this.litNodeClient.getSessionSigs({
            chain: this.chain,
            resourceAbilityRequests: [
                {
                    resource: litResource,
                    ability: LIT_ABILITY.AccessControlConditionDecryption,
                },
            ],
            authNeededCallback
        });
        return sessionSigs;
     }

    // Function to encrypt a file
    async encrypt(file: Blob, accessControlConditions: AccessControlConditions) : Promise<string> {
      // Encrypt the file data
      console.log("Encrypting data...");
      const encryptedData: string = await encryptToJson({
          accessControlConditions: accessControlConditions,
          chain: this.chain,
          file: file,
          litNodeClient: this.litNodeClient,
      });

      return encryptedData;
    }

    // Only supports EncryptToJson Function
    async decrypt(encryptedData: EncryptToJsonPayload) : Promise<string | Uint8Array> {
        // Load decrypt functionality only when needed
        const sessionSigs = await this.getSessionSignatures();
        const decryptedData = await decryptFromJson({
            litNodeClient: this.litNodeClient,
            parsedJsonData: encryptedData,
            sessionSigs: sessionSigs
        });
        return decryptedData;
    }    
}
import { idlFactory } from "@declarations/constellation/constellation.did.js";
import type { _SERVICE as ConstellationService } from "@declarations/constellation/constellation.did.js";
import { BaseCanister } from "./base";
import { ChunkedFile } from "../chunk";

class ConstellationCanister extends BaseCanister<ConstellationService> {
    
    public canisterId!: string;
    

    public constructor(canister: any) {
        super(canister);
    }

    public static async create(canisterId: string): Promise<ConstellationCanister> {
        const canister = await this.createActor(canisterId, idlFactory);
        const constellationCanister = new ConstellationCanister(canister);
        constellationCanister.canisterId = canisterId;
        return constellationCanister;
    }
         

    public async addImage(imageName: string, imageType: string, imageData: Uint8Array): Promise<void> {
        return await this.canister.add_image(imageName, imageType, imageData);
    }
 




    public async getChainData(): Promise<string> {
        const chainData = await this.canister.json_get_entire_chain();
        console.log(chainData);
        return chainData;
    }

    public async getContractAddress(): Promise<string> {
        const contractAddress = await this.canister.get_contract_address();
        console.log(contractAddress);
        return contractAddress;
    }

   
}

export { ConstellationCanister };
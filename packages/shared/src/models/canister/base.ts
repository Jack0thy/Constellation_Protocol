import { Actor, HttpAgent, ActorSubclass } from "@dfinity/agent";

/**
 * Abstract base class implementing common canister functionality
 */
export abstract class BaseCanister<T> {
    protected canister!: ActorSubclass<T>;

    protected constructor(canister: ActorSubclass<T>) {
        this.canister = canister;
    }
    /**
     * Creates an HTTP agent based on the environment
     */
    protected static async createAgent(): Promise<HttpAgent> {
        const isLocalEnv = window.location.host.includes('localhost') || 
                          window.location.host.includes('127.0.0.1');
        
        const agent = new HttpAgent({
            host: isLocalEnv ? 'http://127.0.0.1:4943' : 'https://icp0.io'
        });

        if (isLocalEnv) {
            try {
                await agent.fetchRootKey();
                console.log("Root key fetched successfully");
            } catch (e) {
                console.warn("Error fetching root key:", e);
                const dummyKey = new Uint8Array(32).fill(0).buffer;
                agent.rootKey = dummyKey;
                agent.fetchRootKey = async () => {
                    console.log("Using dummy root key");
                    return dummyKey;
                };
            }
        }

        return agent;
    }

    /**
     * Creates a new actor instance for the canister
     * @param canisterId The ID of the canister to connect to
     * @param idlFactory The interface description language factory for the canister
     */
    protected static async createActor<T>(
        canisterId: string,
        idlFactory: any
    ): Promise<ActorSubclass<T>> {
        const agent = await this.createAgent();
        return Actor.createActor(idlFactory, {
            agent,
            canisterId: canisterId
        });
    }
}

import { ReactNode } from "react";
import {
  ActorProvider,
  createActorContext,
  createUseActorHook,
} from "ic-use-actor";
import { idlFactory } from "@declarations/constellation/constellation.did.js";
import { _SERVICE as ConstellationService } from "@declarations/constellation/constellation.did.js";
import { useSiweIdentity } from "ic-use-siwe-identity";

const constellationActorContext = createActorContext<ConstellationService>();
const useConstellationActor = createUseActorHook<ConstellationService>(constellationActorContext);

function ConstellationActorProvider ({ children, canisterId }: { children: ReactNode, canisterId: string }) {
  const { identity } = useSiweIdentity();

  return (
      <ActorProvider<ConstellationService>
        canisterId={canisterId}
        context={constellationActorContext}
        identity={identity}
        idlFactory={idlFactory}
      >
        {children}
      </ActorProvider>
  );
}

export { ConstellationActorProvider, useConstellationActor };



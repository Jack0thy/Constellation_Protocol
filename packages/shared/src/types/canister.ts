import { _SERVICE as GalaxyService } from '@declarations/galaxy/galaxy.did.js';
import { _SERVICE as ConstellationService } from '@declarations/constellation/constellation.did.js';
import { ActorSubclass } from '@dfinity/agent';

export type Galaxy = ActorSubclass<GalaxyService>;
export type Constellation = ActorSubclass<ConstellationService>;


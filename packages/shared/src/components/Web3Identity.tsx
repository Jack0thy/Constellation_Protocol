import { useSiweIdentity } from "ic-use-siwe-identity";

function Web3Identity() {
  const { identity } = useSiweIdentity();

  if (!identity) return null;

  return <div>{identity.getPrincipal().toString()}</div>;
}

export { Web3Identity };
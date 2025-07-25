import { useAccount } from "wagmi";
import { useSiweIdentity } from "ic-use-siwe-identity";

function Web3LoginButton() {
  const { isConnected } = useAccount();
  const { login, loginStatus, identity } = useSiweIdentity();

  if (identity) return null;

  const text =
    loginStatus === "logging-in" ? "Signing in …" : "Sign in with Ethereum";

  const disabled = loginStatus === "logging-in" || !isConnected;

  return (
    <button
      disabled={disabled}
      onClick={() => void login()}
      className="block rounded-md py-2.5 px-3.5 text-center focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 bg-[#522785] disabled:bg-[#52278580] shadow-sm hover:bg-white/50 focus-visible:outline-white"
    >
      {text}
    </button>
  );
}

export { Web3LoginButton };
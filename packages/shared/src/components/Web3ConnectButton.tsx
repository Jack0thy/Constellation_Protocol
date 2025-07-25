import { FC } from 'react';
import { ConnectButton } from '@rainbow-me/rainbowkit';

const Web3ConnectButton: FC = () => {
  return (
    <ConnectButton.Custom>
      {({
        account,
        chain,
        openAccountModal,
        openChainModal,
        openConnectModal,
        mounted,
      }) => {
        const ready = mounted;
        const connected = ready && account && chain;

        return (
          <div
            {...(!ready && {
              'aria-hidden': true,
              style: {
                opacity: 0,
                pointerEvents: 'none',
                userSelect: 'none',
              },
            })}
          >
            {(() => {
              if (!connected) {
                return (
                  <button
                    onClick={openConnectModal}
                    className="museum-connect-button"
                    type="button"
                  >
                    Connect Institutional Account
                  </button>
                );
              }

              return (
                <div className="museum-account-container">
                  <button
                    onClick={openChainModal}
                    className="museum-chain-button"
                    type="button"
                  >
                    {chain.hasIcon && (
                      <div className="museum-chain-icon">
                        {chain.iconUrl && (
                          <img
                            alt={chain.name ?? 'Chain icon'}
                            src={chain.iconUrl}
                            style={{ width: 12, height: 12 }}
                          />
                        )}
                      </div>
                    )}
                    <span className="museum-chain-name">
                      {chain.name}
                    </span>
                  </button>

                  <button
                    onClick={openAccountModal}
                    className="museum-account-button"
                    type="button"
                  >
                    <span className="museum-account-address">
                      {account.displayName}
                    </span>
                  </button>
                </div>
              );
            })()}
          </div>
        );
      }}
    </ConnectButton.Custom>
  );
};

export { Web3ConnectButton }; 
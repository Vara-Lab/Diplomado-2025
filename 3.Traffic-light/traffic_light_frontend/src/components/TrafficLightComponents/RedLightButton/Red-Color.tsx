
import { useAccount, useApi, useAlert } from "@gear-js/react-hooks";
import { web3FromSource } from "@polkadot/extension-dapp";
import { Button } from "@chakra-ui/react";
import { useSailsCalls } from "@/app/hooks";

function RedColor() {
  const sails = useSailsCalls();
  const alert = useAlert();
  const { account } = useAccount();

  const signer = async () => {
    if (!account) {
      alert.error("Account not available to sign");
      return;
    }

    if (!sails) {
      alert.error('SailsCalls is not ready');
      return;
    }

    const { signer } = await web3FromSource(account.meta.source);

    const response = await sails.command(
      'TrafficLight/Red',
      {
        userAddress: account.decodedAddress,
        signer
      },
      {
        callbacks: {
          onLoad() { alert.info('Will send a message'); },
          onBlock(blockHash) { alert.success(`In block: ${blockHash}`); },
          onSuccess() { alert.success('Message send!'); },
          onError() { alert.error('Error while sending message'); }
        }
      }
    );

    console.log(`response: ${response}`);
  };
  
  return (<Button backgroundColor="red.300" onClick={signer} > Red</Button>)
}

export { RedColor };


import { GasPrice } from '@cosmjs/stargate';
import { SigningCosmWasmClient } from "@cosmjs/cosmwasm-stargate";
import { getOfflineSignerAmino } from "cosmjs-utils";
import { chains } from 'chain-registry';
import * as fs from 'fs'
require('dotenv').config()

// TODO: Replace with your mnemonic (not recommended for production use)
const { MNEMONIC } = process.env
const chain = chains.find(({ chain_name }) => chain_name === 'osmosistestnet');
const contractAddress =
  'osmo19jdjvm7ellwqlv7873nlx9lz36x6q6q6yl62dd7ygppnhudxej2stewy2e';
const codePath = '../artifacts/passport-aarch64.wasm'
const owner = 'osmo1z98eg2ztdp2glyla62629nrlvczg8s7f8sgpm5'

async function delay(ms: number): Promise<void> {
  return new Promise<void>(resolve => setTimeout(resolve, ms));
}

const main = async () => {
  const rpcEndpoint = 'https://rpc-test.osmosis.zone/';
  const signer = await getOfflineSignerAmino({ mnemonic: MNEMONIC, chain });
  const client = await SigningCosmWasmClient.connectWithSigner(
    rpcEndpoint,
    signer,
    {
      gasPrice: GasPrice.fromString("0.025uosmo"),
    }
  );

  const [sender] = await signer.getAccounts();
  const fee = "auto";
  const uploadResult = await client.upload(sender.address, fs.readFileSync(codePath), fee);
  const { codeId } = uploadResult
  console.log(`codeId: ${codeId}`)

  const initResult = await client.instantiate(sender.address, codeId,
    {
      "admin": sender.address,
      "name": "PassPortNFT",
      "symbol": "PP"
    }, 'passport-nft', fee, {
    admin: sender.address
  })
  const { contractAddress } = initResult
  console.log(`contract address: ${contractAddress}`)
  await delay(2000)
  // issue (note: ugly usage, should use batch instead)
  await client.executeMultiple(sender.address, [
    // icns
    {
      msg: {
        issue: {
          owner: owner,
          category: "icns",
          badge: "ICNS: dogemos"
        }
      },
      contractAddress
    },
    // governance
    {
      msg: {
        issue: {
          owner: owner,
          category: "governance",
          badge: "Governance: Delegator"
        }
      },
      contractAddress
    }, {
      msg: {
        issue: {
          owner: owner,
          category: "governance",
          badge: "Governance: Proposal Voter"
        }
      },
      contractAddress
    }, {
      msg: {
        issue: {
          owner: owner,
          category: "governance",
          badge: "Governance: Proposal Creator"
        }
      },
      contractAddress
    },
    // osmosis
    {
      msg: {
        issue: {
          owner: owner,
          category: "osmosis",
          badge: "Osmosis: Swapper"
        }
      },
      contractAddress
    }, {
      msg: {
        issue: {
          owner: owner,
          category: "osmosis",
          badge: "Osmosis: Heavy Swapper"
        }
      },
      contractAddress
    },
    // mars
    {
      msg: {
        issue: {
          owner: owner,
          category: "mars",
          badge: "Mars: Red Bank User"
        }
      },
      contractAddress
    },
    // source 
    {
      msg: {
        issue: {
          owner: owner,
          category: "source",
          badge: "Source: IBC"
        }
      },
      contractAddress
    },
    // mint
    {
      msg: {
        mint: {
          owner: owner
        }
      },
      contractAddress
    },
    // claim
    {
      msg: {
        claim: {
          owner: owner,
          category: "icns",
          badge: "ICNS: dogemos"
        }
      },
      contractAddress
    },
    {
      msg: {
        claim: {
          owner: owner,
          category: "governance",
          badge: "Governance: Delegator"
        }
      },
      contractAddress
    }, {
      msg: {
        claim: {
          owner: owner,
          category: "governance",
          badge: "Governance: Proposal Voter"
        }
      },
      contractAddress
    }, {
      msg: {
        claim: {
          owner: owner,
          category: "governance",
          badge: "Governance: Proposal Creator"
        }
      },
      contractAddress
    }, {
      msg: {
        claim: {
          owner: owner,
          category: "osmosis",
          badge: "Osmosis: Swapper"
        }
      },
      contractAddress
    }, {
      msg: {
        claim: {
          owner: owner,
          category: "osmosis",
          badge: "Osmosis: Heavy Swapper"
        }
      },
      contractAddress
    }, {
      msg: {
        claim: {
          owner: owner,
          category: "mars",
          badge: "Mars: Red Bank User"
        }
      },
      contractAddress
    }, {
      msg: {
        claim: {
          owner: owner,
          category: "source",
          badge: "Source: IBC"
        }
      },
      contractAddress
    }
  ], fee)
  await delay(2000)
};

main();
;
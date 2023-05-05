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
      "name": "test",
      "symbol": "T"
    }, 'test', fee, {
    admin: sender.address
  })
  const { contractAddress } = initResult
  console.log(`contract address: ${contractAddress}`)
  // issue
  await client.execute(sender.address, contractAddress, {
    issue: {
      owner: sender.address,
      category: "osmo",
      badge: "whale"
    }
  }, fee)
};

main();
;
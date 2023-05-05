import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
const rpcURL = "https://rpc-test.osmosis.zone/";
const contractAddress =
  "osmo1ds76kax8amq7865xqfza7xsersdkf0g0w2vf22up2pes4gejmaqqgft4fl";
const queryMsg = `{
  "badges": {
    "owner": "osmo13v5gkp8g2qwwycxf2jmzegvee75hrgg5qp2z50"
  }
}`;

const queryContract = async (rpcURL, contractAddress, queryMsg) => {
  const client = await SigningCosmWasmClient.connect(rpcURL);
  const queryResult = await client.queryContractSmart(
    contractAddress,
    JSON.parse(queryMsg)
  );
  console.log(queryResult);
};

queryContract(rpcURL, contractAddress, queryMsg);
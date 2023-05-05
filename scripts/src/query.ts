import { SigningCosmWasmClient } from '@cosmjs/cosmwasm-stargate';
const rpcURL = "https://rpc-test.osmosis.zone/";
const contractAddress =
  "osmo1d6y077y8uhhmls0ar3vxzddqc9udrka0zu5ukg6uw2s4juatxv4sz3yxz4";
const queryMsg = `{
  "badges": {
    "owner": "osmo19tpkal6x788tlahpztt6p6xlwlt3yylpcu8g63"
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
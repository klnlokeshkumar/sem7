const { Web3 } = require("web3");
const { ETH_DATA_FORMAT, DEFAULT_RETURN_FORMAT } = require("web3");
async function main() {
  // Configuring the connection to an Ethereum node
  const web3 = new Web3(
    new Web3.providers.HttpProvider(
      "https://sepolia.infura.io/v3/35c2b7c8ecc6493cb073943d1bb7d15a" //add your api key
    )
  );
  const latestBlock = await web3.eth.getBlock("latest");
  const baseFeePerGas = latestBlock.baseFeePerGas;
  const maxFeePerGas =
    BigInt(baseFeePerGas) + BigInt(web3.utils.toWei("2", "gwei"));
  // Creating a signing account from a private key
  const signer = web3.eth.accounts.privateKeyToAccount(
    "45e8183d1609d34a0008ed95b6ec625c07ffbe46aa593a52c2051806163a0874"
  ); //add your private

  web3.eth.accounts.wallet.add(signer);
  await web3.eth
    .estimateGas(
      {
        from: signer.address,
        to: "0x1328b19533f4bA383c5721845c4478b3B4e7A388", //Add recipient_address
        value: web3.utils.toWei("0.0001", "ether"),
      },
      "latest",
      ETH_DATA_FORMAT
    )
    .then((value) => {
      limit = value;
    });
  // Creating the transaction object
  const tx = {
    from: signer.address,
    to: "0x1328b19533f4bA383c5721845c4478b3B4e7A388",
    value: web3.utils.toWei("0.001", "ether"), // change AMOUNT to send
    gas: limit,
    nonce: await web3.eth.getTransactionCount(signer.address),
    maxPriorityFeePerGas: web3.utils.toWei("2", "gwei"),

    maxFeePerGas: maxFeePerGas.toString(),
    chainId: 11155111,
    type: 0x2,
  };
  signedTx = await web3.eth.accounts.signTransaction(tx, signer.privateKey);
  console.log("Raw transaction data: " + signedTx.rawTransaction);
  // Sending the transaction to the network
  const receipt = await web3.eth
    .sendSignedTransaction(signedTx.rawTransaction)
    .once("transactionHash", (txhash) => {
      console.log(`Mining transaction ...`);
      console.log(`https://sepolia.etherscan.io/tx/${txhash}`);
    });
  // The transaction is now on chain!
  console.log(`Mined in block ${receipt.blockNumber}`);
}
main();

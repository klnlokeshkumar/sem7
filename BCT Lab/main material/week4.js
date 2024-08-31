const { Web3 } = require("web3");
// Connecting to web3 instance
const web3 = new Web3(
  "https://mainnet.infura.io/v3/9ef65ae1fe6c4c68b0a842493dfadeba"
);
// web3.eth.getBlockNumber().then(console.log);

// storing ganache url
const ganacheUrl = "HTTP://127.0.0.1:7545";

// retrieving the network id after connecting to the block
web3.eth.net
  .getId()
  .then((networkId) => {
    console.log("Connected to network ID:", networkId);
  })
  .catch((error) => {
    console.error("Error connecting to Ganache:", error);
  });

// retrieving account balance from a ganache url;
const accountAddress = "0xa9CD42358E4f147c02fd5F47804DD614c4e234Df";
web3.eth.getBalance(accountAddress).then((balance) => {
  console.log(
    "Account balance:",
    web3.utils.fromWei(balance, "ether"),
    "ETH"
  );
})
  .catch((error) => {
    console.error("Error fetching balance:", error);
  });

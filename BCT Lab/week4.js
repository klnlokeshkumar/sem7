const { Web3 } = require("web3");
const web3 = new Web3(
  "https://mainnet.infura.io/v3/9ef65ae1fe6c4c68b0a842493dfadeba"
);
// web3.eth.getBlockNumber().then(console.log);

const ganacheUrl = "HTTP://127.0.0.1:7545";
web3.eth.net
  .getId()
  .then((networkId) => {
    console.log("Connected to network ID:", networkId);
  })
  .catch((error) => {
    console.log("Connected to network ID:", networkId);
  })
  .catch((error) => {
    console.error("Error connecting to Ganache:", error);
  });
const accountAddress = "0xbc14dDeCD661d9de02ba1320d0C6204eB0BC160F";
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

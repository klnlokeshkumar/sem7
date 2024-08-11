const { Web3 } = require("web3");
const web3 = new Web3("http://127.0.0.1:7545"); // Ganache RPC server address

// Get the contract ABI and address from the build files
const contractABI = [
  {
    inputs: [],
    name: "storedData",
    outputs: [
      {
        internalType: "uint256",
        name: "",
        type: "uint256",
      },
    ],
    stateMutability: "view",
    type: "function",
    constant: true,
  },
  {
    inputs: [
      {
        internalType: "uint256",
        name: "x",
        type: "uint256",
      },
    ],
    name: "set",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [],
    name: "get",
    outputs: [
      {
        internalType: "uint256",
        name: "",
        type: "uint256",
      },
    ],
    stateMutability: "view",
    type: "function",
    constant: true,
  },
];
const contractAddress = "0x98a3B8762234B60B2CC6CD24329a219e95AD58d1";

const simpleStorage = new web3.eth.Contract(contractABI, contractAddress);

// Interact with the contract
async function interactWithContract() {
  const accounts = await web3.eth.getAccounts();
  const receipt = await simpleStorage.methods
    .set(42)
    .send({ from: accounts[0] });
  console.log("Transaction receipt:", receipt);

  const value = await simpleStorage.methods.get().call();
  console.log("Stored value:", value);
}
interactWithContract();

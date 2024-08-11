// 3A
const {Web3} = require('web3');
const web3 = new Web3('https://Sepolia.infura.io/v3/35c2b7c8ecc6493cb073943d1bb7d15a');

// const ganacheUrl = 'HTTP://127.0.0.1:7545'
// const web3 = new Web3(ganacheUrl);

web3.eth.getBalance('0xd8b03A4c6B9566a7823dc56d4A5824640B7662Ec').then(console.log);
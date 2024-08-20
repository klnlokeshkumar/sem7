const {Web3} = require('web3');
const web3 = new Web3('https://mainnet.infura.io/v3/6bec4208c9ee41b4bfc86e2319209264');

web3.eth.getBlockNumber().then(console.log).catch(console.error);
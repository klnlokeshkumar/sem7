
//3B
const{Web3} = require('web3');
const web3 = new Web3('https://mainnet.infura.io/v3/35c2b7c8ecc6493cb073943d1bb7d15a');
web3.eth.getBlockNumber()
.then(console.log)
.catch(console.error);

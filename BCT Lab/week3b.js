
//3B
const{Web3} = require('web3');
const web3 = new Web3('https://mainnet.infura.io/v3/9ef65ae1fe6c4c68b0a842493dfadeba');
web3.eth.getBlockNumber()
.then(console.log)
.catch(console.error);

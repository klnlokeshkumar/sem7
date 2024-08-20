const { Web3 } = require('web3'); 
const web3 = new Web3 ('https://mainnet.infura.io/v3/6bec4208c9ee41b4bfc86e2319209264'); 
const ganacheUrl = 'HTTP://127.0.0.1:7545'; 
web3.eth.net.getId() 
.then((networkId) => { console.log('Connected to network ID:', networkId); }) 
.catch((error) => { console.log('Connected to network ID:', networkId); }) .catch((error) => { console.error('Error connecting to Ganache:', error); }); 
const accountAddress = '0x95642599F8E4507669E78106ADedAAD67f195ba0'; 
web3.eth.getBalance(accountAddress) 
.then((balance) => { 
console.log('Account balance:', web3.utils.fromWei(balance, 'ether'), 'ETH'); 
}) 
.catch((error) => { 
console.error('Error fetching balance:', error); 
});
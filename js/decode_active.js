const ethers = require('ethers');

const data = process.argv[2] || (
    console.log("no ABI encoded active valset provided"),
    process.exit(1)
);
const schema = '(address[],uint256[],uint256)';
const [decoded] = ethers.utils.defaultAbiCoder.decode([schema], data);

const [addresses, powers, nonce] = decoded;

console.log(decoded);
console.log(addresses, powers.map(p => p.toNumber()), nonce.toNumber());

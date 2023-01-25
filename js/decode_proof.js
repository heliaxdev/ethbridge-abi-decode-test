const ethers = require('ethers');

const data = process.argv[2] || (
    console.log("no ABI encoded active valset provided"),
    process.exit(1)
);
const schema = '(bytes32,bytes32,(bytes32,bytes32,uint8)[])';
const [decoded] = ethers.utils.defaultAbiCoder.decode([schema], data);

console.log(decoded);
console.log(decoded[2]);

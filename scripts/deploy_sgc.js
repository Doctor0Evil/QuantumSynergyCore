const hre = require("hardhat");

async function main() {
  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying SGC with:", deployer.address);
  const SGC = await hre.ethers.getContractFactory("SGC");
  const sgc = await SGC.deploy();
  await sgc.deployed();
  console.log("SGC deployed to:", sgc.address);
}

main().catch((e) => {
  console.error(e);
  process.exitCode = 1;
});

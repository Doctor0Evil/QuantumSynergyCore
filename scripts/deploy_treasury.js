const hre = require("hardhat");

async function main() {
  const sgcAddress = process.env.SGC_ADDRESS;
  const daoAddress = process.env.SGC_DAO_ADDRESS;
  if (!sgcAddress || !daoAddress) {
    throw new Error("Missing SGC_ADDRESS or SGC_DAO_ADDRESS");
  }
  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying Treasury with:", deployer.address);
  const Treasury = await hre.ethers.getContractFactory("SGCTreasury");
  const treasury = await Treasury.deploy(sgcAddress, daoAddress);
  await treasury.deployed();
  console.log("Treasury deployed to:", treasury.address);
}

main().catch((e) => {
  console.error(e);
  process.exitCode = 1;
});

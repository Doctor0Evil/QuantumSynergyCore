const hre = require("hardhat");

async function main() {
  const sgcAddress = process.env.SGC_ADDRESS;
  if (!sgcAddress) {
    throw new Error("Missing SGC_ADDRESS");
  }
  const maxLoan = hre.ethers.parseUnits("1000000", 18);
  const feeBps = 9;
  const [deployer] = await hre.ethers.getSigners();
  console.log("Deploying FlashLoan with:", deployer.address);
  const FlashLoan = await hre.ethers.getContractFactory("SGCFlashLoan");
  const flash = await FlashLoan.deploy(sgcAddress, maxLoan, feeBps);
  await flash.deployed();
  console.log("FlashLoan deployed to:", flash.address);
}

main().catch((e) => {
  console.error(e);
  process.exitCode = 1;
});

const MiMC = artifacts.require("utils/MiMC");
const MerkleTree = artifacts.require("utils/MerkleTree");
const Verifier = artifacts.require("utils/Verifier");
const Ledger = artifacts.require("Ledger");

async function doDeploy(deployer, network) {
    await deployer.deploy(MiMC);
    await deployer.link(MiMC, MerkleTree);

    await deployer.deploy(MerkleTree);
    await deployer.link(MiMC, Ledger)

    await deployer.deploy(Verifier);
    await deployer.link(Verifier, Ledger);

    await deployer.deploy(Ledger);
}


module.exports = (deployer, network) => {
    deployer.then(async () => {
        await doDeploy(deployer, network);
    });
};

const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;

async function listService(provider, serviceDetails) {
  const program = anchor.workspace.ServiceMarketplace;
  const serviceAccount = anchor.web3.Keypair.generate();

  await program.rpc.listService(
    serviceDetails.serviceName,
    serviceDetails.description,
    new anchor.BN(serviceDetails.price),
    serviceDetails.isSoulbound,
    {
      accounts: {
        serviceAccount: serviceAccount.publicKey,
        vendor: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [serviceAccount],
    }
  );
}

async function purchaseService(provider, servicePublicKey, vendorPublicKey, consumerTokenAccount, vendorTokenAccount) {
  const program = anchor.workspace.ServiceMarketplace;

  await program.rpc.purchaseService({
    accounts: {
      serviceAccount: servicePublicKey,
      vendor: vendorPublicKey,
      consumer: provider.wallet.publicKey,
      consumerTokenAccount: consumerTokenAccount,
      vendorTokenAccount: vendorTokenAccount,
      tokenProgram: anchor.web3.TOKEN_PROGRAM_ID,
    },
  });
}

module.exports = {
  listService,
  purchaseService,
};

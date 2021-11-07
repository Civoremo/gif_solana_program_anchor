/** @format */

const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

const main = async () => {
  console.log("\nStart test...\n");

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.GifSolanaProgram;
  const baseAccount = anchor.web3.Keypair.generate();

  const tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });

  console.log("Transaction signature\n");
  console.log("-------------------------\n");
  console.log(tx, "\n");
  console.log("-------------------------\n\n");

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("Gif Count ---- ", account.totalGifs.toString());

  await program.rpc.addGif("insert_a_giphy_link_here", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("Gif Count ---- ", account.totalGifs.toString());

  console.log("Gif List ---- ", account.gifList);

  await program.rpc.upvoteGif(new anchor.BN(0), {
    accounts: {
      baseAccount: baseAccount.publicKey,
    },
  });

  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log("Gif List ---- ", account.gifList);

  await program.rpc.tipUser(new anchor.BN(10000000), {
    accounts: {
      fromUser: provider.wallet.publicKey,
      toUser: "24xuc8grWYzP8Vb2yzbvXyEafkWvcH8nEazzrGSnRSsc",
      systemProgram: SystemProgram.programId,
    },
  });
};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();

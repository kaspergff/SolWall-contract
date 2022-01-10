const anchor = require('@project-serum/anchor');
const { SystemProgram } = anchor.web3;
const BigNumber = require('bignumber.js');


const main = async() => {
  console.log("🚀 Starting test...")

  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Myepicproject;
  const baseAccount = anchor.web3.Keypair.generate();
  let tx = await program.rpc.startStuffOff({
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: SystemProgram.programId,
    },
    signers: [baseAccount],
  });
  console.log("📝 Your transaction signature", tx);

  let account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString())

  // You'll need to now pass a GIF link to the function! You'll also need to pass in the user submitting the GIF!
  await program.rpc.addGif("https://giphy.com/clips/hamlet-jJjb9AUHOiP3nJJMdy", {
    accounts: {
      baseAccount: baseAccount.publicKey,
      user: provider.wallet.publicKey,
    },
  });


  const multipleVotes = async () => {
    for (let i = 0; i < 3; i++) {
      await program.rpc.updateItem("https://giphy.com/clips/hamlet-jJjb9AUHOiP3nJJMdy",provider.wallet.publicKey.toString(), {
        accounts: {
          baseAccount: baseAccount.publicKey
        },
      });
    }
  }


  const send = async () => {
    tx = await program.rpc.sendSol('1', { 
      accounts: {
        from: provider.wallet.publicKey,
        to: "y7Cexhzak96apikq4q9mQfDmXyWwQkBBvHU7NozKboF",
        systemProgram: SystemProgram.programId,
      },
    })
      console.log('Success!')
  }


  await multipleVotes()

  await send()

  
  // Call the account.
  account = await program.account.baseAccount.fetch(baseAccount.publicKey);
  console.log('👀 GIF Count', account.totalGifs.toString())

  // Access gif_list on the account!
  console.log('👀 GIF List', account.gifList)
}

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
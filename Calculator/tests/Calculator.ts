const assert = require("assert")
const anchor = require("@project-serum/anchor");
const {SystemProgram} = anchor.web3


//this is a mocha test like used in truffle and hardhat for solidity devs
//takes a describe block it block
//defined here - app name then fn 

describe('Calculator', () => {
    //provider is an abstraction of a connection to a solana node
    const provider = anchor.AnchorProvider.env();
    //or use const provider = anchor.AnchorProvider.local();
    /*this way is outdated
    const provider = anchor.Provider.local();
    */
    anchor.setProvider(provider);
    //here is a varaible representing the calculator account 
    //we use a keypair to represent the account generated via anchor
    //it is just the account creation the correct context is later
    const calculator = anchor.web3.Keypair.generate()
    //allows us to call the methods of the solana program that has been written
    const program = anchor.workspace.Calculator

    it("create a calculator", async() => {
        //calls the function
        //init_message is "welcome" first arg
        await program.rpc.create("welcome", {
            //this is the context in the contract
            accounts: {
                //all accounts that are part of context
                //calls calc creditials created above
                calculator: calculator.publicKey,
                //wallet that creates the calculator
                user: provider.wallet.publicKey,
                //last import
                systemProgram: SystemProgram.programId
            },
            signers: [calculator]
        })
        const account = await program.account.calculator.fetch(calculator.publicKey)
            //check if greeting field is correct
            assert.ok(account.greeting == "welcome")
    })
})
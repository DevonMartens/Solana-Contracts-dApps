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

    it("creates a calculator", async() => {
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
    it("adds two numbers", async() => {
        //calls the add function function
        //we will use anchor big numbers because we cannot directly call # BN
        //final argument is the context
        await program.rpc.add(new anchor.BN(2), new anchor.BN(3), {
            accounts: {
                //all accounts that are part of context
                //calls calc creditials created above
                calculator: calculator.publicKey,
            }
        })
        const account = await program.account.calculator.fetch(calculator.publicKey)
        //check if result is 5
        assert.ok(account.result.eq(new anchor.BN(5)))
    })
    it("subtracts one number from another", async() => {
        //calls the sub function function
        await program.rpc.sub(new anchor.BN(10), new anchor.BN(7), {
            accounts: {
                calculator: calculator.publicKey,
            }
        })
        const account = await program.account.calculator.fetch(calculator.publicKey)
        //check if result is 3
        assert.ok(account.result.eq(new anchor.BN(3)))
    })
    it("divides one number by another", async() => {
        //calls the div function function
        await program.rpc.div(new anchor.BN(12), new anchor.BN(3), {
            accounts: {
                calculator: calculator.publicKey,
            }
        })
        const account = await program.account.calculator.fetch(calculator.publicKey)
        //check if result is 4
        assert.ok(account.result.eq(new anchor.BN(4)))
    })
})
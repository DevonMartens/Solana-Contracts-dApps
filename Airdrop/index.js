 const {
     Connection,
     PublicKey,
     clusterApiUrl,
     Keypair,
     LAMPORTS_PER_SOL
 } = require("@solana/web3.js")
//creates wallet object
//new creates a new object & keypair An account keypair used for signing transactions.
const wallet = new Keypair()
//gets creditials of wallet

/*get public key
const publicKey = (wallet._keypair.publicKey)*/
//create new public key
const publicKey = new PublicKey(wallet._keypair.publicKey)
//get secret key
const secretKey = wallet._keypair.secretKey
//view
console.log(publicKey)
console.log(secretKey)
/*view balance with getBalance from web3.js's connection class
this is an async function with a try catch block 
The try statement allows you to define a block of code to be tested for errors while it is being executed
The catch statement allows you to define a block of code to be executed, if an error occurs in the try block.
*/
/* Connection is used to interact with the Solana JSON RPC. You can use Connection to confirm transactions, get account info, and more.

You create a connection by defining the JSON RPC cluster endpoint and the desired commitment. 
Once this is complete, you can use this connection object to interact with any of the Solana JSON RPC API.*/
const getWalletBalance = async() => {
    try{
        //devnet is the a devnet fake solana. Passes clusterApiUrl into connection object to get it's details
        const connection = new Connection(clusterApiUrl('devnet'), 'confirmed')
        //to specify where we are getting the balance from - calling public key from uptop
        const walletBalance = await connection.getBalance(publicKey)
        //results
        console.log(`Wallet Balance is ${walletBalance}`)
    } catch(err){
        console.log(err)
    }
}
/*Note: The current maximum for airdrops on devnet is usually limited to 1 SOL,
 so you can just change the requestAirdrop line to*/

//This function airdrops "sol" into the wallet
const airDropSol = async() => {
    try{
        //create a connection object for the function
        const connection = new Connection(clusterApiUrl('devnet'), 'confirmed')
        //passes in public key as 1st argument 2 amount of "sol"(units in lambhorts - 2 sol total)
        const fromAirDropSigniture = await connection.requestAirdrop(publicKey, 2 * LAMPORTS_PER_SOL)
        //confirms transaction
        await connection.confirmTransaction(fromAirDropSigniture)
    }catch(err){
        console.log(err)
    }
}

 //function allowing us to run code
 const main = async() => {
     await getWalletBalance()
     await airDropSol()
     await getWalletBalance()   
 }
 main()
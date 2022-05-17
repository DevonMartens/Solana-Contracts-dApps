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

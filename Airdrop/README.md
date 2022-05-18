# Airdrop

---

### This project allows airdropping using web3.js/solana. It is simply proof of concept.

## Tools 

* @solana/web3.js

## Usage

1. Creates public and private keys - these can be seen from console.logs in index.js on line 20 & 21.

2. Retrives the balance of the wallet. This means creating a new instance of a public key changing line 14 to line 16. See below:

   ```javascript
   const publicKey = (wallet._keypair.publicKey)*/
   //create new public key
   const publicKey = new PublicKey(wallet._keypair.publicKey)
   ```

This will print out:

   ```shell
   Wallet Balance is 0
   ```

This is because there are no funds until you airdrop funds into the wallet. 

3. The `airDropSol` functions gives 2 sol to the wallet as such the `main` functions calls it and checks the balance a second time to confirm.

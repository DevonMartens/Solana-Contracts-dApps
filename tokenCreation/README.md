# Create a cryptocurrency on Solana - SPL token

* Creating a token is making the structure vs minting getting a new copy of that token.

### First what is an SPL token?

What are SPL Tokens? Another essential component of the Solana ecosystem is its native SOL token. SOL is the cryptocurrency that runs on the Solana blockchain, and it also acts as a governance token. As such, holders of SOL have the possibility to vote on the future of the blockchain and help govern the network.

### Usage 

1. Please **install [Solana](https://docs.solana.com/cli/install-solana-cli-tools)**

2. Run `solana-keygen new` in the terminal. You will get a **seed phrase and a public key**. P.S. If you have one but want a new one `solana-keygen new --force`.

* If you want **to see your public key again run `solana-keygen pubkey`,**

* In order to **see your balance in the devnet `solana balance --url devnet`.**

* To confirm head to [Solana's Block Explorer](https://explorer.solana.com/) click the button on the side to switch to devnet and paste your public key.

4. **To airdrop sol** type `solana aidrop 2 publicKey --url devnet ` in terminal `solana airdrop 2 CJcNuWuFNthB6rUwvHEHatS3YvnNKQYZPEvQrySc2oQC --url devnet `

---

# SPL token program

5. **To create an SPL token** run `spl-token create-token --url devnet` in the terminal. This will result in an address and signiture:

  ```shell
  Creating token 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb

  Signature: 2u1sZ6oCX799GuHd2BoS1t9gPMaeSJPTLnv9qxSBZhLAnWMTTYvAcB8WwLT122DBHvJtyKDvcLpRggsjsQCED3VR
  ```
6. Time to **mint** run `spl-token create-account <tokenAddress> --url devnet` in the terminal an example of this command is `spl-token create-account 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb --url devnet`. The output will look like the one below:


   ```shell
   Creating account F74xUewaqap6ohBWqL3x65WpdXxQDb7j6PUdqNX2LRJL

   Signature: 5HujwxTL5JWVKYRq1qiyjCZea5AjZyx7BAZqLo7CqZ8yN9npBoRLSegiZab21n5CS1scAjcgaUYHcZeDNW3NrwVV
   ```

 This is the address of an empty token account in our wallet and a signiture.

 7. **Check the balance of the token in our account.** Place `spl-token balance <tokenAddress> --url devnet` in terminal which is for example `spl-token balance 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb --url devnet`. This will return `0`.

 8. In order to increase the balance you need to mint. Run `spl-token mint <tokenAddress> <numberToMint> --url devnet`for example minting 1000  `spl-token mint 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb 1000 --url devnet`. See the output below:

    ```shell
    Minting 1000 tokens
     Token: 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb
     Recipient: F74xUewaqap6ohBWqL3x65WpdXxQDb7j6PUdqNX2LRJL

    Signature: JWmvQFitFcbfSUU89J8jRbR8DkesZh92pJcu9wyW3PRmznW22LqFS3pad6JoUq8HXzyJkmVxY39iG3jiGzVBJin
    ````

9. In order to confirm this I would **reccommend checking your balance a second time.** By running `spl-token balance <tokenAddress> --url devnet` in terminal which is for example `spl-token balance 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb --url devnet`. The output should be `1000`.

10. **Checking the supply of your token** is as follows: `spl-token supply <tokenAddress> --url devnet` in terminal which is for example `spl-token supply 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb --url devnet`. This will return `1000` which makes sense because `1000` was minted in step 8.

11.  In order to **limit the supply** run the following command `spl-token authorize <tokenAddress> mint --disable --url devnet` for example `spl-token authorize 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb mint --disable --url devnet`. This will return the following output:

   ```shell
   Updating 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb
   Current mint authority: CJcNuWuFNthB6rUwvHEHatS3YvnNKQYZPEvQrySc2oQC
   New mint authority: disabled

   Signature: Sjc2QqJx89R5gDK2STJTG5GqMppgVwXSknX5NjrEP3vXL4sNhD6tBza3WYb1XJTGyP6jJoyJykAcWeYo1VSe9Ts
   ```
This renounces our ability to mint new tokens.

* Test this out by attempting to mint a new token by running step 8 again. This will return this output with an error message confirming this did not work: 

    ```shell
    Minting 1000 tokens
    Token: 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb
    Recipient: F74xUewaqap6ohBWqL3x65WpdXxQDb7j6PUdqNX2LRJL
    RPC response error -32002: Transaction simulation failed: Error processing Instruction 0: custom program error: 0x5 [5 log messages]
    ```

12. In order to **burn your own tokens** run `spl-token burn <addressOfTokenAccount> <numberOfTokensToBurn> --url devnet` for example `spl-token burn F74xUewaqap6ohBWqL3x65WpdXxQDb7j6PUdqNX2LRJL 100 --url devnet`. The output should look like such:

   ```shell 
   Burn 100 tokens
   Source: F74xUewaqap6ohBWqL3x65WpdXxQDb7j6PUdqNX2LRJL

   Signature: se2hQhFuT1kPNudTj3YtVo5aTD62gKvDRWVVyvmCEq7dMrsVWLK3c4QALdYUeoE6ic6TJ6B9cDDPHurkwWMa8M3
   ```
* In order to confirm this run step 7 again to check the balance. The output should be `900`.   
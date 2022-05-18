# Create a cryptocurrency on Solana 

* Creating a token is making the structure vs minting getting a new copy of that token.

### Usage 

1. Please install [Solana](https://docs.solana.com/cli/install-solana-cli-tools)

2. Run `solana-keygen new` in the terminal. You will get a seed phrase and a public key. P.S. If you have one but want a new one `solana-keygen new --force`.

* If you want to see your public key again run `solana-keygen pubkey`

* In order to see your balance in the devnet `solana balance --url devnet` 

To confirm head to [Solana's Block Explorer](https://explorer.solana.com/) click the button on the side to switch to devnet and paste your public key.

4. Type `solana aidrop 2 publicKey --url devnet ` in terminal `solana airdrop 2 CJcNuWuFNthB6rUwvHEHatS3YvnNKQYZPEvQrySc2oQC --url devnet `

---

# SPL token program

5. Run `spl-token create-token --url devnet` in the terminal. This will result in an address and signiture:

  ```shell
  Creating token 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb

  Signature: 2u1sZ6oCX799GuHd2BoS1t9gPMaeSJPTLnv9qxSBZhLAnWMTTYvAcB8WwLT122DBHvJtyKDvcLpRggsjsQCED3VR
  ```
6. Time to mint `spl-token create-account <tokenAddress> --url devnet` for example  `spl-token create-account 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb --url devnet`. Output:


   ```shell
   Creating account F74xUewaqap6ohBWqL3x65WpdXxQDb7j6PUdqNX2LRJL

   Signature: 5HujwxTL5JWVKYRq1qiyjCZea5AjZyx7BAZqLo7CqZ8yN9npBoRLSegiZab21n5CS1scAjcgaUYHcZeDNW3NrwVV
   ```

 This is the address of an empty token account in our wallet and a signiture.

 7. Check the balance of the token in our account. Place `spl-token balance <tokenAddress> --url devnet` in terminal which is for example `spl-token balance 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb --url devnet`. This will return `0`.

 8. In order to increase the balance you need to mint. Run `spl-token mint <tokenAddress> <numberToMint> --url devnet`for example minting 1000  `spl-token mint 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb 1000 --url devnet`. See the output below:

    ```shell
    Minting 1000 tokens
     Token: 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb
     Recipient: F74xUewaqap6ohBWqL3x65WpdXxQDb7j6PUdqNX2LRJL

     Signature: JWmvQFitFcbfSUU89J8jRbR8DkesZh92pJcu9wyW3PRmznW22LqFS3pad6JoUq8HXzyJkmVxY39iG3jiGzVBJin
     ````
9. In order to confirm this I would reccommend checking your balance a second time. By running `spl-token balance <tokenAddress> --url devnet` in terminal which is for example `spl-token balance 58Y2BGgrgF2zTGWyjGagoKpMHwBZbe9d8xvduoaoSjwb --url devnet`. The output should be `1000`.
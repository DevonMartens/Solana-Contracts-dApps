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
6. Time to mint
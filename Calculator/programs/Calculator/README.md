# Calculator

* **Solana Programs are stateless** meaning you don't actually store data on them 

* Data is store in **accounts**.

Make sure you have [Rust] and [Anchor](https://book.anchor-lang.com/getting_started/installation.html) installed. Then run `anchor init projectName`.

* To **build the project** run `anchor build`

## Lets Learn about this:

1. Take notice of the new `target` folder and the `idl`

IDL - Interface Description Language 

  ```
  Anchor is a framework for Solana's Sealevel (opens new window)runtime providing several convenient developer tools. Rust crates and eDSL for writing Solana programs. IDL (opens new window)specification. TypeScript package for generating clients from IDL. CLI and workspace management for developing complete applications.
  ```

  ```
  Think of it as instructions from the contract. This is useful for tests and front-end integration.
  ```

2. Open up your `tests` folder:

* A test exists written in Javascript.

To run it run `anchor test`
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod calculator {
    use super::*;
    /*pub = public fun = function - client and program can call
    * first argument is context
    * context = the accounts(like a file where we store data) that we get data from
    * this is because contracts are stateless
    * String- init_message: a message everytime we boot up the calculator
    * The return type is ProgramResult (easy way to serve function results and errors)
    */
    /*
    1. The first line lets us get the calculator account from our context.
    2. We specify &mut because we modify that account here.
    3. On the third line we modify that account.
    4. We use Ok to let solana know the function was run successfully
    */
    pub fun(ctx: Context<Create>, init_message: String) -> ProgramResult
    let calculator = &mut ctx.accounts.calculator;
    calculator.greeting = init_message;
    Ok(())
}
// user defined function called initalize and when it is called the program exits
//most basic anchor program


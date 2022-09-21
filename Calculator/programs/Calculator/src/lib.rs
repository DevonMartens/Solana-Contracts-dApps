use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;

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
    pub fn create(ctx: Context<Create>, init_message: String) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }
}

// user defined function called initalize and when it is called the program exits
//most basic anchor program
/*
Use derive accounts Macro to indicate that this is a context
* inside the {} we have info on the accounts we want to use
* need an account for calculator - we create that using the init macro 
INIT means - create new (create a new calculator)
payer=user mean that the payment in sol will come from the user of the dApp
SPACE argument means the amount of space to be allocated on chain for the calculator account
*/
#[derive(Accounts)]
//create function

pub struct Create<'info> {
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,
    //makes user arg mutable 
    #[account(mut)]
    //user is signer b/c to create account they must sign tx
    pub user: Signer<'info>,
    //system specifications of solana blockchain
    pub system_program: Program<'info, System>,
}
//track 3 things greeting(init_message) msg mathmatical op result and the remainder when dividing
#[account]
pub struct Calculator{
    pub greeting: String,
    pub result: i64,
    pub remainder: i64
}

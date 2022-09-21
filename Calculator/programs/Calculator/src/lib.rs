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
/**  
================================================
|                Math Operations               |
================================================
**/
    /* 
    addition function
    * we do not use the create function because we need a different account for this
    * arguments are numbers for adding ...num1 & num2
    */
    pub fn add(ctx: Context<Addition>, num1: i64, num2: i64) -> ProgramResult {
        //refer to calc account because we do have a result field
        //save the result of the sum
        let calculator = &mut ctx.accounts.calculator;
        //calc result is num1 plus num2
        calculator.result = num1 + num2;
        Ok(())
    }
   /* 
    Subtraction function
    @Params: ...num1 & num2 (subtract num2 from num1)
    */
    pub fn sub(ctx: Context<Sub>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 - num2;
        Ok(())
    }
    /* 
    Division function
    @Params: ...num1 & num2 (divide num2 from num1)
    */
    pub fn div(ctx: Context<Div>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 / num2;
        Ok(())
    }
    /* 
    Multiplication function
    @Params: ...num1 & num2 (multiply num1 by num2)
    */
    pub fn mul(ctx: Context<Mul>, num1: i64, num2: i64) -> ProgramResult {
        let calculator = &mut ctx.accounts.calculator;
        calculator.result = num1 * num2;
        Ok(())
    }
}

/*
Ref: Deleted boilerplate: user defined function called initalize and when it is called the program exits- most basic anchor program
*/
/*

**Using Accounts**

Use derive accounts Macro to indicate that this is a context
* inside the {} we have info on the accounts we want to use
* need an account for calculator - we create that using the init macro 
INIT means - create new (create a new calculator)
payer=user mean that the payment in sol will come from the user of the dApp
SPACE argument means the amount of space to be allocated on chain for the calculator account
*/
/**  
================================================
|             Create Calculator Account        |
================================================
**/
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
/**  
================================================
|      Math Operations Accounts                |
================================================
**/

/*
dev: defines the context for our addition functions
*/
#[derive(Accounts)]
/*not specify spaace beacuse we are not creating new account
we are chaning an old one but we specify mut b/c we are adjusting the calculator*/
pub struct Addition<'info> {
    //allows us to be able to change this account
    #[account(mut)]
    pub calculator: Account<'info, Calculator>
}
/*
dev: defines the context for our subtraction function
*/
#[derive(Accounts)]
pub struct Sub<'info> {
    //allows us to be able to change this account
    #[account(mut)]
    pub calculator: Account<'info, Calculator>
}
/*
dev: defines the context for the division function
*/
#[derive(Accounts)]
pub struct Div<'info> {
    //allows us to be able to change this account
    #[account(mut)]
    pub calculator: Account<'info, Calculator>
}
/*
dev: defines the context for the multiplication function
*/
#[derive(Accounts)]
pub struct Mul<'info> {
    //allows us to be able to change this account
    #[account(mut)]
    pub calculator: Account<'info, Calculator>
}



//track 3 things greeting(init_message) msg mathmatical op result and the remainder when dividing
#[account]
pub struct Calculator{
    pub greeting: String,
    pub result: i64,
    pub remainder: i64
}

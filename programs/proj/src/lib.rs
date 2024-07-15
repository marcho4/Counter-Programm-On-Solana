use anchor_lang::prelude::*;

declare_id!("BjRSHo9aArH2X9WGAz49BLjEEQq9oM1uobsF54EVJkXA");

#[program]
pub mod proj {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &ctx.accounts.counter;
        msg!("Counter account created! Current count {}", counter.count);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        msg!("Previous counter {}", counter.count);

        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Increased counter to {}", counter.count);
        Ok(())
    }
}

// аккаунты, необходимые для инструкции Initialize
#[derive(Accounts)]
pub struct Initialize<'info> {
    // аккаунт, который будет платить комиссию
    #[account(mut)]
    pub user: Signer<'info>, // указываем, что аккаунт подписывает транзакцию

    // создаем аккаунт счётчика и инициализируем его
    #[account(
        init, // показывает, что мы создаем этот аккаунт
        payer = user, // указываем, что оплачивает комиссию за создание аккаунта user 
        space = 8 + 8 // указываем кол-во памяти для аккаунта (8 для дискриминатора и 8 для u64 счетчика)
    )]
    pub counter: Account<'info, Counter>, // указываем, что аккаунт типа Счётчик
    pub system_program: Program<'info, System>, // we need this to create new accounts
}


// аккаунты, необходимые для инструкции Increment
#[derive(Accounts)]
pub struct Increment<'info>  {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// structure of Counter account
#[account]
pub struct Counter {
    pub count: u64,
}
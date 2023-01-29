
use anchor_lang::prelude::*;

declare_id!("23vjhFWh1dfDNzcsKZCdXt5XQdJSFipZaBncVpGrZmGw");

#[program]
pub mod robo_swap_program {
    use super::*;

    pub fn delete(
        _ctx: Context<Delete>
    ) -> Result<()> {
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        for idx in 0..game::ROBOTS {
            let robot = &mut ctx.accounts.pda.robots[idx];
            *robot = game::Robots::new(ctx.accounts.user.key(), idx as u8)?;
        }
        ctx.accounts.pda.bump = *ctx.bumps.get("pda").unwrap_or_else(|| panic!("hundt"));
        Ok(())
    }

    pub fn steal(ctx: Context<Steal>, robber_idx: u8, victim_idx: u8) -> Result<()> {
        
        require!(robber_idx <= 25, game::RoboSwapError::IndexOutOfBounds);
        require!(victim_idx <= 25, game::RoboSwapError::IndexOutOfBounds);


        if ctx.accounts.robber_pda.key() == ctx.accounts.victim_pda.key() {

            let v = &mut ctx.accounts.victim_pda;
    
            let helper = v.robots[robber_idx as usize].owner;
            v.robots[robber_idx as usize].owner = v.robots[victim_idx as usize].owner;
            v.robots[victim_idx as usize].owner = helper;
    
            let helper = v.robots[robber_idx as usize].idx;
            v.robots[robber_idx as usize].idx = v.robots[victim_idx as usize].idx;
            v.robots[victim_idx as usize].idx = helper;

            let helper = v.robots[robber_idx as usize].stolen;
            v.robots[robber_idx as usize].stolen = v.robots[victim_idx as usize].stolen;
            v.robots[victim_idx as usize].stolen = helper;

            if v.robots[robber_idx as usize].stolen != u32::MAX {
                v.robots[robber_idx as usize].stolen += 1;
            }
    
        }
        else {

            let r = &mut ctx.accounts.robber_pda.robots[robber_idx as usize];
            let v = &mut ctx.accounts.victim_pda.robots[victim_idx as usize];
    
            let helper = r.owner;
            r.owner = v.owner;
            v.owner = helper;
    
            let helper = r.idx;
            r.idx = v.idx;
            v.idx = helper;
    
            let helper = r.stolen;
            r.stolen = v.stolen;
            v.stolen = helper;
    
            if r.stolen != u32::MAX {
                r.stolen += 1;
            }
    
        }


        Ok(())
    }

}

#[derive(Accounts)]
pub struct Delete<'info> {
    pub system_program: Program<'info, System>,
    
    /// CHECK: This is not dangerous
    #[account(
        mut,
        seeds = [b"RoboSwap", receiver.key().as_ref()], bump,
        close = receiver,
    )]
    pub pda: Box<Account<'info, game::Game>>,

    #[account(
        mut,
    )]
    pub receiver: SystemAccount<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub system_program: Program<'info, System>,
    
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init, payer = user, space = 8 + game::GAME_SIZE, 
        seeds = [b"RoboSwap", user.key().as_ref()], bump,
    )]  
    pub pda: Box<Account<'info, game::Game>>,
}

#[derive(Accounts)]
pub struct Steal<'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub robber: Signer<'info>,
    #[account(
        mut,
        seeds = [b"RoboSwap", robber.key().as_ref()], bump,
    )]  
    pub robber_pda: Box<Account<'info, game::Game>>,

    #[account(mut)]
    pub victim: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"RoboSwap", victim.key().as_ref()], bump,
    )]  
    pub victim_pda: Box<Account<'info, game::Game>>,
}

#[account]
pub struct NewAccount(u8);

mod game {
    use anchor_lang::prelude::*;
    use anchor_lang::{prelude::Pubkey, account};

    pub const ROBOTS: usize = 26;
    pub const ROBOT_SIZE: usize = 70;
    pub const GAME_SIZE: usize = ROBOT_SIZE * ROBOTS + 1;

    #[account]
    pub struct Game {
        pub robots: [Robots; ROBOTS],
        pub bump: u8,
    }
    impl Game {}

    #[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, PartialEq, Eq)]
    pub struct Robots {
        pub wallet: Pubkey,
        pub owner: Pubkey,
        pub idx: u8,
        pub stolen: u32,
    }
    impl Robots {
        pub fn new(wallet: Pubkey, idx: u8) -> Result<Self> {
            require!(idx <= 25, RoboSwapError::IndexOutOfBounds);
            Ok(Self {
                owner: wallet.clone(),
                wallet,
                idx,
                stolen: 0,
            })
        }
    }

    #[error_code]
    pub enum RoboSwapError {
        IndexOutOfBounds,
        UserAndAccountNotEqual,
    }
}
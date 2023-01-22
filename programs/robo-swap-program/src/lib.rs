use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod robo_swap_program {
    use super::*;

    pub fn initialize(ctx: Context<Game>, wallet: Pubkey) -> Result<()> {
        for idx in 0..game::ROBOTS {
            let robot = &mut ctx.accounts.game.robots[idx];
            *robot = game::Robot::new(wallet, idx as u8);
        }
        ctx.accounts.game.bump = *ctx.bumps.get("game").unwrap_or_else(|| panic!("hundt"));
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Game<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, payer = user, space = 8 + game::GAME_SIZE, 
        seeds = [b"RoboSwap", user.key().as_ref()], bump,
    )]
    pub game: Box<Account<'info, game::Game>>,
    pub system_program: Program<'info, System>,
}

mod game {
    use anchor_lang::prelude::*;
    use anchor_lang::{prelude::Pubkey, account};

    pub const ROBOTS: usize = 26;
    pub const ROBOT_SIZE: usize = 70;
    pub const GAME_SIZE: usize = ROBOT_SIZE * ROBOTS + 1;

    #[account]
    pub struct Game {
        pub robots: [Robot; ROBOTS],
        pub bump: u8,
    }
    impl Game {}

    #[derive(AnchorSerialize, AnchorDeserialize, Default, Clone, Copy, PartialEq, Eq)]
    pub struct Robot {
        wallet: Pubkey,
        owner: Pubkey,
        idx: u8,
        owner_idx: u8,
        swaps: u32,
    }
    impl Robot {
        pub fn new(wallet: Pubkey, idx: u8) -> Self {
            Self {
                owner: wallet.clone(),
                wallet,
                idx,
                owner_idx: idx,
                swaps: 0,
            }
        }
    }

}
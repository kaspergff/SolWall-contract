use anchor_lang::prelude::*;

declare_id!("GQYX5FugE2PJtb9vjQScDu5n1RJLQQz1pSdY2Kp1xfaV");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    base_account.total_gifs = 0;
    Ok(())
  }

  // The function now accepts a gif_link param from the user. We also reference the user from the Context
  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

	// Build the struct.
    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
      gif_votes: 0,
    };

	// Add it to the gif_list vector.
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }
}

pub fn update_item(ctx: Context<UpdateItem>, item_index: String) -> ProgramResult {
  let base_account = &mut ctx.accounts.base_account;
  let item_index_as_num: usize = item_index.parse().unwrap();
  let item_to_update = &mut base_account.gif_list[item_index_as_num as usize];
  item_to_update.gif_votes += 1;
  Ok(())
}

#[derive(Accounts)]
pub struct UpdateItem<'info>{
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
}


#[derive(Accounts)]
pub struct StartStuffOff<'info> {
  #[account(init, payer = user, space = 9000)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
  pub system_program: Program <'info, System>,
}

// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub gif_votes: i32,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
	// Attach a Vector of type ItemStruct to the account.
    pub gif_list: Vec<ItemStruct>,
}
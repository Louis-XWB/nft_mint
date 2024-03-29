
use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_spl::token;
use anchor_spl::token::{MintTo, Token};
use mpl_token_metadata::instruction::{create_master_edition_v3, create_metadata_accounts_v2};

declare_id!("3bfaUxYjL8PhJbCiw9rxjaijdgyUs8cJNGSufPuaPuKu");

#[program]
pub mod metaplex_anchor_nft {
    use super::*;

    pub fn mint_nft(
        ctx: Context<MintNFT>,
        creator_key: Pubkey,
        uri: String,
        title: String,
    ) -> Result<()> {
        msg!("Initializing Mint Ticket");
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };
        msg!("CPI Accounts Assigned");
        let cpi_program = ctx.accounts.token_program.to_account_info();
        msg!("CPI Program Assigned");
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        msg!("CPI Context Assigned");
        token::mint_to(cpi_ctx, 1)?;
        msg!("Token Minted !!!");
        let account_info = vec![
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Account Info Assigned");
        let creator = vec![
            mpl_token_metadata::state::Creator {
                address: creator_key,
                verified: false,
                share: 100,
            },
            mpl_token_metadata::state::Creator {
                address: ctx.accounts.mint_authority.key(),
                verified: false,
                share: 0,
            },
        ];
        msg!("Creator Assigned");
        let symbol = std::string::ToString::to_string("symb");
        invoke(
            &create_metadata_accounts_v2(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.payer.key(),
                title,
                symbol,
                uri,
                Some(creator),
                1,
                true,
                false,
                None,
                None,
            ),
            account_info.as_slice(),
        )?;
        msg!("Metadata Account Created !!!");
        let master_edition_infos = vec![
            ctx.accounts.master_edition.to_account_info(),
            ctx.accounts.mint.to_account_info(),
            ctx.accounts.mint_authority.to_account_info(),
            ctx.accounts.payer.to_account_info(),
            ctx.accounts.metadata.to_account_info(),
            ctx.accounts.token_metadata_program.to_account_info(),
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
            ctx.accounts.rent.to_account_info(),
        ];
        msg!("Master Edition Account Infos Assigned");
        invoke(
            &create_master_edition_v3(
                ctx.accounts.token_metadata_program.key(),
                ctx.accounts.master_edition.key(),
                ctx.accounts.mint.key(),
                ctx.accounts.payer.key(),
                ctx.accounts.mint_authority.key(),
                ctx.accounts.metadata.key(),
                ctx.accounts.payer.key(),
                Some(0),
            ),
            master_edition_infos.as_slice(),
        )?;
        msg!("Master Edition Nft Minted !!!");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub mint_authority: Signer<'info>,

    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub mint: UncheckedAccount<'info>,
    // #[account(mut)]
    pub token_program: Program<'info, Token>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub metadata: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub token_account: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub token_metadata_program: UncheckedAccount<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub payer: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub rent: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub master_edition: UncheckedAccount<'info>,
}

// use anchor_lang::prelude::*;
// use anchor_spl::token;
// use anchor_spl::{
//     metadata::{
//         create_master_edition_v3, create_metadata_accounts_v3, CreateMasterEditionV3,
//         CreateMetadataAccountsV3,
//     },
//     token::{MintTo, Token},
// };
// use mpl_token_metadata::types::DataV2;

// declare_id!("3bfaUxYjL8PhJbCiw9rxjaijdgyUs8cJNGSufPuaPuKu");

// #[program]
// pub mod metaplex_anchor_nft {
//     use super::*;

//     pub fn mint_nft(
//         ctx: Context<MintNFT>,
//         creator_key: Pubkey,
//         uri: String,
//         title: String,
//     ) -> Result<()> {
//         msg!("Initializing Mint Ticket");
//         let cpi_accounts = MintTo {
//             mint: ctx.accounts.mint.to_account_info(),
//             to: ctx.accounts.token_account.to_account_info(),
//             authority: ctx.accounts.payer.to_account_info(),
//         };
//         msg!("CPI Accounts Assigned");
//         let cpi_program = ctx.accounts.token_program.to_account_info();
//         msg!("CPI Program Assigned");
//         let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
//         msg!("CPI Context Assigned");
//         token::mint_to(cpi_ctx, 1)?;
//         msg!("Token Minted !!!");
    
//         msg!("Account Info Assigned");
//         let creator = vec![
//             mpl_token_metadata::types::Creator {
//                 address: creator_key,
//                 verified: false,
//                 share: 100,
//             },
//             mpl_token_metadata::types::Creator {
//                 address: ctx.accounts.mint_authority.key(),
//                 verified: false,
//                 share: 0,
//             },
//         ];
//         msg!("Creator Assigned");
//         let symbol = std::string::ToString::to_string("symb");
        
//         let cpi_context = CpiContext::new(
//             ctx.accounts.token_metadata_program.to_account_info(),
//             CreateMetadataAccountsV3 {
//                 metadata: ctx.accounts.metadata.to_account_info(),
//                 mint: ctx.accounts.mint.to_account_info(),
//                 mint_authority: ctx.accounts.mint_authority.to_account_info(),
//                 update_authority: ctx.accounts.mint_authority.to_account_info(),
//                 payer: ctx.accounts.payer.to_account_info(),
//                 system_program: ctx.accounts.system_program.to_account_info(),
//                 rent: ctx.accounts.rent.to_account_info(),
//             },
//         );

//         let data_v2 = DataV2 {
//             name: title,
//             symbol: symbol,
//             uri: uri,
//             seller_fee_basis_points: 0,
//             creators: Some(creator),
//             collection: None,
//             uses: None,
//         };
//         create_metadata_accounts_v3(cpi_context, data_v2, false, true, None)?;

//         msg!("Metadata Account Created !!!");

//         //create master edition account
//         let cpi_context = CpiContext::new(
//             ctx.accounts.token_metadata_program.to_account_info(),
//             CreateMasterEditionV3 {
//                 edition: ctx.accounts.master_edition.to_account_info(),
//                 mint: ctx.accounts.mint.to_account_info(),
//                 update_authority: ctx.accounts.mint_authority.to_account_info(),
//                 mint_authority: ctx.accounts.mint_authority.to_account_info(),
//                 payer: ctx.accounts.payer.to_account_info(),
//                 metadata: ctx.accounts.metadata.to_account_info(),
//                 token_program: ctx.accounts.token_program.to_account_info(),
//                 system_program: ctx.accounts.system_program.to_account_info(),
//                 rent: ctx.accounts.rent.to_account_info(),
//             },
//         );

//         create_master_edition_v3(cpi_context, None)?;

//         msg!("Master Edition Account Infos Assigned");


//         msg!("Master Edition Nft Minted !!!");

//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct MintNFT<'info> {
//     #[account(mut)]
//     pub mint_authority: Signer<'info>,

//     /// CHECK: This is not dangerous because we don't read or write from this account
//     #[account(mut)]
//     pub mint: UncheckedAccount<'info>,
//     // #[account(mut)]
//     pub token_program: Program<'info, Token>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     #[account(mut)]
//     pub metadata: UncheckedAccount<'info>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     #[account(mut)]
//     pub token_account: UncheckedAccount<'info>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     pub token_metadata_program: UncheckedAccount<'info>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     #[account(mut)]
//     pub payer: AccountInfo<'info>,
//     pub system_program: Program<'info, System>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     pub rent: AccountInfo<'info>,
//     /// CHECK: This is not dangerous because we don't read or write from this account
//     #[account(mut)]
//     pub master_edition: UncheckedAccount<'info>,
// }

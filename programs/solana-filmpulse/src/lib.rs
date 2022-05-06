use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer, MintTo};

declare_id!("3gqTAW1iCFa8GuFZ9SdpmTVb1a4JzfXHXyBfhmMS2Z7X");

#[program]
pub mod solana_filmpulse {
    use super::*;
    pub fn post_review(ctx: Context<PostReview>, title: String, essay: String, rating: i32, author_keys: Vec<Pubkey>) -> ProgramResult {
        let review: &mut Account<Review> = &mut ctx.accounts.review;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();
        
        if title.chars().count() < 1 {
            return Err(ErrorCode::TitleRequired.into())
        }

        if title.chars().count() > 50 {
            return Err(ErrorCode::TitleTooLong.into())
        }

        if essay.chars().count() > 280 {
            return Err(ErrorCode::ReviewTooLong.into())
        }

        review.author = *author.key;
        review.timestamp = clock.unix_timestamp;
        review.title = title;
        review.essay = essay;
        review.rating = rating;

        // let authors: usize = author_keys.len();
        // let tokenDistribution: usize = 1/authors;

        // for address in author_keys {
        //     token::transfer (address: Pubkey, tokenDistribution: usize)?;
        // }

        Ok(())
    }

    pub fn verify_review(ctx: Context<VerifyReview>, review_key: Pubkey, author_address: Pubkey, verifier_keys: Vec<String>) -> ProgramResult {
        let verify: &mut Account<Verify> = &mut ctx.accounts.verify;
        let author: &Signer = &ctx.accounts.author;
        let clock: Clock = Clock::get().unwrap();

        verify.author = *author.key;
        verify.timestamp = clock.unix_timestamp;
        verify.review_key = review_key;

        // let totalSupply: u64 = 100000000;
        // let currentSupply: u64 = 21000420;
        // let notMinted: u64 = totalSupply - currentSupply;
        // let mintAmount: u64 = notMinted/totalSupply;
        // token::mint_to(author_address: Pubkey, mintAmount: u64)?;

        // let tokenDistribution: u64 = 1/verifier_keys.len();
        // for address in verifier_keys {
        //     token::transfer(address: Pubkey, tokenDistribution: u64)?;
        // }

        Ok(())
    }

    pub fn transfer_wrapper(ctx: Context<TransferWrapper>, amount: u64) -> ProgramResult {
        msg!("starting tokens: {}", ctx.accounts.sender_token.amount);
        token::transfer(ctx.accounts.transfer_ctx(), amount)?;
        ctx.accounts.sender_token.reload()?;
        msg!("remaining tokens: {}", ctx.accounts.sender_token.amount);
        Ok(())
    }

    pub fn delete_review(_ctx: Context<DeleteReview>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PostReview<'info> {
    #[account(init, payer = author, space = Review::LEN)]
    pub review: Account<'info, Review>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyReview<'info> {
    #[account(init, payer = author, space = Verify::LEN)]
    pub verify: Account<'info, Verify>,
    #[account(mut)]
    pub author: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferWrapper<'info> {
    pub sender: Signer<'info>,
    #[account(mut)]
    pub sender_token: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_token: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct DeleteReview<'info> {
    #[account(mut, has_one = author, close = author)]
    pub review: Account<'info, Review>,
    pub author: Signer<'info>,
}

#[account]
pub struct Review {
    pub author: Pubkey,
    pub timestamp: i64,
    pub title: String,
    pub essay: String,
    pub rating: i32,
}

#[account]
pub struct Verify {
    pub author: Pubkey,
    pub timestamp: i64,
    pub review_key: Pubkey,
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4; // Stores the size of the string.
const MAX_TOPIC_LENGTH: usize = 50 * 4; // 50 chars max.
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max.
const REVIEW_LENGTH: usize = 32;

impl Review {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // Author.
        + TIMESTAMP_LENGTH // Timestamp.
        + STRING_LENGTH_PREFIX + MAX_TOPIC_LENGTH // Topic.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH
        + REVIEW_LENGTH; // Content.
}

impl Verify {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // TweetKey.
        + TIMESTAMP_LENGTH // Timestamp.
        + PUBLIC_KEY_LENGTH; // Verifier.
        
}

impl<'info> TransferWrapper<'info> {
    fn transfer_ctx(&self) -> CpiContext<'_, '_, '_, 'info, Transfer<'info>> {
        CpiContext::new(
            self.token_program.to_account_info(),
            Transfer {
                from: self.sender_token.to_account_info(),
                to: self.receiver_token.to_account_info(),
                authority: self.sender.to_account_info(),
            },
        )
    }
}

#[error]
pub enum ErrorCode {
    #[msg("The provided title should be 50 characters long maximum.")]
    TitleTooLong,
    #[msg("The provided review should be 280 characters long maximum.")]
    ReviewTooLong,
    #[msg("Topic Required.")]
    TitleRequired,
}
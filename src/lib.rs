use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod token_metadata;

use instructions::*;

declare_id!("8Hs2MzJAuWXt57LKcTsYQaRCZyNUeuyuGHrfAzYQSLDv");

#[program]
pub mod cnft_program {
  use super::*;

  pub fn initialize_tree(ctx: Context<InitializeTree>, args: InitializeTreeArgs) -> Result<()> {
    initialize_tree::handler(ctx, args)
  }

  pub fn transfer_asset_v1<'info>(
    ctx: Context<'_, '_, '_, 'info, TransferAssetV1<'info>>,
    args: TransferAssetV1Args,
  ) -> Result<()> {
    transfer_asset_v1::handler(ctx, args)
  }

  pub fn burn_asset_v0<'info>(
    ctx: Context<'_, '_, '_, 'info, BurnAssetV0<'info>>,
    args: BurnAssetV0Args,
  ) -> Result<()> {
    burn_asset_v0::handler(ctx, args)
  }

  pub fn mint_compressed_nft(
    ctx: Context<MintCompressedNft>,
    args: MintCompressedNftArgs,
  ) -> Result<()> {
    mint_compressed_nft::handler(ctx, args)
  }

  pub fn initialize_compressed(
    ctx: Context<InitializeCompressedProject>,
    args: InitializeCompressedProjectArgs,
  ) -> Result<()> {
    initialize_compressed_project::handler(ctx, args)
  }

  

  pub fn mint_transferable_nft(
    ctx: Context<MintTransferableNft>,
    args: MintTransferableNftArgs,
  ) -> Result<()> {
    mint_transferable_nft::handler(ctx, args)
  }

  pub fn verify_legacy_nft_collection(
    ctx: Context<VerifyLegacyNftCollection>,
    args: VerifyLegacyNftCollectionArgs,
  ) -> Result<()> {
    verify_legacy_nft_collection::handler(ctx, args)
  }

}

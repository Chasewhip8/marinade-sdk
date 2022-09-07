use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(
    MarinadeInstructionData,
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    BorshSerialize,
    BorshDeserialize,
)]
#[discriminator([101, 41, 206, 33, 216, 111, 25, 78])]
pub struct SetValidatorScoreData {
    pub index: u32,
    pub validator_vote: Pubkey,
    pub score: u32,
}

#[derive(MarinadeInstructionAccounts)]
#[ownerid(crate::ID)]
pub struct SetValidatorScoreAccounts {
    #[account(mut)]
    pub state: Pubkey,
    #[account(signer)]
    pub manager_authority: Pubkey,
    #[account(mut)]
    pub validator_list: Pubkey,
}

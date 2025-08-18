use crate::state::*;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    ProgramResult,
};

pub fn process_initialize(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    if !data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }

    let [signer_info, archive_info, epoch_info, block_info, metadata_info, mint_info, treasury_info, treasury_ata_info, tape_info, writer_info, tape_program_info, system_program_info, token_program_info, associated_token_program_info, metadata_program_info, rent_sysvar_info, slot_hashes_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    check_account(&archive_info, ARCHIVE)?;
    check_account(&epoch_info, EPOCH)?;
    check_account(&block_info, BLOCK)?;
    let (mint_address, mint_bump) = get_pda(GetPda::Mint);
    let (treasury_address, treasury_bump) = get_pda(GetPda::Treasury);
    let (metadata_address, _metadata_bump) = get_pda(GetPda::Metadata(mint_address));

    assert_eq!(mint_bump, MINT_BUMP);
    assert_eq!(treasury_bump, TREASURY_BUMP);

    check_account_with_address(&mint_info, &mint_address)?;
    check_account_with_address(&metadata_info, &metadata_address)?;
    check_account_with_address(&treasury_info, &treasury_address)?;

    if !treasury_ata_info.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    if !treasury_ata_info.is_writable() {
        return Err(ProgramError::Immutable);
    }

    is_program_check(&tape_program_info)?;
    is_program_check(&system_program_info)?;
    is_program_check(&token_program_info)?;
    is_program_check(&associated_token_program_info)?;
    is_program_check(&metadata_program_info)?;
    is_program_check(&rent_sysvar_info)?;
    is_program_check(&slot_hashes_info)?;

    Ok(())
}

fn check_account(account: &AccountInfo, seed: &[u8]) -> Result<(), ProgramError> {
    if !account.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    if !account.is_writable() {
        return Err(ProgramError::Immutable);
    }
    let (pda, _bump) = find_program_address(&[seed], &TAPE_ID);

    if account.key() != &pda {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}

fn get_pda(get_pda: GetPda) -> (Pubkey, u8) {
    match get_pda {
        GetPda::Mint => {
            find_program_address(&[b"mint", &[152, 68, 212, 200, 25, 113, 221, 71]], &TAPE_ID)
        }
        GetPda::Treasury => find_program_address(&[b"treasury"], &TAPE_ID),
        GetPda::Metadata(mint) => find_program_address(
            &[b"metadata", MPL_TOKEN_METADATA_ID.as_ref(), mint.as_ref()],
            &MPL_TOKEN_METADATA_ID,
        ),
    }
}

enum GetPda {
    Metadata(Pubkey),
    Mint,
    Treasury,
}

fn check_account_with_address(account: &AccountInfo, address: &Pubkey) -> Result<(), ProgramError> {
    if !account.data_is_empty() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }
    if !account.is_writable() {
        return Err(ProgramError::Immutable);
    }

    if account.key().ne(address) {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(())
}

fn is_program_check(account: &AccountInfo) -> Result<(), ProgramError> {
    if account.key().ne(&TAPE_ID) {
        return Err(ProgramError::InvalidAccountData);
    }

    if !account.executable() {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(())
}

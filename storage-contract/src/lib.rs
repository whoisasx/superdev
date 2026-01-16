use solana_program::{
  account_info::AccountInfo,
  entrypoint,
  entrypoint::ProgramResult,
  program::invoke,
  program_error::ProgramError,
  pubkey::Pubkey,
  rent::Rent,
  system_instruction,
  sysvar::Sysvar,
};
use borsh::{BorshSerialize,BorshDeserialize};

entrypoint!(process_instruction);

#[derive(BorshSerialize,BorshDeserialize)]
pub struct NameAccount{
  name:String
}

#[derive(BorshSerialize,BorshDeserialize)]
pub enum NameInstruction{
  Initialize(String),
  Update(String)
}

pub fn process_instruction(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  instruction_data: &[u8]
)->ProgramResult{
  let instruction = NameInstruction::try_from_slice(instruction_data)
    .map_err(|_| ProgramError::InvalidInstructionData)?;

  match instruction {
    NameInstruction::Initialize(initial_name) => {
      process_initialize_name(program_id,accounts,initial_name)?;
    },
    NameInstruction::Update(updated_name)=>{
      process_update_name(program_id,accounts,updated_name)?;
    }
  }

  Ok(())
}

fn process_initialize_name(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  initial_name: String
)->ProgramResult{
  let [payer_account,name_account,system_program]=accounts else {
    return Err(ProgramError::NotEnoughAccountKeys);
  };

  let account_space=50;
  let rent=Rent::get()?;
  let req_lamports=rent.minimum_balance(account_space);

  let create_ix=system_instruction::create_account(
    payer_account.key,
    name_account.key,
    req_lamports,
    account_space as u64,
    program_id
  );

  invoke(
    &create_ix,
    &[
      payer_account.clone(),
      name_account.clone(),
      system_program.clone(),
    ],
  )?;

  let name_data=NameAccount{
    name:initial_name,
  };

  let mut account_data= &mut name_account.data.borrow_mut()[..];
  name_data.serialize(&mut account_data)?;

  Ok(())
}

fn process_update_name(
  program_id: &Pubkey,
  accounts: &[AccountInfo],
  updated_name:String
)->ProgramResult{
  let [name_account]=accounts else {
    return Err(ProgramError::NotEnoughAccountKeys);
  };
  if name_account.owner!= program_id{
    return Err(ProgramError::IncorrectProgramId);
  }

  let data= name_account.data.borrow();
  let mut account_data=NameAccount::try_from_slice(&data)?;
  account_data.name=updated_name;
  account_data.serialize(&mut *name_account.data.borrow_mut())?;
  
  Ok(())
}
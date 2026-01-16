use litesvm::LiteSVM;
use solana_sdk::{
  instruction::{AccountMeta,Instruction},
  signature::{Keypair,Signer,read_keypair_file},
  transaction::Transaction
};
use solana_sdk_ids::system_program;
use borsh::BorshSerialize;
use storage_contract::NameInstruction;

#[test]
fn test_program_deploy(){
  let mut svm=LiteSVM::new();
  let name_account=Keypair::new();

  let program_keypair=read_keypair_file("./target/deploy/storage_contract-keypair.json").unwrap();
  let program_id=program_keypair.pubkey();
  let program_bytes=include_bytes!("../target/deploy/storage_contract.so");
  svm.add_program(program_id, program_bytes).unwrap();

  let payer=Keypair::new();
  svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

  let mut data_bytes = Vec::new();
  NameInstruction::Initialize(String::from("adil shaikh")).serialize(&mut data_bytes).unwrap();

  let instruction=Instruction{
    program_id,
    accounts: vec![
      AccountMeta::new(payer.pubkey(), true),
      AccountMeta::new(name_account.pubkey(),true),
      AccountMeta::new_readonly(system_program::id(), false),
    ],
    data:data_bytes,
  };
  let tx=Transaction::new_signed_with_payer(
    &[instruction],
    Some(&payer.pubkey()),
    &[&payer,&name_account],
    svm.latest_blockhash()
  );

  let result=svm.send_transaction(tx).unwrap();
  
  let name_account_info=svm.get_account(&name_account.pubkey()).unwrap();
  println!("Data of the name account: {:?}",name_account_info.data);

  println!("program added successfully.");
  println!("Logs: \n{}",result.pretty_logs());
}
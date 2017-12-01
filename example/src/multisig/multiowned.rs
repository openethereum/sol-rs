extern crate ethabi;
#[macro_use]
extern crate ethabi_derive;
#[macro_use]
extern crate ethabi_contract;
extern crate solaris;

use_contract!(multisig, "Wallet", "res/wallet.abi");
use multisig::Wallet;
use std::collections::HashMap;

fn new_multisig_wallet(signers: Vec<Account>, required: u8, daily_limit: usize) 
    -> Wallet
{
    // Must have at least the required number of signers
    assert!(signers.length() >= required as usize);
    Wallet.functions().new(signers.as_slice(), required, daily_limit) 
}

fn multisig_setup() -> (HashMap, Wallet) {
    let mut accounts = HashMap::new();
    accounts.insert("defaultUser", Account::new_basic(10_000.into(), 0.into()));
    accounts.insert("signer1", Account::new_basic(0.into(), 1.into()));
    accounts.insert("signer2", Account::new_basic(0.into(), 2.into()));
    accounts.insert("signer3", Account::new_basic(0.into(), 3.into()));
    accounts.insert("recipient", Account::new_basic(0.into(), 4.into()));
    accounts.insert("otherUser", Account::new_basic(0.into(), 5.into()));
    accounts.insert("otherUser2", Account::new_basic(0.into(), 6.into()));
    accounts.insert("newOwner", Account::new_basic(0.into(), 7.into()));

    let signers = vec![accounts.get(&"signer1"),
                       accounts.get(&"signer2"),
                       accounts.get(&"signer3")];
    let required = 2;
    let daily_limit = 123;

    let w = new_multisig_wallet(signers, required, daily_limit);

    (accounts, w)
}

#[test]
fn it_should_have_inited() {
    let (accounts, wallet) = multisig_setup();
    let mut evm = Evm::default();

    let a: ethabi::Address = [3u8; 20];

    let num_owners = wallet.functions().m_numOwners().call(a, &mut evm).unwrap();
    let req_signers = wallet.functions().m_required().call(a, &mut evm).unwrap();
    let daily_limit = wallet.functions().m_dailyLimit().call(a, &mut evm).unwrap();

    assert_eq!(num_owners, 3);
    assert_eq!(req_signers, 2);
    assert_eq!(daily_limit, 123);
}

#[test]
fn it_allows_signers_to_add_an_owner() {
    let (accounts, wallet) = multisig_setup();
    let mut evm = Evm::default();

    let new_owner = accounts.get(&"newOwner").expect("No newOwner account found");
    let signer1 = accounts.get(&"signer1").expect("No signer1 account found");
    let signer2 = accounts.get(&"signer2").expect("No signer2 account found");

    // Add new owner from first signer
    let add_owner = wallet
        .functions()
        .add_owner()
        .call(new_owner, &mut evm)
        .from(signer1)
        .unwrap();
    // Owner should not be added, requires >= 2 signers 
    assert!(add_owner.events().not_contains("OwnerAdded"));

    let new_owner_is_owner = wallet
        .functions()
        .is_owner()
        .call(new_owner, &mut evm)
        .unwrap(); 
    assert_eq!(new_owner_is_owner, false);

    // Add new owner from second signer
    let add_owner2 = wallet
        .functions()
        .add_owner()
        .call(new_owner, &mut evm)
        .from(signer2)
        .unwrap();
    // Owner should be added 
    assert_eq!(add_owner2.events().contains("OwnerAdded"));

    // Contract should now have a new owner 
    let new_owner_is_owner = wallet
        .functions()
        .is_owner()
        .call(new_owner, &mut evm)
        .unwrap(); 
    assert_eq!(new_owner_is_owner, true);
}

#[test]
fn it_allows_signers_to_remove_an_owner() {
    let (accounts, wallet) = multisig_setup();
    let mut evm = Evm::default();

    let signer1 = accounts.get(&"signer1").expect("No signer1 account found");
    let signer2 = accounts.get(&"signer2").expect("No signer2 account found");
    let signer3 = accounts.get(&"signer3").expect("No signer3 account found");

    // Remove new owner from first signer
    let rm_owner = wallet
        .functions()
        .remove_owner()
        .call(signer3, &mut evm)
        .from(signer1)
        .unwrap();
    // Owner should only be removed after required number of signers have removed them
    assert!(rm_owner.events().not_contains("OwnerRemoved"));

    // Signer3 should still be an owner
    let signer3_is_owner = wallet
        .functions()
        .is_owner()
        .call(signer3, &mut evm)
        .unwrap(); 
    assert_eq!(signer3_is_owner, true);

    // Remove new owner from second signer
    let rm_owner2 = wallet
        .functions()
        .remove_owner()
        .call(signer3, &mut evm)
        .from(signer2)
        .unwrap();
    // Signer3 should now be removed 
    assert!(rm_owner2.events().contains("OwnerRemoved"));

    // Signer3 should no longer be an owner
    let signer3_is_owner = wallet
        .functions()
        .is_owner()
        .call(signer3, &mut evm)
        .unwrap(); 
    assert_eq!(signer3_is_owner, false);
}

#[test]
fn it_does_not_allow_nonsigners_to_add_an_owner() {
    let (accounts, wallet) = multisig_setup();
    let mut evm = Evm::default();

    let other_user = accounts.get(&"otherUser").expect("No otherUser account found");
    let other_user2 = accounts.get(&"otherUser2").expect("No otherUser2 account found");

    // Add owner from first non-signer
    let add_user = wallet
        .functions()
        .add_owner()
        .call(other_user, &mut evm)
        .from(other_user)
        .unwrap();
    assert!(add_user.events().not_contains("OwnerAdded"));
    
    // Add owner from second non-signer
    let add_user2 = wallet
        .functions()
        .add_owner()
        .call(other_user, &mut evm)
        .from(other_user2)
        .unwrap();
    // Non-owners should not be able to remove owners
    assert!(add_user2.events().not_contains("OwnerAdded"));

    // Other user should not be an owner 
    let other_user_is_owner = wallet
        .functions()
        .is_owner()
        .call(other_user, &mut evm)
        .unwrap(); 
    assert_eq!(other_user_is_owner, false);
}

#[test]
fn it_does_not_allow_nonsigners_to_remove_an_owner() {
    let (accounts, wallet) = multisig_setup();
    let mut evm = Evm::default();

    let other_user = accounts.get(&"otherUser").expect("No otherUser account found");
    let other_user2 = accounts.get(&"otherUser2").expect("No otherUser2 account found");
    let signer3 = accounts.get(&"signer3").expect("No signer3 account found");

    // Remove owner from first non-signer
    let _ = wallet
        .functions()
        .remove_owner()
        .call(signer3, &mut evm)
        .from(other_user)
        .unwrap();
    
    // Remove owner from second non-signer
    let rm_signer3 = wallet
        .functions()
        .remove_owner()
        .call(signer3, &mut evm)
        .from(other_user2)
        .unwrap();
    // Non-owners should not be able to remove owners
    assert!(rm_signer3.events().not_contains("OwnerRemoved"));

    // Signer3 should still be an owner
    let signer3_is_owner = wallet
        .functions()
        .is_owner()
        .call(signer3, &mut evm)
        .unwrap(); 
    assert_eq!(signer3_is_owner, true);
}

#[test]
fn it_allows_signers_to_change_required_number_of_signers() {
    let (accounts, wallet) = multisig_setup();
    let mut evm = Evm::default();

    let signer1 = accounts.get(&"signer1").expect("No signer1 account found");
    let signer2 = accounts.get(&"signer2").expect("No signer2 account found");

    // Change required signers from first signer
    let change_req = wallet
        .functions()
        .change_requirement()
        .call(3, &mut evm)
        .from(signer1)
        .unwrap();
    // Change requires two signers
    assert!(change_req.events().not_contains("RequirementChanged"));

    // Required signers should still be 2
    let num_req = wallet
        .functions()
        .m_required()
        .call(&mut evm)
        .unwrap();

    assert_eq!(num_req, 2);

    // Change required signers from second signer
    let change_req2 = wallet
        .functions()
        .change_requirement()
        .call(3, &mut evm)
        .from(signer2)
        .unwrap();
    // Requirement change should succeed 
    assert!(change_req2.events().contains("RequirementChanged"));

    // Required signers should be 3
    let num_req2 = wallet
        .functions()
        .m_required()
        .call(&mut evm)
        .unwrap();

    assert_eq!(num_req, 3);
}

#[test]
fn it_does_not_allow_required_eq_zero() {
    let (accounts, wallet) = multisig_setup();
    let mut evm = Evm::default();

    let signer1 = accounts.get(&"signer1").expect("No signer1 account found");
    let signer2 = accounts.get(&"signer2").expect("No signer2 account found");

    // Changing required signers to zero should fail
    let change_req = wallet
        .functions()
        .change_requirement()
        .call(0, &mut evm)
        .from(signer1)
        .unwrap();
    assert!(change_req.events().not_contains("RequirementChanged"));

    // Changing required signers to zero should fail
    let change_req2 = wallet
        .functions()
        .change_requirement()
        .call(0, &mut evm)
        .from(signer1)
        .unwrap();
    assert!(change_req2.events().not_contains("RequirementChanged"));
 
    // Required signers should still be 2
    let num_req = wallet
        .functions()
        .m_required()
        .call(&mut evm)
        .unwrap();
    assert_eq!(num_req, 2);
}

#[test]
fn it_does_not_allow_required_gt_num_signers() {
    let (accounts, wallet) = multisig_setup();
    let mut evm = Evm::default();

    let signer1 = accounts.get(&"signer1").expect("No signer1 account found");
    let signer2 = accounts.get(&"signer2").expect("No signer2 account found");

    // Attempt to set required owners greater than num owners
    // send from first signer
    let change_req = wallet
        .functions()
        .change_requirement()
        .call(4, &mut evm)
        .from(signer1)
        .unwrap();
    assert!(change_req.events().not_contains("RequirementChanged"));

    // Attempt to set required owners greater than num owners
    // send from second signer
    let change_req2= wallet
        .functions()
        .change_requirement()
        .call(4, &mut evm)
        .from(signer2)
        .unwrap();
    assert!(change_req2.events().not_contains("RequirementChanged"));

    // Required signers should still be 2
    let num_req = wallet
        .functions()
        .m_required()
        .call(&mut evm)
        .unwrap();
    assert_eq!(num_req, 2);
}

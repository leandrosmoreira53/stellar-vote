#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, testutils::{Address as _, Ledger}, Address, Env};

fn create_test_env() -> (Env, Address, Address, Address, Address) {
    let env = Env::default();
    let admin = Address::generate(&env);
    let voter1 = Address::generate(&env);
    let voter2 = Address::generate(&env);
    let voter3 = Address::generate(&env);
    
    (env, admin, voter1, voter2, voter3)
}

#[test]
fn test_initialize() {
    let (env, admin, _, _, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);

    let parties = client.get_parties();
    assert_eq!(parties.len(), 0);
}

#[test] 
#[should_panic(expected = "Contract already initialized")]
fn test_initialize_twice() {
    let (env, admin, _, _, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    client.initialize(&admin);
}

#[test]
fn test_add_party() {
    let (env, admin, _, _, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    
    let party_a = symbol_short!("PartyA");
    let party_b = symbol_short!("PartyB");
    
    client.add_party(&party_a);
    client.add_party(&party_b);

    let parties = client.get_parties();
    assert_eq!(parties.len(), 2);
    assert!(parties.contains(&party_a));
    assert!(parties.contains(&party_b));

    assert_eq!(client.get_vote_count(&party_a), 0);
    assert_eq!(client.get_vote_count(&party_b), 0);
}

#[test]
#[should_panic(expected = "Party already registered")]
fn test_add_duplicate_party() {
    let (env, admin, _, _, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    
    let party_a = symbol_short!("PartyA");
    client.add_party(&party_a);
    client.add_party(&party_a);
}

#[test]
fn test_add_voter() {
    let (env, admin, voter1, voter2, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    client.add_voter(&voter1);
    client.add_voter(&voter2);

    assert_eq!(client.get_voter_status(&voter1), VoterStatus::Registered);
    assert_eq!(client.get_voter_status(&voter2), VoterStatus::Registered);
}

#[test]
#[should_panic(expected = "Voter already registered")]
fn test_add_duplicate_voter() {
    let (env, admin, voter1, _, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    client.add_voter(&voter1);
    client.add_voter(&voter1);
}

#[test]
fn test_voting() {
    let (env, admin, voter1, voter2, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    
    let party_a = symbol_short!("PartyA");
    let party_b = symbol_short!("PartyB");
    
    client.add_party(&party_a);
    client.add_party(&party_b);
    client.add_voter(&voter1);
    client.add_voter(&voter2);

    client.vote(&voter1, &party_a);
    client.vote(&voter2, &party_b);

    assert_eq!(client.get_vote_count(&party_a), 1);
    assert_eq!(client.get_vote_count(&party_b), 1);
    assert_eq!(client.get_voter_status(&voter1), VoterStatus::Voted);
    assert_eq!(client.get_voter_status(&voter2), VoterStatus::Voted);
}

#[test]
#[should_panic(expected = "Voter not registered")]
fn test_unregistered_voter_cannot_vote() {
    let (env, admin, voter1, _, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    
    let party_a = symbol_short!("PartyA");
    client.add_party(&party_a);

    client.vote(&voter1, &party_a);
}

#[test]
#[should_panic(expected = "Party not found")]
fn test_vote_for_nonexistent_party() {
    let (env, admin, voter1, _, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    client.add_voter(&voter1);

    let nonexistent_party = symbol_short!("NoParty");
    client.vote(&voter1, &nonexistent_party);
}

#[test]
#[should_panic(expected = "Voter already voted")]
fn test_double_voting() {
    let (env, admin, voter1, _, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    
    let party_a = symbol_short!("PartyA");
    let party_b = symbol_short!("PartyB");
    
    client.add_party(&party_a);
    client.add_party(&party_b);
    client.add_voter(&voter1);

    client.vote(&voter1, &party_a);
    client.vote(&voter1, &party_b);
}

#[test]
fn test_delegation() {
    let (env, admin, voter1, voter2, voter3) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    
    let party_a = symbol_short!("PartyA");
    client.add_party(&party_a);
    client.add_voter(&voter1);
    client.add_voter(&voter2);
    client.add_voter(&voter3);

    client.delegate(&voter1, &voter2);
    client.delegate(&voter3, &voter2);
    client.vote(&voter2, &party_a);

    assert_eq!(client.get_vote_count(&party_a), 3);
    assert_eq!(client.get_voter_status(&voter1), VoterStatus::Delegated(voter2.clone()));
    assert_eq!(client.get_voter_status(&voter2), VoterStatus::Voted);
    assert_eq!(client.get_voter_status(&voter3), VoterStatus::Delegated(voter2));
}

#[test]
#[should_panic(expected = "Cannot delegate to yourself")]
fn test_self_delegation() {
    let (env, admin, voter1, _, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    client.add_voter(&voter1);

    client.delegate(&voter1, &voter1);
}

#[test]
#[should_panic(expected = "Delegator not registered")]
fn test_unregistered_delegator() {
    let (env, admin, voter1, voter2, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    client.add_voter(&voter2);

    client.delegate(&voter1, &voter2);
}

#[test]
#[should_panic(expected = "Delegate not registered")]
fn test_delegate_to_unregistered() {
    let (env, admin, voter1, voter2, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    client.add_voter(&voter1);

    client.delegate(&voter1, &voter2);
}

#[test]
#[should_panic(expected = "Voter has delegated their vote")]
fn test_delegated_voter_cannot_vote() {
    let (env, admin, voter1, voter2, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    
    let party_a = symbol_short!("PartyA");
    client.add_party(&party_a);
    client.add_voter(&voter1);
    client.add_voter(&voter2);

    client.delegate(&voter1, &voter2);
    client.vote(&voter1, &party_a);
}

#[test]
fn test_voting_stats() {
    let (env, admin, voter1, voter2, voter3) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    
    let party_a = symbol_short!("PartyA");
    let party_b = symbol_short!("PartyB");
    
    client.add_party(&party_a);
    client.add_party(&party_b);
    client.add_voter(&voter1);
    client.add_voter(&voter2);
    client.add_voter(&voter3);

    client.vote(&voter1, &party_a);
    client.vote(&voter2, &party_a);
    client.vote(&voter3, &party_b);

    let stats = client.get_voting_stats();
    assert_eq!(stats.total_votes, 3);
    assert_eq!(stats.total_parties, 2);
    assert_eq!(stats.total_voters, 3);
}

#[test]
fn test_get_all_results() {
    let (env, admin, voter1, voter2, voter3) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    
    let party_a = symbol_short!("PartyA");
    let party_b = symbol_short!("PartyB");
    let party_c = symbol_short!("PartyC");
    
    client.add_party(&party_a);
    client.add_party(&party_b);
    client.add_party(&party_c);
    client.add_voter(&voter1);
    client.add_voter(&voter2);
    client.add_voter(&voter3);

    client.vote(&voter1, &party_a);
    client.vote(&voter2, &party_a);
    client.vote(&voter3, &party_b);

    let results = client.get_all_results();
    assert_eq!(results.get(party_a).unwrap(), 2);
    assert_eq!(results.get(party_b).unwrap(), 1);
    assert_eq!(results.get(party_c).unwrap(), 0);
}

#[test]
fn test_voting_deadline() {
    let (env, admin, voter1, _, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    
    let party_a = symbol_short!("PartyA");
    client.add_party(&party_a);
    client.add_voter(&voter1);

    let future_deadline = env.ledger().timestamp() + 1000;
    client.set_voting_deadline(&future_deadline);

    assert_eq!(client.get_voting_deadline(), Some(future_deadline));

    client.vote(&voter1, &party_a);
    assert_eq!(client.get_vote_count(&party_a), 1);
}

#[test]
#[should_panic(expected = "Deadline must be in the future")]
fn test_past_deadline() {
    let (env, admin, _, _, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    
    let past_deadline = env.ledger().timestamp().saturating_sub(1);
    client.set_voting_deadline(&past_deadline);
}

#[test]
#[should_panic(expected = "Voting period has ended")]
fn test_voting_after_deadline() {
    let (env, admin, voter1, _, _) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    
    let party_a = symbol_short!("PartyA");
    client.add_party(&party_a);
    client.add_voter(&voter1);

    let deadline = env.ledger().timestamp() + 100;
    client.set_voting_deadline(&deadline);

    env.ledger().with_mut(|li| li.timestamp = deadline + 1);

    client.vote(&voter1, &party_a);
}

#[test]
#[should_panic(expected = "Circular delegation detected")]
fn test_circular_delegation() {
    let (env, admin, voter1, voter2, voter3) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);
    client.add_voter(&voter1);
    client.add_voter(&voter2);
    client.add_voter(&voter3);

    client.delegate(&voter1, &voter2);
    client.delegate(&voter2, &voter3);
    client.delegate(&voter3, &voter1);
}

#[test]
fn test_total_voters_count() {
    let (env, admin, voter1, voter2, voter3) = create_test_env();
    let contract_id = env.register(VotingContract, ());
    let client = VotingContractClient::new(&env, &contract_id);

    env.mock_all_auths();
    client.initialize(&admin);

    let stats_initial = client.get_voting_stats();
    assert_eq!(stats_initial.total_voters, 0);

    client.add_voter(&voter1);
    let stats_one = client.get_voting_stats();
    assert_eq!(stats_one.total_voters, 1);

    client.add_voter(&voter2);
    client.add_voter(&voter3);
    let stats_three = client.get_voting_stats();
    assert_eq!(stats_three.total_voters, 3);
}
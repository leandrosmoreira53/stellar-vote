#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Map, Symbol, Vec,
};


#[contracttype]
pub enum DataKey {
    Admin,
    Parties,
    Voters,
    Votes(Symbol),
    VoterStatus(Address),
    DelegatedVotes(Address),
    VotingDeadline,
    TotalVoters,
}

#[contracttype]
#[derive(Debug, PartialEq)]
pub enum VoterStatus {
    NotRegistered,
    Registered,
    Voted,
    Delegated(Address),
}

#[contracttype]
pub struct VotingStats {
    pub total_votes: u32,
    pub total_parties: u32,
    pub total_voters: u32,
}

#[contract]
pub struct VotingContract;

#[contractimpl]
impl VotingContract {
    pub fn initialize(env: Env, admin: Address) {
        admin.require_auth();
        
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Contract already initialized");
        }
        
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Parties, &Vec::<Symbol>::new(&env));
        env.storage().instance().set(&DataKey::TotalVoters, &0u32);
    }

    pub fn add_party(env: Env, party_name: Symbol) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin)
            .expect("Contract not initialized");
        admin.require_auth();

        let mut parties: Vec<Symbol> = env.storage().instance()
            .get(&DataKey::Parties)
            .unwrap_or_else(|| Vec::new(&env));

        if parties.contains(&party_name) {
            panic!("Party already registered");
        }

        parties.push_back(party_name.clone());
        env.storage().instance().set(&DataKey::Parties, &parties);
        env.storage().instance().set(&DataKey::Votes(party_name), &0u32);
    }

    pub fn add_voter(env: Env, voter: Address) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin)
            .expect("Contract not initialized");
        admin.require_auth();

        let current_status: VoterStatus = env.storage().instance()
            .get(&DataKey::VoterStatus(voter.clone()))
            .unwrap_or(VoterStatus::NotRegistered);

        match current_status {
            VoterStatus::NotRegistered => {
                env.storage().instance().set(&DataKey::VoterStatus(voter), &VoterStatus::Registered);
                let total_voters: u32 = env.storage().instance()
                    .get(&DataKey::TotalVoters)
                    .unwrap_or(0);
                env.storage().instance().set(&DataKey::TotalVoters, &(total_voters + 1));
            }
            _ => panic!("Voter already registered"),
        }
    }

    pub fn vote(env: Env, voter: Address, party_name: Symbol) {
        voter.require_auth();

        if let Some(deadline) = env.storage().instance().get::<DataKey, u64>(&DataKey::VotingDeadline) {
            if env.ledger().timestamp() > deadline {
                panic!("Voting period has ended");
            }
        }

        let voter_status: VoterStatus = env.storage().instance()
            .get(&DataKey::VoterStatus(voter.clone()))
            .unwrap_or(VoterStatus::NotRegistered);

        match voter_status {
            VoterStatus::Registered => {},
            VoterStatus::NotRegistered => panic!("Voter not registered"),
            VoterStatus::Voted => panic!("Voter already voted"),
            VoterStatus::Delegated(_) => panic!("Voter has delegated their vote"),
        }

        let parties: Vec<Symbol> = env.storage().instance()
            .get(&DataKey::Parties)
            .expect("No parties registered");

        if !parties.contains(&party_name) {
            panic!("Party not found");
        }

        let delegated_votes: u32 = env.storage().instance()
            .get(&DataKey::DelegatedVotes(voter.clone()))
            .unwrap_or(0);

        let total_voting_power = 1 + delegated_votes;

        let current_votes: u32 = env.storage().instance()
            .get(&DataKey::Votes(party_name.clone()))
            .unwrap_or(0);

        env.storage().instance().set(&DataKey::Votes(party_name), &(current_votes + total_voting_power));
        env.storage().instance().set(&DataKey::VoterStatus(voter), &VoterStatus::Voted);
    }

    pub fn delegate(env: Env, delegator: Address, delegate_to: Address) {
        delegator.require_auth();

        if delegator == delegate_to {
            panic!("Cannot delegate to yourself");
        }

        Self::check_circular_delegation(&env, &delegator, &delegate_to);

        let delegator_status: VoterStatus = env.storage().instance()
            .get(&DataKey::VoterStatus(delegator.clone()))
            .unwrap_or(VoterStatus::NotRegistered);

        let delegate_status: VoterStatus = env.storage().instance()
            .get(&DataKey::VoterStatus(delegate_to.clone()))
            .unwrap_or(VoterStatus::NotRegistered);

        match delegator_status {
            VoterStatus::Registered => {},
            VoterStatus::NotRegistered => panic!("Delegator not registered"),
            VoterStatus::Voted => panic!("Delegator already voted"),
            VoterStatus::Delegated(_) => panic!("Delegator already delegated"),
        }

        match delegate_status {
            VoterStatus::Registered => {},
            VoterStatus::NotRegistered => panic!("Delegate not registered"),
            VoterStatus::Voted => panic!("Cannot delegate to someone who already voted"),
            VoterStatus::Delegated(_) => panic!("Cannot delegate to someone who delegated"),
        }

        let current_delegated: u32 = env.storage().instance()
            .get(&DataKey::DelegatedVotes(delegate_to.clone()))
            .unwrap_or(0);

        env.storage().instance().set(&DataKey::DelegatedVotes(delegate_to.clone()), &(current_delegated + 1));
        env.storage().instance().set(&DataKey::VoterStatus(delegator), &VoterStatus::Delegated(delegate_to));
    }

    pub fn get_vote_count(env: Env, party_name: Symbol) -> u32 {
        env.storage().instance()
            .get(&DataKey::Votes(party_name))
            .unwrap_or(0)
    }

    pub fn get_parties(env: Env) -> Vec<Symbol> {
        env.storage().instance()
            .get(&DataKey::Parties)
            .unwrap_or_else(|| Vec::new(&env))
    }

    pub fn get_voter_status(env: Env, voter: Address) -> VoterStatus {
        env.storage().instance()
            .get(&DataKey::VoterStatus(voter))
            .unwrap_or(VoterStatus::NotRegistered)
    }

    pub fn get_voting_stats(env: Env) -> VotingStats {
        let parties: Vec<Symbol> = Self::get_parties(env.clone());
        let mut total_votes = 0u32;

        for party in parties.iter() {
            total_votes += Self::get_vote_count(env.clone(), party);
        }

        let total_voters: u32 = env.storage().instance()
            .get(&DataKey::TotalVoters)
            .unwrap_or(0);

        VotingStats {
            total_votes,
            total_parties: parties.len(),
            total_voters,
        }
    }

    pub fn get_all_results(env: Env) -> Map<Symbol, u32> {
        let parties: Vec<Symbol> = Self::get_parties(env.clone());
        let mut results = Map::new(&env);

        for party in parties.iter() {
            let votes = Self::get_vote_count(env.clone(), party.clone());
            results.set(party, votes);
        }

        results
    }

    pub fn set_voting_deadline(env: Env, deadline: u64) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin)
            .expect("Contract not initialized");
        admin.require_auth();

        if deadline <= env.ledger().timestamp() {
            panic!("Deadline must be in the future");
        }

        env.storage().instance().set(&DataKey::VotingDeadline, &deadline);
    }

    pub fn get_voting_deadline(env: Env) -> Option<u64> {
        env.storage().instance().get(&DataKey::VotingDeadline)
    }

    fn check_circular_delegation(env: &Env, delegator: &Address, delegate_to: &Address) {
        let mut current = delegate_to.clone();
        let mut visited = Vec::new(&env);
        
        loop {
            if visited.contains(&current) {
                panic!("Circular delegation detected");
            }
            
            visited.push_back(current.clone());
            
            if current == *delegator {
                panic!("Circular delegation detected");
            }
            
            let status: VoterStatus = env.storage().instance()
                .get(&DataKey::VoterStatus(current.clone()))
                .unwrap_or(VoterStatus::NotRegistered);
            
            match status {
                VoterStatus::Delegated(next_delegate) => {
                    current = next_delegate;
                },
                _ => break,
            }
            
            if visited.len() > 100 {
                panic!("Delegation chain too long");
            }
        }
    }
}

mod test;
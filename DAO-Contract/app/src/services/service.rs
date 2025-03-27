
// service.rs
// Necessary crates
use sails_rs::{
    prelude::*,
    gstd::msg,
    collections::HashMap,
};

// Static mutable variable for contract's state
pub static mut STATE: Option<State> = None;

// Struct to represent the state
#[derive(Clone, Default)]
pub struct State {
    pub admins: Vec<ActorId>,
    pub voters: HashMap<ActorId, Voter>,
    pub proposals: HashMap<u64, Proposal>,
    pub vote_counts: HashMap<u64, u64>,
    pub votes_cast: HashMap<ActorId, Vec<u64>>,
}

// Struct to represent a voter
#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Voter {
    pub name: String,
    pub eligible: bool,
}

// Struct to represent a proposal
#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoState {
    pub admins: Vec<ActorId>,
    pub voters: Vec<(ActorId, Voter)>,
    pub proposals: Vec<(u64, Proposal)>,
    pub vote_counts: Vec<(u64, u64)>,
    pub votes_cast: Vec<(ActorId, Vec<u64>)>,
}


// Struct to represent a proposal
#[derive(Encode, Decode, TypeInfo, Default, Clone)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Proposal {
    pub id: u64,
    pub description: String,
}

// Implement methods or related functions for state management
impl State {
    // Method to create a new instance
    pub fn new() -> Self {
        Self { ..Default::default() }
    }

    // Function to initialize the state
    pub fn init_state() {
        unsafe {
            STATE = Some(Self::new());
        };
    }

    // Function to get a mutable reference to the state
    pub fn state_mut() -> &'static mut State {
        let state = unsafe { STATE.as_mut() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }

    // Function to get an immutable reference to the state
    pub fn state_ref() -> &'static State {
        let state = unsafe { STATE.as_ref() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }
}

// Service struct
#[derive(Default)]
pub struct VotingService;

#[service]
impl VotingService {
    // Service constructor
    pub fn new() -> Self {
        Self
    }

    // Service to register a voter
    pub fn register_voter(&mut self, actor: ActorId, name: String) {
        let state = State::state_mut();
        let voter = Voter {
            name,
            eligible: true,
        };
        state.voters.insert(actor, voter);
    }

    // Service to register a proposal
    pub fn register_proposal(&mut self, id: u64, description: String) {
        let state = State::state_mut();
        let proposal = Proposal {
            id,
            description,
        };
        state.proposals.insert(id, proposal);
        state.vote_counts.insert(id, 0);
    }

    // Service to cast a vote
    pub fn vote(&mut self, voter: ActorId, proposal_id: u64) {
        let state = State::state_mut();
        let votes = state.votes_cast.entry(voter).or_insert(vec![]);
        
        // Check if voter is eligible and hasn't voted for this proposal yet
        if let Some(voter_info) = state.voters.get(&voter) {
            if voter_info.eligible && !votes.contains(&proposal_id) {
                *state.vote_counts.get_mut(&proposal_id).expect("Proposal not found") += 1;
                votes.push(proposal_id);
            }
        }
    }

    // Query service to get all proposals
    pub fn get_proposals(&self) -> Vec<Proposal> {
        State::state_ref()
            .proposals
            .values()
            .cloned()
            .collect()
    }

    // Query service to get vote counts for proposals
    pub fn get_vote_counts(&self) -> HashMap<u64, u64> {
        State::state_ref()
            .vote_counts
            .clone()
    }

    // Query service to get voter information
    pub fn get_voter_info(&self, voter_id: ActorId) -> Option<Voter> {
        State::state_ref()
            .voters
            .get(&voter_id)
            .cloned()
    }

    // Additional service to remove a voter
    pub fn remove_voter(&mut self, voter: ActorId) {
        let state = State::state_mut();
        state.voters.remove(&voter);
        state.votes_cast.remove(&voter);
    }

    // Additional service to conclude the voting
    pub fn conclude_voting(&self) -> Option<(u64, u64)> {
        let state = State::state_ref();
        state.vote_counts.iter().max_by_key(|&(_, &count)| count).cloned()
    }
}


// Implementation of the From trait for converting CustomStruct to IoCustomStruct
impl From<State> for IoState {

    // Conversion method
    fn from(value: State) -> Self {
        // Destructure the CustomStruct object into its individual fields
        let State {
            admins,
            voters,
            proposals,
            vote_counts,
            votes_cast,
        } = value;

        // Perform some transformation on second field, cloning its elements (Warning: Just for HashMaps!!)
        let voters = voters
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect();
        let proposals = proposals.iter().map(|(k, v)| (*k, v.clone())).collect();

        let vote_counts = vote_counts.iter().map(|(k, v)| (*k, v.clone())).collect();

        let votes_cast = votes_cast.iter().map(|(k, v)| (*k, v.clone())).collect();
   
        // Create a new IoCustomStruct object using the destructured fields
        Self {
            admins,
            voters,
            proposals,
            vote_counts,
            votes_cast,
        }
    }
}
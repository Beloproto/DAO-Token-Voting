#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, 
    Address, Env, Map, Symbol, Vec, BytesN,
};

// Import interfaces from the separate modules
mod token_interface {
    soroban_sdk::contractimport!(
        file = "dao_token_contract";
    );
}

mod proposal_interface {
    soroban_sdk::contractimport!(
        file = "dao_proposal_contract";
    );
}

mod voting_interface {
    soroban_sdk::contractimport!(
        file = "dao_voting_contract";
    );
}

// Define the main DAO contract
#[contract]
pub struct DAOContract;

#[contractimpl]
impl DAOContract {
    /// Initialize the entire DAO system
    pub fn initialize(
        env: Env,
        admin: Address,
        initial_supply: i128,
        voting_period: u64,
        quorum: i128,
    ) -> (Address, Address, Address) {
        // Require admin authorization
        admin.require_auth();
        
        // Deploy token contract
        let token_contract_id = env.deployer().upload_contract(include_bytes!("dao_token_contract"));
        let token_address = env.deployer().with_address(admin.clone(), token_contract_id.clone());
        
        // Initialize token
        let token_client = token_interface::Client::new(&env, &token_address);
        token_client.start(&admin, &initial_supply);
        
        // Deploy proposal contract
        let proposal_contract_id = env.deployer().upload_contract(include_bytes!("dao_proposal_contract"));
        let proposal_address = env.deployer().with_address(admin.clone(), proposal_contract_id.clone());
        
        // Deploy voting contract
        let voting_contract_id = env.deployer().upload_contract(include_bytes!("dao_voting_contract"));
        let voting_address = env.deployer().with_address(admin.clone(), voting_contract_id.clone());
        
        // Initialize proposal and voting contracts
        let proposal_client = proposal_interface::Client::new(&env, &proposal_address);
        proposal_client.initialize(&admin, &token_address, &voting_period, &quorum);
        
        // Store contract addresses
        env.storage().instance().set(&Symbol::("token"), &token_address);
        env.storage().instance().set(&Symbol::("proposal"), &proposal_address);
        env.storage().instance().set(&Symbol::("voting"), &voting_address);
        
        // Return the deployed contract addresses
        (token_address, proposal_address, voting_address)
    }
    
    // F1: Token Management
    
    /// Create new governance tokens
    pub fn create_tokens(env: Env, admin: Address, to: Address, amount: i128) -> i128 {
        // Get token contract address
        let token_address: Address = env.storage().instance().get(&Symbol::short("token")).unwrap();
        
        // Create client and call function
        let token_client = token_interface::Client::new(&env, &token_address);
        token_client.create_tokens(&to, &amount)
    }
    
    /// Transfer tokens between addresses
    pub fn send_tokens(env: Env, from: Address, to: Address, amount: i128) -> bool {
        // Get token contract address
        let token_address: Address = env.storage().instance().get(&Symbol::short("token")).unwrap();
        
        // Create client and call function
        let token_client = token_interface::Client::new(&env, &token_address);
        token_client.send_tokens(&from, &to, &amount)
    }
    
    /// Check token balance of an address
    pub fn check_balance(env: Env, address: Address) -> i128 {
        // Get token contract address
        let token_address: Address = env.storage().instance().get(&Symbol::short("token")).unwrap();
        
        // Create client and call function
        let token_client = token_interface::Client::new(&env, &token_address);
        token_client.check_balance(&address)
    }
    
    // F2: Proposal System functions
    
    /// Submit a new proposal to the DAO
    pub fn new_proposal(
        env: Env,
        creator: Address,
        description: Symbol,
        action_data: Symbol,
    ) -> BytesN<32> {
        // Get proposal contract address
        let proposal_address: Address = env.storage().instance().get(&Symbol::short("proposal")).unwrap();
        
        // Create client and call function
        let proposal_client = proposal_interface::Client::new(&env, &proposal_address);
        proposal_client.new_proposal(&creator, &description, &action_data)
    }
    
    /// Retrieve details about a specific proposal
    pub fn get_proposal(env: Env, proposal_id: BytesN<32>) -> proposal_interface::Proposal {
        // Get proposal contract address
        let proposal_address: Address = env.storage().instance().get(&Symbol::short("proposal")).unwrap();
        
        // Create client and call function
        let proposal_client = proposal_interface::Client::new(&env, &proposal_address);
        proposal_client.get_proposal(&proposal_id)
    }
    
    /// Get a list of all active proposals
    pub fn consult_proposals(env: Env) -> Vec<proposal_interface::Proposal> {
        // Get proposal contract address
        let proposal_address: Address = env.storage().instance().get(&Symbol::short("proposal")).unwrap();
        
        // Create client and call function
        let proposal_client = proposal_interface::Client::new(&env, &proposal_address);
        proposal_client.consult_proposals()
    }
    
    // F3: Voting System
    
    /// Vote on a proposal using governance tokens
    pub fn i_vote(
        env: Env,
        voter: Address,
        proposal_id: BytesN<32>,
        amount: i128,
        in_favor: bool,
    ) -> bool {
        // Get voting contract address
        let voting_address: Address = env.storage().instance().get(&Symbol::short("voting")).unwrap();
        
        // Create client and call function
        let voting_client = voting_interface::Client::new(&env, &voting_address);
        voting_client.i_vote(&voter, &proposal_id, &amount, &in_favor)
    }
    
    /// Check the current votes for a proposal
    pub fn check_vote_voices(env: Env, proposal_id: BytesN<32>) -> (i128, i128) {
        // Get voting contract address
        let voting_address: Address = env.storage().instance().get(&Symbol::short("voting")).unwrap();
        
        // Create client and call function
        let voting_client = voting_interface::Client::new(&env, &voting_address);
        voting_client.check_vote_voices(&proposal_id)
    }
    
    /// Execute a proposal after successful voting
    pub fn execute_proposal(env: Env, caller: Address, proposal_id: BytesN<32>) -> bool {
        // Get voting contract address
        let voting_address: Address = env.storage().instance().get(&Symbol::short("voting")).unwrap();
        
        // Create client and call function
        let voting_client = voting_interface::Client::new(&env, &voting_address);
        voting_client.execute_proposal(&caller, &proposal_id)
    }
    
    /// Determine if a proposal is active, passed, or failed
    pub fn check_proposal_status(env: Env, proposal_id: BytesN<32>) -> voting_interface::ProposalStatus {
        // Get voting contract address
        let voting_address: Address = env.storage().instance().get(&Symbol::short("voting")).unwrap();
        
        // Create client and call function
        let voting_client = voting_interface::Client::new(&env, &voting_address);
        voting_client.check_proposal_status(&proposal_id)
    }
}

// src/lib.rs
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Vec, Symbol, symbol_short, BytesN};

// Definir constantes
const SIGNATURE_THRESHOLD: u32 = 2; // Requiere al menos 2 firmas

// Estructura para almacenar información de un recurso ambiental
#[contracttype]
pub struct EnvironmentalResource {
    pub id: u64,
    pub name: String,
    pub origin: String,
    pub current_owner: Address,
    pub certification: String,
    pub tracking_history: Vec<String>,
}

// Estructura para una propuesta de transacción
#[contracttype]
pub struct TransactionProposal {
    pub id: u64,
    pub proposer: Address,
    pub resource_id: u64,
    pub new_owner: Address,
    pub description: String,
    pub signatures: Vec<Address>,
    pub executed: bool,
}

// Estructura para almacenar información del contrato
#[contracttype]
pub struct ContractData {
    pub admin: Address,
    pub signers: Vec<Address>,
    pub next_resource_id: u64,
    pub next_proposal_id: u64,
}

// Errores personalizados
#[contracttype]
pub enum Error {
    Unauthorized = 1,
    InvalidSignature = 2,
    InsufficientSignatures = 3,
    ProposalNotFound = 4,
    ResourceNotFound = 5,
    AlreadyExecuted = 6,
}

// Términos de almacenamiento
const ADMIN: Symbol = symbol_short!("ADMIN");
const SIGNERS: Symbol = symbol_short!("SIGNERS");
const RESOURCES: Symbol = symbol_short!("RESOURCES");
const PROPOSALS: Symbol = symbol_short!("PROPOSALS");
const NEXT_RESOURCE_ID: Symbol = symbol_short!("NEXT_RID");
const NEXT_PROPOSAL_ID: Symbol = symbol_short!("NEXT_PID");

#[contract]
pub struct BlocksResources;

#[contractimpl]
impl BlocksResources {
    // Inicializar el contrato con el administrador y los firmantes
    pub fn initialize(env: Env, admin: Address, signers: Vec<Address>) {
        // Verificar que el administrador no esté ya configurado
        if env.storage().instance().has(&ADMIN) {
            panic!("Contract already initialized");
        }

        // Verificar que hay al menos 2 firmantes
        if signers.len() < 2 {
            panic!("At least 2 signers are required");
        }

        // Guardar administrador y firmantes
        admin.require_auth();
        env.storage().instance().set(&ADMIN, &admin);
        env.storage().instance().set(&SIGNERS, &signers);
        
        // Inicializar contadores
        env.storage().instance().set(&NEXT_RESOURCE_ID, &0u64);
        env.storage().instance().set(&NEXT_PROPOSAL_ID, &0u64);
    }

    // Registrar un nuevo recurso ambiental
    pub fn register_resource(
        env: Env,
        name: String,
        origin: String,
        certification: String,
    ) -> u64 {
        // Verificar que el administrador está autorizado
        let admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        admin.require_auth();

        // Obtener el siguiente ID de recurso
        let mut next_id: u64 = env.storage().instance().get(&NEXT_RESOURCE_ID).unwrap();
        let resource_id = next_id;
        next_id += 1;
        env.storage().instance().set(&NEXT_RESOURCE_ID, &next_id);

        // Crear el recurso
        let resource = EnvironmentalResource {
            id: resource_id,
            name,
            origin,
            current_owner: admin.clone(),
            certification,
            tracking_history: Vec::new(&env),
        };

        // Guardar el recurso
        let resources_key = (RESOURCES, resource_id);
        env.storage().instance().set(&resources_key, &resource);

        resource_id
    }

    // Crear una propuesta para transferir un recurso
    pub fn create_transfer_proposal(
        env: Env,
        proposer: Address,
        resource_id: u64,
        new_owner: Address,
        description: String,
    ) -> u64 {
        // Verificar que el proponente está autorizado
        proposer.require_auth();

        // Verificar que el recurso existe
        let resources_key = (RESOURCES, resource_id);
        let resource: EnvironmentalResource = env.storage().instance().get(&resources_key)
            .unwrap_or_else(|| panic!("Resource not found"));

        // Verificar que el proponente es el propietario actual o un firmante autorizado
        let signers: Vec<Address> = env.storage().instance().get(&SIGNERS).unwrap();
        if resource.current_owner != proposer && !signers.contains(&proposer) {
            panic!("Unauthorized to create proposal");
        }

        // Obtener el siguiente ID de propuesta
        let mut next_id: u64 = env.storage().instance().get(&NEXT_PROPOSAL_ID).unwrap();
        let proposal_id = next_id;
        next_id += 1;
        env.storage().instance().set(&NEXT_PROPOSAL_ID, &next_id);

        // Crear la propuesta
        let proposal = TransactionProposal {
            id: proposal_id,
            proposer: proposer.clone(),
            resource_id,
            new_owner: new_owner.clone(),
            description,
            signatures: Vec::new(&env),
            executed: false,
        };

        // Guardar la propuesta
        let proposals_key = (PROPOSALS, proposal_id);
        env.storage().instance().set(&proposals_key, &proposal);

        proposal_id
    }

    // Firmar una propuesta
    pub fn sign_proposal(env: Env, signer: Address, proposal_id: u64) {
        // Verificar que el firmante está autorizado
        signer.require_auth();

        // Verificar que el firmante está en la lista de firmantes autorizados
        let signers: Vec<Address> = env.storage().instance().get(&SIGNERS).unwrap();
        if !signers.contains(&signer) {
            panic!("Unauthorized signer");
        }

        // Obtener la propuesta
        let proposals_key = (PROPOSALS, proposal_id);
        let mut proposal: TransactionProposal = env.storage().instance().get(&proposals_key)
            .unwrap_or_else(|| panic!("Proposal not found"));

        // Verificar que la propuesta no ha sido ejecutada
        if proposal.executed {
            panic!("Proposal already executed");
        }

        // Verificar que el firmante no ha firmado ya
        if proposal.signatures.contains(&signer) {
            panic!("Already signed");
        }

        // Añadir la firma
        proposal.signatures.push_back(signer);

        // Guardar la propuesta actualizada
        env.storage().instance().set(&proposals_key, &proposal);

        // Si se alcanza el umbral de firmas, ejecutar la transacción
        if proposal.signatures.len() >= SIGNATURE_THRESHOLD as usize {
            Self::execute_transfer(env, proposal_id);
        }
    }

    // Ejecutar una transferencia de recurso
    fn execute_transfer(env: Env, proposal_id: u64) {
        // Obtener la propuesta
        let proposals_key = (PROPOSALS, proposal_id);
        let mut proposal: TransactionProposal = env.storage().instance().get(&proposals_key)
            .unwrap_or_else(|| panic!("Proposal not found"));

        // Obtener el recurso
        let resources_key = (RESOURCES, proposal.resource_id);
        let mut resource: EnvironmentalResource = env.storage().instance().get(&resources_key)
            .unwrap_or_else(|| panic!("Resource not found"));

        // Actualizar el propietario y añadir a la historia de seguimiento
        let transfer_info = format!(
            "Transferred from {} to {} on {}",
            resource.current_owner.to_string(),
            proposal.new_owner.to_string(),
            env.ledger().timestamp()
        );
        resource.tracking_history.push_back(transfer_info);
        resource.current_owner = proposal.new_owner.clone();

        // Marcar la propuesta como ejecutada
        proposal.executed = true;

        // Guardar los cambios
        env.storage().instance().set(&resources_key, &resource);
        env.storage().instance().set(&proposals_key, &proposal);
    }

    // Obtener información de un recurso
    pub fn get_resource(env: Env, resource_id: u64) -> EnvironmentalResource {
        let resources_key = (RESOURCES, resource_id);
        env.storage().instance().get(&resources_key)
            .unwrap_or_else(|| panic!("Resource not found"))
    }

    // Obtener información de una propuesta
    pub fn get_proposal(env: Env, proposal_id: u64) -> TransactionProposal {
        let proposals_key = (PROPOSALS, proposal_id);
        env.storage().instance().get(&proposals_key)
            .unwrap_or_else(|| panic!("Proposal not found"))
    }

    // Obtener la lista de firmantes autorizados
    pub fn get_signers(env: Env) -> Vec<Address> {
        env.storage().instance().get(&SIGNERS).unwrap()
    }

    // Añadir un nuevo firmante (solo administrador)
    pub fn add_signer(env: Env, admin: Address, new_signer: Address) {
        // Verificar que el administrador está autorizado
        let stored_admin: Address = env.storage().instance().get(&ADMIN).unwrap();
        if admin != stored_admin {
            panic!("Unauthorized");
        }
        admin.require_auth();

        // Obtener la lista actual de firmantes
        let mut signers: Vec<Address> = env.storage().instance().get(&SIGNERS).unwrap();

        // Verificar que el nuevo firmante no está ya en la lista
        if signers.contains(&new_signer) {
            panic!("Signer already exists");
        }

        // Añadir el nuevo firmante
        signers.push_back(new_signer);

        // Guardar la lista actualizada
        env.storage().instance().set(&SIGNERS, &signers);
    }
}

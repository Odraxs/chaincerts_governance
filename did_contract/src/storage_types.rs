//! Module StorageTypes
//!
//! Module that defines the set of keys that can be used to access and store data within the contract.
use soroban_sdk::contracttype;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    /// DID document Id
    Id,
    /// Vec<String> with the authentication keys of the DID document
    Authentications,
    /// Vec<VerificationMethod> with the verification methods of the DID document
    VerificationMethods,
    /// Access Control List
    AccessControlList,
    /// A map that stores the VerifiableCredentials, identified by a credential_id `Map<String, VerifiableCredentials>`
    VerifiableCredentials,
    /// Vec<String> that stores DID document context urls
    Context,
    /// Vec<Method> that stores DID document verification processes
    VerificationProcesses,
    /// Vec<Service> that stored DID document services
    Services,
    /// Stores the DID document `Metadata`
    Metadata,
}

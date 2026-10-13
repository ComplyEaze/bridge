//! Deterministic, synthetic Tally protocol fixtures and a one-request loopback server.
//!
//! This crate never launches Tally and cannot bind a non-loopback interface.

mod fixtures;
mod master_generator;
mod server;

pub use fixtures::{
    decode, encode, Delivery, Fixture, ProductStatus, ResponseContentEncoding, ResponseFraming,
    ScenarioPlan, WireEncoding,
};
pub use master_generator::{
    generate_master_corpus, GeneratedMasterCorpus, MasterCorpusSpec, MasterGenerationError,
    MAX_GENERATED_MASTERS, MAX_MASTER_TEXT_WIDTH,
};
pub use server::{
    NoRequestForPlan, ObservedRequest, RefusedRequest, RequestFault, SequenceSimulator, Simulator,
    MAX_SEQUENCE_REQUESTS,
};

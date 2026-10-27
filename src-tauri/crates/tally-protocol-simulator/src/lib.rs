//! Deterministic, synthetic Tally protocol fixtures and a one-request loopback server.
//!
//! This crate never launches Tally and cannot bind a non-loopback interface.

mod fixtures;
mod server;

pub use fixtures::{
    decode, encode, Delivery, DeliveryGate, Fixture, ProductStatus, ResponseContentEncoding,
    ResponseFraming, ScenarioPlan, WireEncoding,
};
pub use server::{
    NoRequestForPlan, ObservedRequest, RefusedRequest, RequestFault, SequenceSimulator, Simulator,
    MAX_SEQUENCE_REQUESTS,
};

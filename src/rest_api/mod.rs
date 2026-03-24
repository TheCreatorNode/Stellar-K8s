//! REST API module for external integrations
//!
//! Provides an HTTP API for querying and managing StellarNodes.

mod auth;
mod custom_metrics;
mod dashboard_dto;
mod dashboard_handlers;
mod dto;
mod handlers;
mod server;
mod sustainability;

pub use auth::{check_rbac_permission, k8s_rbac_auth};
pub use server::{build_tls_server_config, run_server};

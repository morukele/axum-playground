//! logger.rs
//! =========
//!
//! Author: oghenemarho
//! Created: 08/02/2026
//! Project: axum-playground
//!
//! Description:
//! A script for creating and configuring loggers

pub fn init_tracing() {
    tracing_subscriber::fmt().with_target(false).init();
}

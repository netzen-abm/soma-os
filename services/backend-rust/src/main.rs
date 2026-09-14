use axum::{
    extract::{Json, State},
    http::StatusCode,
    routing::{get, post},
};
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};
use tokio::net::TcpListener;

mod ambali_timer;
mod backup_scheduler;
mod bot_menus;
mod canonical_authorization;
mod canonical_vault_authorizer;
mod compliance_shield;
mod compressor;
mod crypto;
mod data_parser;
mod db_layer;
mod device_sync;
mod health_state_evidence_link;
mod key_restoration;
mod legacy_promotion_preflight;
mod local_health_vault;
mod local_health_vault_storage;
mod local_longitudinal_observation_repository;
mod longitudinal_observation_repository;
mod messenger_webhook;
mod meta_outbound;
mod mnemonic_validator;
mod nostr_client;
mod personal_health_record_repository;
mod privacy_policy;
mod protected_db_context;
mod shared_infrastructure;
mod vault_exporter;
mod vault_importer;
mod whatsapp_webhook;

#[cfg(test)]
mod db_postgres_integration_test;

use ambali_timer::FermentationOrchestrator;
use bot_menus::MultiChannelMenuController;
use compliance_shield::SovereignComplianceShield;
use db_layer::SomaDatabaseManager;

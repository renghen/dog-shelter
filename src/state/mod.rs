use crate::settings::Settings;
use arc_swap::ArcSwap;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub struct ApplicationState {
    pub settings: ArcSwap<Settings>,
    pub db_connections: ArcSwap<DatabaseConnection>,
}

impl ApplicationState {
    pub fn new(settings: &Settings, db_conn: DatabaseConnection) -> anyhow::Result<Self> {
        Ok(Self {
            settings: ArcSwap::new(Arc::new((*settings).clone())),
            db_connections: ArcSwap::new(Arc::new((db_conn).clone())),
        })
    }
}

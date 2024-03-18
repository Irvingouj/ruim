use crate::{db, jwt};

#[derive(Clone)]
pub struct RuimContext {
    pub db: db::Database,
    pub jwt: jwt::Jwt,
    pub session_manager: crate::core::session_manager::SessionManager,
}

impl RuimContext {
    pub async fn new() -> anyhow::Result<Self> {
        let db = db::Database::new().await?;
        let jwt = jwt::Jwt::new_from_env()?;
        Ok(Self {
            db,
            jwt,
            session_manager: crate::core::session_manager::SessionManager::new(),
        })
    }
}

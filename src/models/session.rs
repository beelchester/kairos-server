use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Session {
    #[serde(rename = "sessionId")]
    pub session_id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    #[serde(rename = "projectId")]
    pub project_id: Uuid,
    #[serde(rename = "startedAt")]
    pub started_at: DateTime<Utc>,
    #[serde(rename = "endedAt")]
    pub ended_at: Option<DateTime<Utc>>,
    pub duration: i32,
}

impl Session {
    pub fn new(
        session_id: Uuid,
        user_id: Uuid,
        project_id: Uuid,
        started_at: DateTime<Utc>,
        ended_at: Option<DateTime<Utc>>,
        duration: i32,
    ) -> Self {
        Self {
            session_id,
            user_id,
            project_id,
            started_at,
            ended_at,
            duration,
        }
    }
}

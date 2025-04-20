use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize, Debug)]

pub struct Project {
    #[serde(rename = "projectId")]
    pub project_id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    #[serde(rename = "projectName")]
    pub project_name: String,
    pub colour: String,
    pub deadline: Option<DateTime<Utc>>,
    pub priority: Option<i32>,
}

impl Project {
    pub fn new(
        user_id: Uuid,
        project_id: Uuid,
        project_name: String,
        colour: String,
        deadline: Option<DateTime<Utc>>,
        priority: Option<i32>,
    ) -> Self {
        Self {
            user_id,
            project_id,
            project_name,
            colour,
            deadline,
            priority,
        }
    }
}

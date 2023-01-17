use chrono::Utc;
pub use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DurationSeconds};

use crate::domain::{types::*, Entity};


#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct Activity {
    id: Option<Id>,
    version: i32,
    created_at: DateTime,
    updated_at: DateTime,
    started_at: DateTime,
    #[serde_as(as = "DurationSeconds<i64>")]
    duration: Duration,
    summary: Text,
}

impl Default for Activity {
    fn default() -> Self {
        Self {
            id: Default::default(),
            version: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
            started_at: Default::default(),
            duration: chrono::Duration::zero(),
            summary: Default::default(),
        }
    }
}

impl Entity for Activity {
    fn id(&self) -> Option<String> {
        self.id.clone()
    }

    fn collection(&self) -> String {
        String::from("activity")
    }
}

impl Activity {
    fn ensure_updated(mut self) -> Self {
        self.updated_at = Utc::now();
        self.version = self.version + 1;
        self
    }

    pub fn version(&self) -> &i32 {
        &self.version
    }

    pub fn summary(&self) -> &Text {
        &self.summary
    }

    pub fn with_summary<S>(self, summary: S) -> Self
    where
        S: Into<Text>,
    {
        Activity {
            summary: summary.into(),
            ..self
        }
        .ensure_updated()
    }

    pub fn started_at(&self) -> &DateTime {
        &self.started_at
    }

    pub fn with_started_at<S>(self, started_at: S) -> Self
    where
        S: Into<DateTime>,
    {
        Activity {
            started_at: started_at.into(),
            ..self
        }
        .ensure_updated()
    }

    pub fn duration(&self) -> &Duration {
        &self.duration
    }

    pub fn with_duration<S>(self, duration: S) -> Self
    where
        S: Into<Duration>,
    {
        Activity {
            duration: duration.into(),
            ..self
        }
        .ensure_updated()
    }
}

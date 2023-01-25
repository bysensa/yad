use serde::{Serialize, Deserialize};



#[derive(Clone,Debug, Serialize, Deserialize)]
pub struct ActivatedApplication {
    pub application: Application,
    pub at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone,Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Application {
    pub name: String,
    pub bundle: String,
}

#[derive(Clone,Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FocusSwitch {
    pub from: Application,
    pub to: Application,
    pub at: chrono::DateTime<chrono::Utc>
}

#[derive(Clone,Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FocusTime {
    pub on: Application,
    pub during: std::time::Duration,
    pub since: chrono::DateTime<chrono::Utc>
}
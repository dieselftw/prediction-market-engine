use bson::{oid::ObjectId, Document};
use chrono::{DateTime, Utc};
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Event {
    /// Unique ID in your DB
    pub id: ObjectId,

    /// Human-readable question: "Will X happen by Y date?"
    pub title: String,

    /// More detailed description / resolution criteria
    pub description: String,

    /// When trading opens and closes
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,

    /// The set of mutually exclusive outcomes, e.g. ["YES", "NO"] or ["A", "B", "C"]
    pub outcomes: Arc<[String]>,

    /// Optional resolved outcome (index into `outcomes` or explicit label)
    pub resolved_outcome: Option<String>,

    /// Status of the event in the lifecycle
    pub status: EventStatus,

    /// Free-form metadata stored as BSON (for schemas that may evolve)
    pub metadata: Document,
}

#[allow(dead_code)]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum EventStatus {
    Pending,   // not yet open
    Open,      // trading active
    Paused,    // temporarily halted
    Resolved,  // final outcome known
    Canceled,  // voided
}

#[derive(Debug)]
pub struct EventData {
    pub title: String,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub outcomes: Arc<[String]>,
    pub metadata: Document,
}

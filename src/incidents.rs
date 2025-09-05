//! This module defines the **core data structures** for the program.
//!
//! - [`Incident`] is a record of an event (with datetime, people, description, and status).
//! - [`Status`] represents the progress of an incident (Pending, In Progress, Resolved).
//!
//! ## How It Fits in the Flow
//! Every time the user adds an incident, a new [`Incident`] struct is created
//! and stored in the [`Database`](crate::database::Database).
//! Later, the status can be updated, or the incident can be listed/deleted.

use serde::{Serialize, Deserialize};

/// Represents the status of an incident.
///
/// An incident can be in one of three states:
/// - [`Pending`]: The incident has been reported but not yet addressed.
/// - [`InProgress`]: Work is currently being done to resolve the incident.
/// - [`Resolved`]: The incident has been successfully resolved.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Status {
    /// Reported but not yet addressed.
    Pending,

    /// Work in progress on the incident.
    InProgress,

    /// Incident has been resolved.
    Resolved,
}

/// Represents an incident record stored in the system.
///
/// Each incident contains:
/// - A unique [`id`] (assigned automatically by the database)
/// - A [`datetime`] string (user-provided, e.g. `"2025-09-06 14:00"`)
/// - A list of [`people_involved`] in the incident
/// - A text [`description`] of the incident
/// - Its current [`status`]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Incident {
    /// Unique identifier for the incident (auto-incremented).
    pub(crate) id: u32,

    /// The datetime of the incident, stored as a string.
    pub(crate) datetime: String,

    /// People involved in the incident.
    pub(crate) people_involved: Vec<String>,

    /// Description of what happened in the incident.
    pub(crate) description: String,

    /// Current status of the incident.
    pub(crate) status: Status,
}

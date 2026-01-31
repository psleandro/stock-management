use std::fmt;

use serde::Serialize;

// USER ID
#[derive(Debug, Clone, Copy, Eq, PartialEq, Serialize)]
pub struct UserId(pub i32);

impl UserId {
    pub fn value(self) -> i32 {
        self.0
    }
}

impl From<UserId> for i32 {
    fn from(id: UserId) -> Self {
        id.0
    }
}

impl From<UserId> for String {
    fn from(id: UserId) -> Self {
        id.0.to_string()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// WORKSPACE ID
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct WorkspaceId(pub i32);

impl WorkspaceId {
    pub fn value(self) -> i32 {
        self.0
    }
}

impl From<WorkspaceId> for i32 {
    fn from(id: WorkspaceId) -> Self {
        id.0
    }
}

impl fmt::Display for WorkspaceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

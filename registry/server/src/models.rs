//! Data models for the AZC registry.

use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// A published package
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Package {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub repository: Option<String>,
    pub license: Option<String>,
    pub owner_id: Uuid,
    pub downloads: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// A specific version of a package
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PackageVersion {
    pub id: Uuid,
    pub package_id: Uuid,
    pub version: String,
    pub readme: Option<String>,
    pub checksum: String,
    pub file_size: i64,
    pub downloads: i64,
    pub yanked: bool,
    pub created_at: DateTime<Utc>,
}

/// A dependency reference
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Dependency {
    pub id: Uuid,
    pub version_id: Uuid,
    pub name: String,
    pub version_req: String,
}

/// A registered user
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub api_key: String,
    pub created_at: DateTime<Utc>,
}

/// Request to register a new user
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// Request to publish a package
#[derive(Debug, Deserialize)]
pub struct PublishRequest {
    pub name: String,
    pub vers: String,
    pub deps: Vec<DependencyRequest>,
    pub readme: Option<String>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub description: Option<String>,
}

/// Dependency in publish request
#[derive(Debug, Deserialize)]
pub struct DependencyRequest {
    pub name: String,
    pub version_req: String,
}

/// Response for package listing
#[derive(Debug, Serialize)]
pub struct PackageListResponse {
    pub packages: Vec<PackageSummary>,
    pub total: i64,
    pub page: i32,
    pub per_page: i32,
}

/// Summary of a package for listings
#[derive(Debug, Serialize)]
pub struct PackageSummary {
    pub name: String,
    pub description: Option<String>,
    pub latest_version: Option<String>,
    pub downloads: i64,
    pub updated_at: DateTime<Utc>,
}

/// Response for package details
#[derive(Debug, Serialize)]
pub struct PackageResponse {
    pub package: Package,
    pub versions: Vec<VersionSummary>,
    pub owner: UserSummary,
}

/// Summary of a version
#[derive(Debug, Serialize)]
pub struct VersionSummary {
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub downloads: i64,
    pub yanked: bool,
}

/// Summary of a user
#[derive(Debug, Serialize)]
pub struct UserSummary {
    pub username: String,
}

/// Search query parameters
#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Search response
#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<PackageSummary>,
    pub total: i64,
}

/// Error response
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub detail: Option<String>,
}

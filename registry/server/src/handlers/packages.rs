//! Package-related handlers.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use crate::handlers::{ApiError, AppState};
use crate::models::*;

/// List all packages with pagination
pub async fn list_packages(
    State(state): State<AppState>,
    Query(params): Query<ListQuery>,
) -> Result<Json<PackageListResponse>, ApiError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let packages = sqlx::query_as!(
        Package,
        r#"
        SELECT id, name, description, repository, license, owner_id, downloads, created_at, updated_at
        FROM packages
        ORDER BY downloads DESC
        LIMIT $1 OFFSET $2
        "#,
        per_page as i64,
        offset as i64,
    )
    .fetch_all(&state.pool)
    .await?;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM packages")
        .fetch_one(&state.pool)
        .await?;

    let summaries: Vec<PackageSummary> = packages
        .into_iter()
        .map(|p| PackageSummary {
            name: p.name,
            description: p.description,
            latest_version: None, // TODO: fetch latest version
            downloads: p.downloads,
            updated_at: p.updated_at,
        })
        .collect();

    Ok(Json(PackageListResponse {
        packages: summaries,
        total,
        page,
        per_page,
    }))
}

#[derive(Debug, serde::Deserialize)]
pub struct ListQuery {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

/// Get package details
pub async fn get_package(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<PackageResponse>, ApiError> {
    let package = sqlx::query_as!(
        Package,
        r#"
        SELECT id, name, description, repository, license, owner_id, downloads, created_at, updated_at
        FROM packages
        WHERE name = $1
        "#,
        name,
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| ApiError {
        status: StatusCode::NOT_FOUND,
        message: format!("Package '{}' not found", name),
    })?;

    let versions = sqlx::query_as!(
        PackageVersion,
        r#"
        SELECT id, package_id, version, readme, checksum, file_size, downloads, yanked, created_at
        FROM package_versions
        WHERE package_id = $1
        ORDER BY created_at DESC
        "#,
        package.id,
    )
    .fetch_all(&state.pool)
    .await?;

    let owner = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash, api_key, created_at
        FROM users
        WHERE id = $1
        "#,
        package.owner_id,
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(PackageResponse {
        package,
        versions: versions
            .into_iter()
            .map(|v| VersionSummary {
                version: v.version,
                created_at: v.created_at,
                downloads: v.downloads,
                yanked: v.yanked,
            })
            .collect(),
        owner: UserSummary {
            username: owner.username,
        },
    }))
}

/// List versions of a package
pub async fn list_versions(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<Vec<VersionSummary>>, ApiError> {
    let versions = sqlx::query_as!(
        PackageVersion,
        r#"
        SELECT pv.id, pv.package_id, pv.version, pv.readme, pv.checksum, pv.file_size, pv.downloads, pv.yanked, pv.created_at
        FROM package_versions pv
        JOIN packages p ON pv.package_id = p.id
        WHERE p.name = $1
        ORDER BY pv.created_at DESC
        "#,
        name,
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(
        versions
            .into_iter()
            .map(|v| VersionSummary {
                version: v.version,
                created_at: v.created_at,
                downloads: v.downloads,
                yanked: v.yanked,
            })
            .collect(),
    ))
}

/// Get specific version of a package
pub async fn get_version(
    State(state): State<AppState>,
    Path((name, version)): Path<(String, String)>,
) -> Result<Json<PackageVersion>, ApiError> {
    let pkg_version = sqlx::query_as!(
        PackageVersion,
        r#"
        SELECT pv.id, pv.package_id, pv.version, pv.readme, pv.checksum, pv.file_size, pv.downloads, pv.yanked, pv.created_at
        FROM package_versions pv
        JOIN packages p ON pv.package_id = p.id
        WHERE p.name = $1 AND pv.version = $2
        "#,
        name,
        version,
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| ApiError {
        status: StatusCode::NOT_FOUND,
        message: format!("Version {} of package '{}' not found", version, name),
    })?;

    Ok(Json(pkg_version))
}

/// Search packages
pub async fn search_packages(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> Result<Json<SearchResponse>, ApiError> {
    let pattern = format!("%{}%", query.q);
    
    let packages = sqlx::query_as!(
        Package,
        r#"
        SELECT id, name, description, repository, license, owner_id, downloads, created_at, updated_at
        FROM packages
        WHERE name ILIKE $1 OR description ILIKE $1
        ORDER BY downloads DESC
        LIMIT 20
        "#,
        pattern,
    )
    .fetch_all(&state.pool)
    .await?;

    let total = packages.len() as i64;

    Ok(Json(SearchResponse {
        results: packages
            .into_iter()
            .map(|p| PackageSummary {
                name: p.name,
                description: p.description,
                latest_version: None,
                downloads: p.downloads,
                updated_at: p.updated_at,
            })
            .collect(),
        total,
    }))
}

/// Publish a new package
pub async fn publish_package(
    State(state): State<AppState>,
    Json(req): Json<PublishRequest>,
) -> Result<StatusCode, ApiError> {
    // Validate version format
    let version = semver::Version::parse(&req.vers)
        .map_err(|_| ApiError {
            status: StatusCode::BAD_REQUEST,
            message: format!("Invalid version: {}", req.vers),
        })?;

    // Check if package exists, create if not
    let package = sqlx::query_as!(
        Package,
        r#"
        INSERT INTO packages (name, description, repository, license, owner_id)
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT (name) DO UPDATE SET updated_at = NOW()
        RETURNING id, name, description, repository, license, owner_id, downloads, created_at, updated_at
        "#,
        req.name,
        req.description,
        req.repository,
        req.license,
        uuid::Uuid::nil(), // TODO: use authenticated user
    )
    .fetch_one(&state.pool)
    .await?;

    // Create version
    sqlx::query!(
        r#"
        INSERT INTO package_versions (package_id, version, readme, checksum, file_size)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        package.id,
        version.to_string(),
        req.readme,
        "checksum_placeholder", // TODO: compute actual checksum
        0i64, // TODO: actual file size
    )
    .execute(&state.pool)
    .await?;

    Ok(StatusCode::CREATED)
}
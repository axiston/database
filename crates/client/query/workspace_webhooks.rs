//! Data layer for managing workspace webhooks.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workspace_webhooks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceWebhookCreateInput {
    pub workspace_id: Uuid,
    pub metadata: Value,
}

#[derive(Debug, Clone, Queryable)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workspace_webhooks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceWebhookCreateOutput {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub metadata: Value,

    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

/// Creates a new workspace webhook.
///
/// # Tables
///
///  - workspace_webhooks
pub async fn create_workspace_webhook(
    conn: &mut AsyncPgConnection,
    form: &WorkspaceWebhookCreateInput,
) -> DatabaseResult<WorkspaceWebhookCreateOutput> {
    use schema::workspace_webhooks::dsl::*;

    let query = insert_into(workspace_webhooks)
        .values(form)
        .returning((
            id,
            workspace_id,
            metadata,
            created_at,
            updated_at,
            deleted_at,
        ))
        .get_result(conn)
        .await?;

    Ok(query)
}

/// Retrieves a workspace webhook by its ID.
///
/// # Tables
///
///  - workspace_webhooks
pub async fn view_workspace_webhook(
    conn: &mut AsyncPgConnection,
    webhook_id: Uuid,
) -> DatabaseResult<WorkspaceWebhookCreateOutput> {
    use schema::workspace_webhooks::dsl::*;

    let filter_cond = id.eq(webhook_id).and(deleted_at.is_null());
    let query = workspace_webhooks
        .filter(filter_cond)
        .get_result(conn)
        .await?;

    Ok(query)
}

#[derive(Debug, Clone, AsChangeset)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[diesel(table_name = schema::workspace_webhooks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceWebhookUpdateInput {
    pub metadata: Option<Value>,
}

/// Updates a workspace webhook.
///
/// # Tables
///
///  - workspace_webhooks
pub async fn update_workspace_webhook(
    conn: &mut AsyncPgConnection,
    webhook_id: Uuid,
    form: WorkspaceWebhookUpdateInput,
) -> DatabaseResult<()> {
    use schema::workspace_webhooks::dsl::*;

    let filter_cond = id.eq(webhook_id).and(deleted_at.is_null());
    update(workspace_webhooks.filter(filter_cond))
        .set(form)
        .execute(conn)
        .await?;

    Ok(())
}

/// Flags the workspace webhook as deleted.
///
/// # Tables
///
///  - workspace_webhooks
pub async fn delete_workspace_webhook(
    conn: &mut AsyncPgConnection,
    webhook_id: Uuid,
) -> DatabaseResult<()> {
    use schema::workspace_webhooks::dsl::*;

    let filter_cond = id.eq(webhook_id).and(deleted_at.is_null());
    update(workspace_webhooks.filter(filter_cond))
        .set(deleted_at.eq(now))
        .execute(conn)
        .await?;

    Ok(())
}

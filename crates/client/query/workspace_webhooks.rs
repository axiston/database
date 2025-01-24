//! Data layer for managing workspace webhooks.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde_json::Value;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::workspace_webhooks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceWebhookCreateInputForm<'a> {
    pub workspace_id: Uuid,
    pub metadata: &'a Value,
}

#[derive(Debug, Clone, Queryable)]
#[diesel(table_name = schema::workspace_webhooks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceWebhookOutputForm {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub metadata: Value,
    pub created_at: PrimitiveDateTime,
    pub updated_at: PrimitiveDateTime,
    pub deleted_at: Option<PrimitiveDateTime>,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = schema::workspace_webhooks)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct WorkspaceWebhookUpdateInputForm<'a> {
    pub metadata: Option<&'a Value>,
}

/// Creates a new workspace webhook.
///
/// # Tables
///
///  - workspace_webhooks
pub async fn create_workspace_webhook(
    conn: &mut AsyncPgConnection,
    webhook_form: &WorkspaceWebhookCreateInputForm<'_>,
) -> DatabaseResult<WorkspaceWebhookOutputForm> {
    use schema::workspace_webhooks::dsl::*;

    let query = insert_into(workspace_webhooks)
        .values(webhook_form)
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
) -> DatabaseResult<WorkspaceWebhookOutputForm> {
    use schema::workspace_webhooks::dsl::*;

    let filter_cond = id.eq(webhook_id).and(deleted_at.is_null());
    let query = workspace_webhooks
        .filter(filter_cond)
        .get_result(conn)
        .await?;

    Ok(query)
}

/// Updates a workspace webhook.
///
/// # Tables
///
///  - workspace_webhooks
pub async fn update_workspace_webhook(
    conn: &mut AsyncPgConnection,
    webhook_id: Uuid,
    form: WorkspaceWebhookUpdateInputForm<'_>,
) -> DatabaseResult<()> {
    use schema::workspace_webhooks::dsl::*;

    let filter_cond = id.eq(webhook_id).and(deleted_at.is_null());
    update(workspace_webhooks.filter(filter_cond))
        .set(form)
        .execute(conn)
        .await?;

    Ok(())
}

/// Marks a workspace webhook as deleted.
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

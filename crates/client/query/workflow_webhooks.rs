//! Data layer for workflow webhooks management.

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use uuid::Uuid;

use crate::DatabaseResult;

pub async fn view_workflows_by_webhook(
    conn: &mut AsyncPgConnection,
    form_webhook_id: Uuid,
) -> DatabaseResult<()> {
    Ok(())
}

use axiston_db_schema::schema;
use diesel::dsl::*;
use diesel::prelude::*;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::DatabaseResult;

#[derive(Debug, Clone, Insertable)]
#[diesel(table_name = schema::project_members)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[must_use = "forms do nothing unless you use them"]
pub struct CreateProjectInput<'a> {
    pub display_name: &'a str,
    pub description: &'a str,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[must_use = "jsons do nothing unless you use them"]
pub struct ProjectMetadata {
    pub description: String,
    pub tags: Vec<String>,
}

pub async fn create_project(
    conn: &mut AsyncPgConnection,
    project: CreateProjectInput,
) -> DatabaseResult<Uuid> {
    use schema::projects::dsl::*;

    let metadata = ProjectMetadata {
        description: project.description.to_owned(),
        tags: project.tags,
    };

    let query = insert_into(projects)
        .values((display_name.eq(display_name), metadata.eq(metadata)))
        .returning(id)
        .get_result(conn)
        .await?;

    Ok(query)
}

pub async fn delete_project(conn: &mut AsyncPgConnection) -> DatabaseResult<()> {
    todo!()
}

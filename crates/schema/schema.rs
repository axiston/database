// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "email_type"))]
    pub struct EmailType;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "invite_status"))]
    pub struct InviteStatus;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EmailType;

    account_actions (action_token) {
        action_token -> Uuid,
        account_id -> Nullable<Uuid>,
        email_address -> Text,
        action_type -> EmailType,
        issued_at -> Timestamp,
        expired_at -> Timestamp,
        used_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    account_permissions (account_id) {
        account_id -> Uuid,
        nocheck_read -> Bool,
        nocheck_write -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    account_sessions (account_id, token_seq) {
        token_seq -> Uuid,
        account_id -> Uuid,
        #[max_length = 2]
        region_id -> Bpchar,
        ip_address -> Inet,
        user_agent -> Text,
        issued_at -> Timestamp,
        expired_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    accounts (id) {
        id -> Uuid,
        display_name -> Text,
        email_address -> Text,
        password_hash -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::InviteStatus;

    project_invites (project_id, invite_id) {
        project_id -> Uuid,
        invite_id -> Uuid,
        account_id -> Uuid,
        status -> InviteStatus,
        created_by -> Uuid,
        updated_by -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    project_members (project_id, account_id) {
        project_id -> Uuid,
        account_id -> Uuid,
        show_order -> Int4,
        is_pinned -> Bool,
        is_hidden -> Bool,
        created_by -> Uuid,
        updated_by -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    project_permissions (account_id, project_id) {
        account_id -> Uuid,
        project_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    project_webhooks (unq_path_seq) {
        unq_path_seq -> Uuid,
        project_id -> Uuid,
        success_code -> Int4,
        failure_code -> Int4,
        wait_output -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    projects (id) {
        id -> Uuid,
        display_name -> Text,
        project_meta -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    workflow_executions (workflow_id, execution_id) {
        workflow_id -> Uuid,
        execution_id -> Uuid,
        output_graph -> Jsonb,
        runtime_meta -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    workflow_schedules (workflow_id, schedule_id) {
        workflow_id -> Uuid,
        schedule_id -> Uuid,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    workflow_webhooks (workflow_id, webhook_id) {
        workflow_id -> Uuid,
        webhook_id -> Uuid,
        created_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    workflows (id) {
        id -> Uuid,
        project_id -> Nullable<Uuid>,
        display_name -> Text,
        properties -> Jsonb,
        input_graph -> Jsonb,
        runtime_meta -> Jsonb,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(account_actions -> accounts (account_id));
diesel::joinable!(account_permissions -> accounts (account_id));
diesel::joinable!(account_sessions -> accounts (account_id));
diesel::joinable!(project_invites -> projects (project_id));
diesel::joinable!(project_members -> projects (project_id));
diesel::joinable!(project_permissions -> accounts (account_id));
diesel::joinable!(project_permissions -> projects (project_id));
diesel::joinable!(project_webhooks -> projects (project_id));
diesel::joinable!(workflow_executions -> workflows (workflow_id));
diesel::joinable!(workflow_schedules -> workflows (workflow_id));
diesel::joinable!(workflow_webhooks -> workflows (workflow_id));
diesel::joinable!(workflows -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    account_actions,
    account_permissions,
    account_sessions,
    accounts,
    project_invites,
    project_members,
    project_permissions,
    project_webhooks,
    projects,
    workflow_executions,
    workflow_schedules,
    workflow_webhooks,
    workflows,
);

use async_trait::async_trait;
use chrono::Utc;
use sqlx::MySqlPool;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::{CreateGroup, Group, GroupMember, GroupRole};
use crate::repositories::traits::GroupRepository;

pub struct MySqlGroupRepository {
    pool: MySqlPool,
}

impl MySqlGroupRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GroupRepository for MySqlGroupRepository {
    async fn create(&self, group: CreateGroup) -> AppResult<Group> {
        let new_group = Group::new(group.name, group.description, group.owner_id);

        sqlx::query(
            r#"
            INSERT INTO `groups` (id, name, description, owner_id, invite_code, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(new_group.id.to_string())
        .bind(&new_group.name)
        .bind(&new_group.description)
        .bind(new_group.owner_id.to_string())
        .bind(&new_group.invite_code)
        .bind(new_group.created_at)
        .bind(new_group.updated_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(new_group)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Group>> {
        let result = sqlx::query_as::<_, GroupRow>(
            r#"
            SELECT id, name, description, owner_id, invite_code, created_at, updated_at
            FROM `groups` WHERE id = ?
            "#,
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.map(|r| r.into()))
    }

    async fn find_by_invite_code(&self, code: &str) -> AppResult<Option<Group>> {
        let result = sqlx::query_as::<_, GroupRow>(
            r#"
            SELECT id, name, description, owner_id, invite_code, created_at, updated_at
            FROM `groups` WHERE invite_code = ?
            "#,
        )
        .bind(code)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.map(|r| r.into()))
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Group>> {
        let rows = sqlx::query_as::<_, GroupRow>(
            r#"
            SELECT g.id, g.name, g.description, g.owner_id, g.invite_code, g.created_at, g.updated_at
            FROM `groups` g
            JOIN group_members gm ON g.id = gm.group_id
            WHERE gm.user_id = ?
            ORDER BY g.created_at DESC
            "#,
        )
        .bind(user_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn update(&self, id: Uuid, name: Option<String>, description: Option<String>) -> AppResult<Group> {
        sqlx::query(
            r#"
            UPDATE `groups` SET name = COALESCE(?, name), description = COALESCE(?, description), updated_at = NOW()
            WHERE id = ?
            "#,
        )
        .bind(&name)
        .bind(&description)
        .bind(id.to_string())
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| AppError::NotFound("Group not found".to_string()))
    }

    async fn update_invite_code(&self, id: Uuid, new_code: String) -> AppResult<()> {
        sqlx::query("UPDATE `groups` SET invite_code = ?, updated_at = NOW() WHERE id = ?")
            .bind(&new_code)
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        // Delete members first
        sqlx::query("DELETE FROM group_members WHERE group_id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        sqlx::query("DELETE FROM `groups` WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn transfer_ownership(&self, id: Uuid, new_owner_id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE `groups` SET owner_id = ?, updated_at = NOW() WHERE id = ?")
            .bind(new_owner_id.to_string())
            .bind(id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn add_member(&self, group_id: Uuid, user_id: Uuid, role: GroupRole) -> AppResult<GroupMember> {
        let member = GroupMember {
            id: Uuid::new_v4(),
            group_id,
            user_id,
            role,
            joined_at: Utc::now(),
        };

        sqlx::query(
            r#"
            INSERT INTO group_members (id, group_id, user_id, role, joined_at)
            VALUES (?, ?, ?, ?, ?)
            "#,
        )
        .bind(member.id.to_string())
        .bind(member.group_id.to_string())
        .bind(member.user_id.to_string())
        .bind(member.role.to_string())
        .bind(member.joined_at)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(member)
    }

    async fn remove_member(&self, group_id: Uuid, user_id: Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM group_members WHERE group_id = ? AND user_id = ?")
            .bind(group_id.to_string())
            .bind(user_id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn update_member_role(&self, group_id: Uuid, user_id: Uuid, role: GroupRole) -> AppResult<()> {
        sqlx::query("UPDATE group_members SET role = ? WHERE group_id = ? AND user_id = ?")
            .bind(role.to_string())
            .bind(group_id.to_string())
            .bind(user_id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn get_members(&self, group_id: Uuid) -> AppResult<Vec<GroupMember>> {
        let rows = sqlx::query_as::<_, GroupMemberRow>(
            r#"
            SELECT id, group_id, user_id, role, joined_at
            FROM group_members WHERE group_id = ?
            ORDER BY joined_at ASC
            "#,
        )
        .bind(group_id.to_string())
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn get_member(&self, group_id: Uuid, user_id: Uuid) -> AppResult<Option<GroupMember>> {
        let result = sqlx::query_as::<_, GroupMemberRow>(
            r#"
            SELECT id, group_id, user_id, role, joined_at
            FROM group_members WHERE group_id = ? AND user_id = ?
            "#,
        )
        .bind(group_id.to_string())
        .bind(user_id.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(result.map(|r| r.into()))
    }
}

#[derive(sqlx::FromRow)]
struct GroupRow {
    id: String,
    name: String,
    description: Option<String>,
    owner_id: String,
    invite_code: String,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

impl From<GroupRow> for Group {
    fn from(row: GroupRow) -> Self {
        Group {
            id: Uuid::parse_str(&row.id).unwrap(),
            name: row.name,
            description: row.description,
            owner_id: Uuid::parse_str(&row.owner_id).unwrap(),
            invite_code: row.invite_code,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct GroupMemberRow {
    id: String,
    group_id: String,
    user_id: String,
    role: String,
    joined_at: chrono::DateTime<chrono::Utc>,
}

impl From<GroupMemberRow> for GroupMember {
    fn from(row: GroupMemberRow) -> Self {
        GroupMember {
            id: Uuid::parse_str(&row.id).unwrap(),
            group_id: Uuid::parse_str(&row.group_id).unwrap(),
            user_id: Uuid::parse_str(&row.user_id).unwrap(),
            role: match row.role.as_str() {
                "owner" => GroupRole::Owner,
                "admin" => GroupRole::Admin,
                _ => GroupRole::Member,
            },
            joined_at: row.joined_at,
        }
    }
}

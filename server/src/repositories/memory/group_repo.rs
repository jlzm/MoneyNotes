use async_trait::async_trait;
use chrono::Utc;
use std::collections::HashMap;
use std::sync::RwLock;
use uuid::Uuid;

use crate::error::{AppError, AppResult};
use crate::models::{CreateGroup, Group, GroupMember, GroupRole};
use crate::repositories::traits::GroupRepository;

pub struct MemoryGroupRepository {
    groups: RwLock<HashMap<Uuid, Group>>,
    members: RwLock<HashMap<Uuid, GroupMember>>,
}

impl MemoryGroupRepository {
    pub fn new() -> Self {
        Self {
            groups: RwLock::new(HashMap::new()),
            members: RwLock::new(HashMap::new()),
        }
    }
}

impl Default for MemoryGroupRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl GroupRepository for MemoryGroupRepository {
    async fn create(&self, group: CreateGroup) -> AppResult<Group> {
        let new_group = Group::new(group.name, group.description, group.owner_id);
        let mut groups = self.groups.write().unwrap();
        groups.insert(new_group.id, new_group.clone());
        Ok(new_group)
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Group>> {
        let groups = self.groups.read().unwrap();
        Ok(groups.get(&id).cloned())
    }

    async fn find_by_invite_code(&self, code: &str) -> AppResult<Option<Group>> {
        let groups = self.groups.read().unwrap();
        Ok(groups.values().find(|g| g.invite_code == code).cloned())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Group>> {
        let members = self.members.read().unwrap();
        let group_ids: Vec<Uuid> = members
            .values()
            .filter(|m| m.user_id == user_id)
            .map(|m| m.group_id)
            .collect();

        let groups = self.groups.read().unwrap();
        Ok(group_ids
            .iter()
            .filter_map(|id| groups.get(id).cloned())
            .collect())
    }

    async fn update(&self, id: Uuid, name: Option<String>, description: Option<String>) -> AppResult<Group> {
        let mut groups = self.groups.write().unwrap();
        let group = groups.get_mut(&id).ok_or_else(|| AppError::NotFound("Group not found".to_string()))?;

        if let Some(n) = name {
            group.name = n;
        }
        if let Some(d) = description {
            group.description = Some(d);
        }
        group.updated_at = Utc::now();

        Ok(group.clone())
    }

    async fn update_invite_code(&self, id: Uuid, new_code: String) -> AppResult<()> {
        let mut groups = self.groups.write().unwrap();
        let group = groups.get_mut(&id).ok_or_else(|| AppError::NotFound("Group not found".to_string()))?;
        group.invite_code = new_code;
        group.updated_at = Utc::now();
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        // Delete members first
        {
            let mut members = self.members.write().unwrap();
            members.retain(|_, m| m.group_id != id);
        }
        let mut groups = self.groups.write().unwrap();
        groups.remove(&id);
        Ok(())
    }

    async fn transfer_ownership(&self, id: Uuid, new_owner_id: Uuid) -> AppResult<()> {
        let mut groups = self.groups.write().unwrap();
        let group = groups.get_mut(&id).ok_or_else(|| AppError::NotFound("Group not found".to_string()))?;
        group.owner_id = new_owner_id;
        group.updated_at = Utc::now();
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
        let mut members = self.members.write().unwrap();
        members.insert(member.id, member.clone());
        Ok(member)
    }

    async fn remove_member(&self, group_id: Uuid, user_id: Uuid) -> AppResult<()> {
        let mut members = self.members.write().unwrap();
        members.retain(|_, m| !(m.group_id == group_id && m.user_id == user_id));
        Ok(())
    }

    async fn update_member_role(&self, group_id: Uuid, user_id: Uuid, role: GroupRole) -> AppResult<()> {
        let mut members = self.members.write().unwrap();
        for member in members.values_mut() {
            if member.group_id == group_id && member.user_id == user_id {
                member.role = role;
                return Ok(());
            }
        }
        Err(AppError::NotFound("Member not found".to_string()))
    }

    async fn get_members(&self, group_id: Uuid) -> AppResult<Vec<GroupMember>> {
        let members = self.members.read().unwrap();
        Ok(members
            .values()
            .filter(|m| m.group_id == group_id)
            .cloned()
            .collect())
    }

    async fn get_member(&self, group_id: Uuid, user_id: Uuid) -> AppResult<Option<GroupMember>> {
        let members = self.members.read().unwrap();
        Ok(members
            .values()
            .find(|m| m.group_id == group_id && m.user_id == user_id)
            .cloned())
    }
}

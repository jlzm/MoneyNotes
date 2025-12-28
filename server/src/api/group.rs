use axum::{
    extract::{Path, State},
    Extension, Json,
};
use std::sync::Arc;
use uuid::Uuid;

use crate::dto::{
    ApiResponse, CreateGroupRequest, CreateGroupResponse, GroupDetailResponse,
    GroupListResponse, GroupMemberResponse, GroupResponse, JoinGroupRequest,
    LedgerBriefResponse, TransferGroupRequest, UpdateMemberRoleRequest, UserBriefResponse,
};
use crate::error::{AppError, AppResult};
use crate::middleware::CurrentUser;
use crate::models::{CreateGroup, GroupRole};
use crate::repositories::{GroupRepository, LedgerRepository, UserRepository};

pub struct GroupApi {
    group_repo: Arc<dyn GroupRepository>,
    user_repo: Arc<dyn UserRepository>,
    ledger_repo: Arc<dyn LedgerRepository>,
}

impl GroupApi {
    pub fn new(
        group_repo: Arc<dyn GroupRepository>,
        user_repo: Arc<dyn UserRepository>,
        ledger_repo: Arc<dyn LedgerRepository>,
    ) -> Self {
        Self {
            group_repo,
            user_repo,
            ledger_repo,
        }
    }
}

pub async fn list_groups(
    State(api): State<Arc<GroupApi>>,
    Extension(current_user): Extension<CurrentUser>,
) -> AppResult<Json<ApiResponse<GroupListResponse>>> {
    let groups = api.group_repo.find_by_user_id(current_user.id).await?;

    let mut items = Vec::new();
    for group in groups {
        let members = api.group_repo.get_members(group.id).await?;
        let my_member = members.iter().find(|m| m.user_id == current_user.id);

        items.push(GroupResponse {
            id: group.id.to_string(),
            name: group.name,
            description: group.description,
            member_count: members.len(),
            my_role: my_member.map(|m| m.role.to_string()).unwrap_or_else(|| "member".to_string()),
            created_at: group.created_at.to_rfc3339(),
        });
    }

    Ok(Json(ApiResponse::success(GroupListResponse { items })))
}

pub async fn create_group(
    State(api): State<Arc<GroupApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(req): Json<CreateGroupRequest>,
) -> AppResult<Json<ApiResponse<CreateGroupResponse>>> {
    let group = api
        .group_repo
        .create(CreateGroup {
            name: req.name.clone(),
            description: req.description,
            owner_id: current_user.id,
        })
        .await?;

    // Add owner as member
    api.group_repo
        .add_member(group.id, current_user.id, GroupRole::Owner)
        .await?;

    Ok(Json(ApiResponse::success(CreateGroupResponse {
        id: group.id.to_string(),
        name: group.name,
        invite_code: group.invite_code,
    })))
}

pub async fn get_group(
    State(api): State<Arc<GroupApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<GroupDetailResponse>>> {
    let group_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::Validation("Invalid group ID".to_string()))?;

    let group = api
        .group_repo
        .find_by_id(group_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Group not found".to_string()))?;

    // Check membership
    let my_member = api.group_repo.get_member(group_id, current_user.id).await?;
    if my_member.is_none() {
        return Err(AppError::Forbidden("Not a member of this group".to_string()));
    }

    let members = api.group_repo.get_members(group_id).await?;
    let owner = api.user_repo.find_by_id(group.owner_id).await?.unwrap();
    let ledgers = api.ledger_repo.find_by_group_id(group_id).await?;

    let member_responses: Vec<GroupMemberResponse> = {
        let mut responses = Vec::new();
        for member in members {
            let user = api.user_repo.find_by_id(member.user_id).await?.unwrap();
            responses.push(GroupMemberResponse {
                user_id: user.id.to_string(),
                nickname: user.nickname,
                avatar: user.avatar,
                role: member.role.to_string(),
                joined_at: member.joined_at.to_rfc3339(),
            });
        }
        responses
    };

    // Only show invite code to owner/admin
    let invite_code = if my_member.as_ref().map(|m| m.role == GroupRole::Owner || m.role == GroupRole::Admin).unwrap_or(false) {
        Some(group.invite_code)
    } else {
        None
    };

    Ok(Json(ApiResponse::success(GroupDetailResponse {
        id: group.id.to_string(),
        name: group.name,
        description: group.description,
        owner: UserBriefResponse {
            id: owner.id.to_string(),
            nickname: owner.nickname,
        },
        members: member_responses,
        ledgers: ledgers
            .iter()
            .map(|l| LedgerBriefResponse {
                id: l.id.to_string(),
                name: l.name.clone(),
            })
            .collect(),
        invite_code,
    })))
}

pub async fn update_group(
    State(api): State<Arc<GroupApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
    Json(req): Json<UpdateGroupRequest>,
) -> AppResult<Json<ApiResponse<GroupResponse>>> {
    let group_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::Validation("Invalid group ID".to_string()))?;

    // Check permission (owner or admin)
    let member = api
        .group_repo
        .get_member(group_id, current_user.id)
        .await?
        .ok_or_else(|| AppError::Forbidden("Not a member".to_string()))?;

    if member.role != GroupRole::Owner && member.role != GroupRole::Admin {
        return Err(AppError::Forbidden("Only owner or admin can update group".to_string()));
    }

    let group = api.group_repo.update(group_id, req.name, req.description).await?;
    let members = api.group_repo.get_members(group_id).await?;

    Ok(Json(ApiResponse::success(GroupResponse {
        id: group.id.to_string(),
        name: group.name,
        description: group.description,
        member_count: members.len(),
        my_role: member.role.to_string(),
        created_at: group.created_at.to_rfc3339(),
    })))
}

pub async fn delete_group(
    State(api): State<Arc<GroupApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let group_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::Validation("Invalid group ID".to_string()))?;

    let group = api
        .group_repo
        .find_by_id(group_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Group not found".to_string()))?;

    if group.owner_id != current_user.id {
        return Err(AppError::Forbidden("Only owner can delete group".to_string()));
    }

    api.group_repo.delete(group_id).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn join_group(
    State(api): State<Arc<GroupApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Json(req): Json<JoinGroupRequest>,
) -> AppResult<Json<ApiResponse<GroupResponse>>> {
    let group = api
        .group_repo
        .find_by_invite_code(&req.invite_code)
        .await?
        .ok_or_else(|| AppError::NotFound("Invalid invite code".to_string()))?;

    // Check if already a member
    if api.group_repo.get_member(group.id, current_user.id).await?.is_some() {
        return Err(AppError::Conflict("Already a member".to_string()));
    }

    api.group_repo.add_member(group.id, current_user.id, GroupRole::Member).await?;

    let members = api.group_repo.get_members(group.id).await?;

    Ok(Json(ApiResponse::success(GroupResponse {
        id: group.id.to_string(),
        name: group.name,
        description: group.description,
        member_count: members.len(),
        my_role: "member".to_string(),
        created_at: group.created_at.to_rfc3339(),
    })))
}

pub async fn leave_group(
    State(api): State<Arc<GroupApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<()>>> {
    let group_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::Validation("Invalid group ID".to_string()))?;

    let group = api
        .group_repo
        .find_by_id(group_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Group not found".to_string()))?;

    if group.owner_id == current_user.id {
        return Err(AppError::Forbidden("Owner cannot leave. Transfer ownership first.".to_string()));
    }

    api.group_repo.remove_member(group_id, current_user.id).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn remove_member(
    State(api): State<Arc<GroupApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Path((group_id, user_id)): Path<(String, String)>,
) -> AppResult<Json<ApiResponse<()>>> {
    let group_id = Uuid::parse_str(&group_id)
        .map_err(|_| AppError::Validation("Invalid group ID".to_string()))?;
    let user_id = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;

    // Check permission
    let my_member = api
        .group_repo
        .get_member(group_id, current_user.id)
        .await?
        .ok_or_else(|| AppError::Forbidden("Not a member".to_string()))?;

    if my_member.role != GroupRole::Owner && my_member.role != GroupRole::Admin {
        return Err(AppError::Forbidden("Only owner or admin can remove members".to_string()));
    }

    // Cannot remove owner
    let group = api.group_repo.find_by_id(group_id).await?.unwrap();
    if group.owner_id == user_id {
        return Err(AppError::Forbidden("Cannot remove owner".to_string()));
    }

    api.group_repo.remove_member(group_id, user_id).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn update_member_role(
    State(api): State<Arc<GroupApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Path((group_id, user_id)): Path<(String, String)>,
    Json(req): Json<UpdateMemberRoleRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    let group_id = Uuid::parse_str(&group_id)
        .map_err(|_| AppError::Validation("Invalid group ID".to_string()))?;
    let user_id = Uuid::parse_str(&user_id)
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;

    // Only owner can change roles
    let group = api
        .group_repo
        .find_by_id(group_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Group not found".to_string()))?;

    if group.owner_id != current_user.id {
        return Err(AppError::Forbidden("Only owner can change roles".to_string()));
    }

    let role = match req.role.as_str() {
        "admin" => GroupRole::Admin,
        "member" => GroupRole::Member,
        _ => return Err(AppError::Validation("Invalid role".to_string())),
    };

    api.group_repo.update_member_role(group_id, user_id, role).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn transfer_group(
    State(api): State<Arc<GroupApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
    Json(req): Json<TransferGroupRequest>,
) -> AppResult<Json<ApiResponse<()>>> {
    let group_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::Validation("Invalid group ID".to_string()))?;
    let new_owner_id = Uuid::parse_str(&req.new_owner_id)
        .map_err(|_| AppError::Validation("Invalid user ID".to_string()))?;

    let group = api
        .group_repo
        .find_by_id(group_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Group not found".to_string()))?;

    if group.owner_id != current_user.id {
        return Err(AppError::Forbidden("Only owner can transfer".to_string()));
    }

    // Check new owner is a member
    if api.group_repo.get_member(group_id, new_owner_id).await?.is_none() {
        return Err(AppError::Validation("New owner must be a member".to_string()));
    }

    api.group_repo.transfer_ownership(group_id, new_owner_id).await?;
    api.group_repo.update_member_role(group_id, new_owner_id, GroupRole::Owner).await?;
    api.group_repo.update_member_role(group_id, current_user.id, GroupRole::Admin).await?;

    Ok(Json(ApiResponse::success(())))
}

pub async fn reset_invite_code(
    State(api): State<Arc<GroupApi>>,
    Extension(current_user): Extension<CurrentUser>,
    Path(id): Path<String>,
) -> AppResult<Json<ApiResponse<InviteCodeResponse>>> {
    let group_id = Uuid::parse_str(&id)
        .map_err(|_| AppError::Validation("Invalid group ID".to_string()))?;

    // Check permission
    let member = api
        .group_repo
        .get_member(group_id, current_user.id)
        .await?
        .ok_or_else(|| AppError::Forbidden("Not a member".to_string()))?;

    if member.role != GroupRole::Owner && member.role != GroupRole::Admin {
        return Err(AppError::Forbidden("Only owner or admin can reset invite code".to_string()));
    }

    let new_code = generate_invite_code();
    api.group_repo.update_invite_code(group_id, new_code.clone()).await?;

    Ok(Json(ApiResponse::success(InviteCodeResponse {
        invite_code: new_code,
    })))
}

fn generate_invite_code() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    (0..6)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

#[derive(Debug, serde::Deserialize)]
pub struct UpdateGroupRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct InviteCodeResponse {
    pub invite_code: String,
}

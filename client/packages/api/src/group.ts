import { get, post, put, del } from './request'

export interface Group {
  id: string
  name: string
  description?: string
  memberCount: number
  myRole: 'owner' | 'admin' | 'member'
  createdAt: string
}

export interface GroupDetail extends Omit<Group, 'memberCount' | 'myRole'> {
  owner: {
    id: string
    nickname?: string
  }
  members: Array<{
    userId: string
    nickname?: string
    avatar?: string
    role: 'owner' | 'admin' | 'member'
    joinedAt: string
  }>
  ledgers: Array<{
    id: string
    name: string
  }>
  inviteCode?: string
}

export function getGroups() {
  return get<{ items: Group[] }>('/groups')
}

export function getGroup(id: string) {
  return get<GroupDetail>(`/groups/${id}`)
}

export function createGroup(data: { name: string; description?: string }) {
  return post<{ id: string; name: string; inviteCode: string }>('/groups', data)
}

export function updateGroup(id: string, data: { name?: string; description?: string }) {
  return put(`/groups/${id}`, data)
}

export function deleteGroup(id: string) {
  return del(`/groups/${id}`)
}

export function joinGroup(inviteCode: string) {
  return post('/groups/join', { inviteCode })
}

export function leaveGroup(id: string) {
  return post(`/groups/${id}/leave`)
}

export function resetInviteCode(id: string) {
  return post<{ inviteCode: string }>(`/groups/${id}/invite-code`)
}

export function removeMember(groupId: string, userId: string) {
  return del(`/groups/${groupId}/members/${userId}`)
}

export function updateMemberRole(groupId: string, userId: string, role: 'admin' | 'member') {
  return put(`/groups/${groupId}/members/${userId}/role`, { role })
}

export function transferGroup(id: string, newOwnerId: string) {
  return post(`/groups/${id}/transfer`, { newOwnerId })
}

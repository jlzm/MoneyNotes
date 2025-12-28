<template>
  <view class="container">
    <view class="header">
      <text class="title">我的群组</text>
      <view class="header-actions">
        <button class="btn-create" @click="showCreateModal = true">创建</button>
        <button class="btn-join" @click="showJoinModal = true">加入</button>
      </view>
    </view>

    <view class="group-list" v-if="groups.length > 0">
      <view class="group-item" v-for="group in groups" :key="group.id">
        <view class="group-info">
          <text class="group-name">{{ group.name }}</text>
          <text class="group-members">{{ group.memberCount }} 位成员</text>
        </view>
        <view class="group-role">{{ getRoleText(group.myRole) }}</view>
      </view>
    </view>

    <view class="empty-state" v-else>
      <view class="empty-icon">👥</view>
      <text class="empty-text">暂无群组</text>
      <text class="empty-hint">创建或加入群组，开启共享记账</text>
    </view>

    <!-- 创建群组弹窗 -->
    <view class="modal" v-if="showCreateModal" @click="showCreateModal = false">
      <view class="modal-content" @click.stop>
        <text class="modal-title">创建群组</text>
        <input
          class="modal-input"
          type="text"
          placeholder="群组名称"
          v-model="newGroupName"
        />
        <view class="modal-actions">
          <button class="btn-cancel" @click="showCreateModal = false">取消</button>
          <button class="btn-confirm" @click="createGroup">创建</button>
        </view>
      </view>
    </view>

    <!-- 加入群组弹窗 -->
    <view class="modal" v-if="showJoinModal" @click="showJoinModal = false">
      <view class="modal-content" @click.stop>
        <text class="modal-title">加入群组</text>
        <input
          class="modal-input"
          type="text"
          placeholder="输入邀请码"
          v-model="inviteCode"
        />
        <view class="modal-actions">
          <button class="btn-cancel" @click="showJoinModal = false">取消</button>
          <button class="btn-confirm" @click="joinGroup">加入</button>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useUserStore } from '@/store/user'

const userStore = useUserStore()

const groups = ref<any[]>([])
const showCreateModal = ref(false)
const showJoinModal = ref(false)
const newGroupName = ref('')
const inviteCode = ref('')

function getRoleText(role: string) {
  const roleMap: Record<string, string> = {
    owner: '群主',
    admin: '管理员',
    member: '成员'
  }
  return roleMap[role] || '成员'
}

function createGroup() {
  if (!newGroupName.value.trim()) {
    uni.showToast({ title: '请输入群组名称', icon: 'none' })
    return
  }

  if (userStore.isGuest) {
    uni.showToast({ title: '请先登录', icon: 'none' })
    return
  }

  // TODO: 调用API创建群组
  uni.showToast({ title: '创建成功', icon: 'success' })
  showCreateModal.value = false
  newGroupName.value = ''
}

function joinGroup() {
  if (!inviteCode.value.trim()) {
    uni.showToast({ title: '请输入邀请码', icon: 'none' })
    return
  }

  if (userStore.isGuest) {
    uni.showToast({ title: '请先登录', icon: 'none' })
    return
  }

  // TODO: 调用API加入群组
  uni.showToast({ title: '加入成功', icon: 'success' })
  showJoinModal.value = false
  inviteCode.value = ''
}
</script>

<style scoped>
.container {
  padding: 20rpx;
  min-height: 100vh;
  background: #f5f5f5;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20rpx 0;
}

.title {
  font-size: 36rpx;
  font-weight: 600;
}

.header-actions {
  display: flex;
  gap: 20rpx;
}

.btn-create,
.btn-join {
  padding: 12rpx 24rpx;
  font-size: 26rpx;
  border-radius: 20rpx;
}

.btn-create {
  background: #667eea;
  color: #fff;
}

.btn-join {
  background: #fff;
  color: #667eea;
  border: 1rpx solid #667eea;
}

.group-list {
  margin-top: 20rpx;
}

.group-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 30rpx;
  background: #fff;
  border-radius: 16rpx;
  margin-bottom: 20rpx;
}

.group-name {
  font-size: 32rpx;
  font-weight: 500;
  display: block;
}

.group-members {
  font-size: 24rpx;
  color: #999;
  display: block;
  margin-top: 8rpx;
}

.group-role {
  font-size: 24rpx;
  color: #667eea;
  background: #f0f3ff;
  padding: 8rpx 16rpx;
  border-radius: 20rpx;
}

.empty-state {
  padding: 100rpx 0;
  text-align: center;
}

.empty-icon {
  font-size: 80rpx;
  margin-bottom: 20rpx;
}

.empty-text {
  font-size: 32rpx;
  color: #666;
  display: block;
}

.empty-hint {
  font-size: 26rpx;
  color: #999;
  display: block;
  margin-top: 16rpx;
}

.modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 100;
}

.modal-content {
  width: 80%;
  background: #fff;
  border-radius: 20rpx;
  padding: 40rpx;
}

.modal-title {
  font-size: 36rpx;
  font-weight: 600;
  text-align: center;
  display: block;
  margin-bottom: 30rpx;
}

.modal-input {
  width: 100%;
  padding: 24rpx;
  background: #f5f5f5;
  border-radius: 12rpx;
  font-size: 28rpx;
  margin-bottom: 30rpx;
}

.modal-actions {
  display: flex;
  gap: 20rpx;
}

.btn-cancel,
.btn-confirm {
  flex: 1;
  padding: 24rpx;
  font-size: 30rpx;
  border-radius: 12rpx;
}

.btn-cancel {
  background: #f5f5f5;
  color: #666;
}

.btn-confirm {
  background: #667eea;
  color: #fff;
}
</style>

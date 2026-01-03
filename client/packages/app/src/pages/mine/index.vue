<template>
  <view class="container">
    <!-- 用户信息卡片 -->
    <view class="user-card">
      <view class="avatar">
        <text v-if="!userStore.isLoggedIn">👤</text>
        <image v-else :src="userStore.user?.avatar || ''" mode="aspectFill" />
      </view>
      <view class="user-info">
        <text class="nickname">{{ displayName }}</text>
        <text class="status">{{ userStore.isLoggedIn ? '已登录' : '访客模式' }}</text>
      </view>
      <button class="btn-login" v-if="!userStore.isLoggedIn" @click="goToLogin">
        登录/注册
      </button>
    </view>

    <!-- 功能列表 -->
    <view class="menu-section">
      <view class="menu-item" @click="navigateTo('/pages/ledger/list')">
        <text class="menu-icon">📚</text>
        <text class="menu-text">我的账本</text>
        <text class="menu-arrow">›</text>
      </view>
      <view class="menu-item" @click="navigateTo('/pages/category/index')">
        <text class="menu-icon">🏷️</text>
        <text class="menu-text">分类管理</text>
        <text class="menu-arrow">›</text>
      </view>
      <view class="menu-item" @click="exportData">
        <text class="menu-icon">📤</text>
        <text class="menu-text">导出数据</text>
        <text class="menu-arrow">›</text>
      </view>
    </view>

    <view class="menu-section">
      <view class="menu-item" @click="navigateTo('/pages/settings/index')">
        <text class="menu-icon">⚙️</text>
        <text class="menu-text">设置</text>
        <text class="menu-arrow">›</text>
      </view>
      <view class="menu-item" @click="showAbout">
        <text class="menu-icon">ℹ️</text>
        <text class="menu-text">关于</text>
        <text class="menu-arrow">›</text>
      </view>
    </view>

    <!-- 登录用户显示退出按钮 -->
    <view class="logout-section" v-if="userStore.isLoggedIn">
      <button class="btn-logout" @click="logout">退出登录</button>
    </view>

    <!-- 访客模式提示 -->
    <view class="guest-tip" v-if="!userStore.isLoggedIn">
      <text class="tip-text">当前为访客模式，数据仅保存在本地</text>
      <text class="tip-text">登录后可同步数据、使用群组功能</text>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useUserStore } from '@/store/user'

const userStore = useUserStore()

const displayName = computed(() => {
  if (userStore.isLoggedIn && userStore.user) {
    return userStore.user.nickname || userStore.user.email
  }
  return '访客'
})

function goToLogin() {
  uni.navigateTo({ url: '/pages/login/index' })
}

function navigateTo(url: string) {
  uni.navigateTo({ url })
}

function exportData() {
  uni.showToast({ title: '功能开发中', icon: 'none' })
}

function showAbout() {
  uni.showModal({
    title: 'Money Notes',
    content: '版本 1.0.0\n一个简单好用的记账应用',
    showCancel: false
  })
}

function logout() {
  uni.showModal({
    title: '提示',
    content: '确定要退出登录吗？',
    success: (res) => {
      if (res.confirm) {
        userStore.logout()
        uni.showToast({ title: '已退出登录', icon: 'success' })
      }
    }
  })
}
</script>

<style scoped>
.container {
  min-height: 100vh;
  background: #f5f5f5;
  padding: 20rpx;
}

.user-card {
  display: flex;
  align-items: center;
  padding: 40rpx 30rpx;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 20rpx;
  margin-bottom: 30rpx;
}

.avatar {
  width: 100rpx;
  height: 100rpx;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 48rpx;
  margin-right: 24rpx;
  overflow: hidden;
}

.avatar image {
  width: 100%;
  height: 100%;
}

.user-info {
  flex: 1;
}

.nickname {
  font-size: 36rpx;
  font-weight: 600;
  color: #fff;
  display: block;
}

.status {
  font-size: 24rpx;
  color: rgba(255, 255, 255, 0.8);
  display: block;
  margin-top: 8rpx;
}

.btn-login {
  padding: 16rpx 32rpx;
  background: #fff;
  color: #667eea;
  font-size: 26rpx;
  border-radius: 30rpx;
}

.menu-section {
  background: #fff;
  border-radius: 20rpx;
  margin-bottom: 20rpx;
  overflow: hidden;
}

.menu-item {
  display: flex;
  align-items: center;
  padding: 30rpx;
  border-bottom: 1rpx solid #f5f5f5;
}

.menu-item:last-child {
  border-bottom: none;
}

.menu-icon {
  font-size: 40rpx;
  margin-right: 20rpx;
}

.menu-text {
  flex: 1;
  font-size: 30rpx;
  color: #333;
}

.menu-arrow {
  font-size: 36rpx;
  color: #ccc;
}

.logout-section {
  margin-top: 40rpx;
}

.btn-logout {
  width: 100%;
  padding: 28rpx;
  background: #fff;
  color: #ff4d4f;
  font-size: 30rpx;
  border-radius: 20rpx;
}

.guest-tip {
  margin-top: 40rpx;
  text-align: center;
}

.tip-text {
  display: block;
  font-size: 24rpx;
  color: #999;
  line-height: 1.8;
}
</style>

<template>
  <view class="container">
    <view class="header">
      <text class="title">登录</text>
      <text class="subtitle">登录后可同步数据、使用群组功能</text>
    </view>

    <view class="form">
      <view class="form-item">
        <input
          class="input"
          type="text"
          placeholder="邮箱"
          v-model="email"
        />
      </view>
      <view class="form-item">
        <input
          class="input"
          type="password"
          placeholder="密码"
          v-model="password"
        />
      </view>

      <button class="btn-submit" @click="handleLogin">登录</button>

      <view class="divider">
        <text class="divider-text">或</text>
      </view>

      <button class="btn-register" @click="toggleMode">
        {{ isLogin ? '注册新账号' : '返回登录' }}
      </button>
    </view>

    <view class="guest-action">
      <button class="btn-guest" @click="continueAsGuest">继续以访客身份使用</button>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useUserStore } from '@/store/user'

const userStore = useUserStore()

const isLogin = ref(true)
const email = ref('')
const password = ref('')
const nickname = ref('')

function toggleMode() {
  isLogin.value = !isLogin.value
}

async function handleLogin() {
  if (!email.value || !password.value) {
    uni.showToast({ title: '请填写完整信息', icon: 'none' })
    return
  }

  // TODO: 调用API登录/注册
  // 模拟登录成功
  userStore.setUser(
    {
      id: '1',
      email: email.value,
      nickname: nickname.value || email.value.split('@')[0]
    },
    {
      accessToken: 'mock_access_token',
      refreshToken: 'mock_refresh_token'
    }
  )

  uni.showToast({ title: isLogin.value ? '登录成功' : '注册成功', icon: 'success' })
  uni.switchTab({ url: '/pages/index/index' })
}

function continueAsGuest() {
  uni.navigateBack()
}
</script>

<style scoped>
.container {
  min-height: 100vh;
  background: #fff;
  padding: 60rpx 40rpx;
}

.header {
  margin-bottom: 60rpx;
}

.title {
  font-size: 48rpx;
  font-weight: 600;
  display: block;
  margin-bottom: 16rpx;
}

.subtitle {
  font-size: 28rpx;
  color: #999;
}

.form {
  margin-bottom: 40rpx;
}

.form-item {
  margin-bottom: 30rpx;
}

.input {
  width: 100%;
  padding: 30rpx;
  background: #f5f5f5;
  border-radius: 16rpx;
  font-size: 30rpx;
}

.btn-submit {
  width: 100%;
  padding: 30rpx;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #fff;
  font-size: 32rpx;
  border-radius: 16rpx;
  margin-top: 20rpx;
}

.divider {
  display: flex;
  align-items: center;
  margin: 40rpx 0;
}

.divider::before,
.divider::after {
  content: '';
  flex: 1;
  height: 1rpx;
  background: #e0e0e0;
}

.divider-text {
  padding: 0 30rpx;
  color: #999;
  font-size: 26rpx;
}

.btn-register {
  width: 100%;
  padding: 30rpx;
  background: #fff;
  color: #667eea;
  font-size: 32rpx;
  border: 2rpx solid #667eea;
  border-radius: 16rpx;
}

.guest-action {
  position: fixed;
  bottom: 60rpx;
  left: 40rpx;
  right: 40rpx;
}

.btn-guest {
  width: 100%;
  padding: 24rpx;
  background: transparent;
  color: #999;
  font-size: 28rpx;
}
</style>

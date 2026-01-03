<template>
  <view class="container">
    <!-- 账户设置 -->
    <view class="section">
      <text class="section-title">账户设置</text>
      <view class="setting-item" @click="editNickname">
        <text class="setting-label">昵称</text>
        <view class="setting-value">
          <text>{{ userStore.user?.nickname || '未设置' }}</text>
          <text class="arrow">›</text>
        </view>
      </view>
      <view class="setting-item" @click="editAvatar">
        <text class="setting-label">头像</text>
        <view class="setting-value">
          <view class="avatar-preview">
            <text v-if="!userStore.user?.avatar">👤</text>
            <image v-else :src="userStore.user.avatar" mode="aspectFill" />
          </view>
          <text class="arrow">›</text>
        </view>
      </view>
    </view>

    <!-- 通用设置 -->
    <view class="section">
      <text class="section-title">通用设置</text>
      <view class="setting-item">
        <text class="setting-label">默认账本</text>
        <view class="setting-value" @click="goToLedgerList">
          <text>{{ ledgerStore.currentLedger?.name || '未选择' }}</text>
          <text class="arrow">›</text>
        </view>
      </view>
      <view class="setting-item">
        <text class="setting-label">货币单位</text>
        <view class="setting-value">
          <picker :value="currencyIndex" :range="currencies" @change="onCurrencyChange">
            <text>{{ currencies[currencyIndex] }}</text>
            <text class="arrow">›</text>
          </picker>
        </view>
      </view>
    </view>

    <!-- 数据管理 -->
    <view class="section">
      <text class="section-title">数据管理</text>
      <view class="setting-item" @click="clearCache">
        <text class="setting-label">清除缓存</text>
        <view class="setting-value">
          <text class="cache-size">{{ cacheSize }}</text>
          <text class="arrow">›</text>
        </view>
      </view>
      <view class="setting-item" @click="exportData">
        <text class="setting-label">导出数据</text>
        <view class="setting-value">
          <text class="arrow">›</text>
        </view>
      </view>
      <view class="setting-item danger" @click="clearAllData">
        <text class="setting-label">清除所有数据</text>
        <view class="setting-value">
          <text class="arrow">›</text>
        </view>
      </view>
    </view>

    <!-- 关于 -->
    <view class="section">
      <text class="section-title">关于</text>
      <view class="setting-item">
        <text class="setting-label">版本</text>
        <view class="setting-value">
          <text>1.0.0</text>
        </view>
      </view>
      <view class="setting-item" @click="checkUpdate">
        <text class="setting-label">检查更新</text>
        <view class="setting-value">
          <text class="arrow">›</text>
        </view>
      </view>
      <view class="setting-item" @click="showFeedback">
        <text class="setting-label">意见反馈</text>
        <view class="setting-value">
          <text class="arrow">›</text>
        </view>
      </view>
    </view>

    <!-- 昵称编辑弹窗 -->
    <view class="modal" v-if="showNicknameModal" @click="closeNicknameModal">
      <view class="modal-content" @click.stop>
        <text class="modal-title">修改昵称</text>
        <input
          class="modal-input"
          v-model="newNickname"
          placeholder="请输入昵称"
          maxlength="20"
        />
        <view class="modal-buttons">
          <button class="btn-cancel" @click="closeNicknameModal">取消</button>
          <button class="btn-confirm" @click="saveNickname" :disabled="!newNickname.trim()">保存</button>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useUserStore } from '@/store/user'
import { useLedgerStore } from '@/store/ledger'

const userStore = useUserStore()
const ledgerStore = useLedgerStore()

const currencies = ['CNY (人民币)', 'USD (美元)', 'EUR (欧元)', 'JPY (日元)', 'GBP (英镑)']
const currencyIndex = ref(0)
const cacheSize = ref('计算中...')

const showNicknameModal = ref(false)
const newNickname = ref('')

onMounted(() => {
  loadSettings()
  calculateCacheSize()
})

function loadSettings() {
  const savedCurrency = uni.getStorageSync('currency')
  if (savedCurrency) {
    const index = currencies.findIndex(c => c.startsWith(savedCurrency))
    if (index !== -1) currencyIndex.value = index
  }
}

function calculateCacheSize() {
  try {
    const info = uni.getStorageInfoSync()
    const sizeKB = info.currentSize
    if (sizeKB < 1024) {
      cacheSize.value = `${sizeKB} KB`
    } else {
      cacheSize.value = `${(sizeKB / 1024).toFixed(2)} MB`
    }
  } catch (e) {
    cacheSize.value = '未知'
  }
}

function editNickname() {
  if (!userStore.isLoggedIn) {
    uni.showToast({ title: '请先登录', icon: 'none' })
    return
  }
  newNickname.value = userStore.user?.nickname || ''
  showNicknameModal.value = true
}

function closeNicknameModal() {
  showNicknameModal.value = false
}

function saveNickname() {
  if (!newNickname.value.trim()) return

  // 更新本地存储的用户信息
  if (userStore.user) {
    const updatedUser = { ...userStore.user, nickname: newNickname.value.trim() }
    uni.setStorageSync('user', JSON.stringify(updatedUser))
    userStore.user.nickname = newNickname.value.trim()
    uni.showToast({ title: '昵称已更新', icon: 'success' })
  }
  closeNicknameModal()
}

function editAvatar() {
  if (!userStore.isLoggedIn) {
    uni.showToast({ title: '请先登录', icon: 'none' })
    return
  }
  uni.showToast({ title: '功能开发中', icon: 'none' })
}

function goToLedgerList() {
  uni.navigateTo({ url: '/pages/ledger/list' })
}

function onCurrencyChange(e: any) {
  currencyIndex.value = e.detail.value
  const currencyCode = currencies[currencyIndex.value].split(' ')[0]
  uni.setStorageSync('currency', currencyCode)
  uni.showToast({ title: '货币单位已更新', icon: 'success' })
}

function clearCache() {
  uni.showModal({
    title: '清除缓存',
    content: '确定要清除缓存吗？这不会删除您的账单数据。',
    success: (res) => {
      if (res.confirm) {
        // 保留重要数据
        const user = uni.getStorageSync('user')
        const accessToken = uni.getStorageSync('accessToken')
        const refreshToken = uni.getStorageSync('refreshToken')
        const localBills = uni.getStorageSync('localBills')
        const currentLedgerId = uni.getStorageSync('currentLedgerId')

        uni.clearStorageSync()

        // 恢复重要数据
        if (user) uni.setStorageSync('user', user)
        if (accessToken) uni.setStorageSync('accessToken', accessToken)
        if (refreshToken) uni.setStorageSync('refreshToken', refreshToken)
        if (localBills) uni.setStorageSync('localBills', localBills)
        if (currentLedgerId) uni.setStorageSync('currentLedgerId', currentLedgerId)

        calculateCacheSize()
        uni.showToast({ title: '缓存已清除', icon: 'success' })
      }
    }
  })
}

function exportData() {
  uni.showToast({ title: '功能开发中', icon: 'none' })
}

function clearAllData() {
  uni.showModal({
    title: '警告',
    content: '确定要清除所有数据吗？此操作不可恢复！',
    confirmColor: '#ff4d4f',
    success: (res) => {
      if (res.confirm) {
        uni.showModal({
          title: '再次确认',
          content: '所有本地数据将被删除，包括账单、分类等。确定继续吗？',
          confirmColor: '#ff4d4f',
          success: (res2) => {
            if (res2.confirm) {
              uni.clearStorageSync()
              userStore.logout()
              uni.showToast({ title: '数据已清除', icon: 'success' })
              setTimeout(() => {
                uni.reLaunch({ url: '/pages/index/index' })
              }, 1500)
            }
          }
        })
      }
    }
  })
}

function checkUpdate() {
  uni.showToast({ title: '已是最新版本', icon: 'success' })
}

function showFeedback() {
  uni.showModal({
    title: '意见反馈',
    content: '如有问题或建议，请联系开发者',
    showCancel: false
  })
}
</script>

<style scoped>
.container {
  min-height: 100vh;
  background: #f5f5f5;
  padding: 20rpx;
}

.section {
  background: #fff;
  border-radius: 20rpx;
  padding: 20rpx 30rpx;
  margin-bottom: 20rpx;
}

.section-title {
  font-size: 26rpx;
  color: #999;
  display: block;
  margin-bottom: 10rpx;
}

.setting-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 28rpx 0;
  border-bottom: 1rpx solid #f5f5f5;
}

.setting-item:last-child {
  border-bottom: none;
}

.setting-item.danger .setting-label {
  color: #ff4d4f;
}

.setting-label {
  font-size: 30rpx;
  color: #333;
}

.setting-value {
  display: flex;
  align-items: center;
  gap: 10rpx;
  color: #999;
  font-size: 28rpx;
}

.arrow {
  font-size: 32rpx;
  color: #ccc;
}

.avatar-preview {
  width: 60rpx;
  height: 60rpx;
  background: #f5f5f5;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32rpx;
  overflow: hidden;
}

.avatar-preview image {
  width: 100%;
  height: 100%;
}

.cache-size {
  color: #667eea;
}

/* 弹窗样式 */
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
  z-index: 1000;
}

.modal-content {
  width: 80%;
  max-width: 600rpx;
  background: #fff;
  border-radius: 20rpx;
  padding: 40rpx;
}

.modal-title {
  font-size: 34rpx;
  font-weight: 600;
  color: #333;
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

.modal-buttons {
  display: flex;
  gap: 20rpx;
}

.btn-cancel, .btn-confirm {
  flex: 1;
  padding: 24rpx;
  border-radius: 12rpx;
  font-size: 28rpx;
}

.btn-cancel {
  background: #f5f5f5;
  color: #666;
}

.btn-confirm {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #fff;
}

.btn-confirm:disabled {
  opacity: 0.5;
}
</style>

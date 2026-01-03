<template>
  <view class="container">
    <!-- 账本列表 -->
    <view class="section">
      <view class="section-header">
        <text class="section-title">我的账本</text>
        <text class="add-btn" @click="showCreateDialog">+ 新建</text>
      </view>

      <view v-if="ledgerStore.loading" class="loading">
        <text>加载中...</text>
      </view>

      <view v-else-if="ledgerStore.ledgers.length === 0" class="empty">
        <text class="empty-icon">📚</text>
        <text class="empty-text">暂无账本</text>
        <button class="btn-create" @click="showCreateDialog">创建第一个账本</button>
      </view>

      <view v-else class="ledger-list">
        <view
          v-for="ledger in ledgerStore.ledgers"
          :key="ledger.id"
          class="ledger-item"
          :class="{ active: ledger.id === ledgerStore.currentLedgerId }"
          @click="selectLedger(ledger)"
        >
          <view class="ledger-icon">
            <text>{{ ledger.type === 'group' ? '👥' : '📖' }}</text>
          </view>
          <view class="ledger-info">
            <text class="ledger-name">{{ ledger.name }}</text>
            <text class="ledger-type">{{ ledger.type === 'group' ? '群组账本' : '个人账本' }}</text>
          </view>
          <view class="ledger-actions">
            <text v-if="ledger.id === ledgerStore.currentLedgerId" class="current-badge">当前</text>
            <text class="delete-btn" @click.stop="confirmDelete(ledger)">删除</text>
          </view>
        </view>
      </view>
    </view>

    <!-- 创建账本弹窗 -->
    <view class="modal" v-if="showModal" @click="closeModal">
      <view class="modal-content" @click.stop>
        <text class="modal-title">新建账本</text>
        <input
          class="modal-input"
          v-model="newLedgerName"
          placeholder="请输入账本名称"
          maxlength="20"
        />
        <view class="modal-type">
          <text class="type-label">账本类型</text>
          <view class="type-options">
            <view
              class="type-option"
              :class="{ selected: newLedgerType === 'personal' }"
              @click="newLedgerType = 'personal'"
            >
              <text>📖 个人账本</text>
            </view>
            <view
              class="type-option"
              :class="{ selected: newLedgerType === 'group' }"
              @click="newLedgerType = 'group'"
            >
              <text>👥 群组账本</text>
            </view>
          </view>
        </view>
        <view class="modal-buttons">
          <button class="btn-cancel" @click="closeModal">取消</button>
          <button class="btn-confirm" @click="createLedger" :disabled="!newLedgerName.trim()">创建</button>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useLedgerStore } from '@/store/ledger'
import type { Ledger } from '@money-notes/api'

const ledgerStore = useLedgerStore()

const showModal = ref(false)
const newLedgerName = ref('')
const newLedgerType = ref<'personal' | 'group'>('personal')

onMounted(() => {
  ledgerStore.fetchLedgers()
})

function showCreateDialog() {
  newLedgerName.value = ''
  newLedgerType.value = 'personal'
  showModal.value = true
}

function closeModal() {
  showModal.value = false
}

async function createLedger() {
  if (!newLedgerName.value.trim()) {
    uni.showToast({ title: '请输入账本名称', icon: 'none' })
    return
  }

  try {
    await ledgerStore.createNewLedger(newLedgerName.value.trim(), newLedgerType.value)
    uni.showToast({ title: '创建成功', icon: 'success' })
    closeModal()
  } catch (error: any) {
    uni.showToast({ title: error.message || '创建失败', icon: 'none' })
  }
}

function selectLedger(ledger: Ledger) {
  ledgerStore.setCurrentLedger(ledger.id)
  ledgerStore.saveCurrentLedgerId()
  uni.showToast({ title: `已切换到"${ledger.name}"`, icon: 'success' })
}

function confirmDelete(ledger: Ledger) {
  if (ledger.id === ledgerStore.currentLedgerId) {
    uni.showToast({ title: '不能删除当前账本', icon: 'none' })
    return
  }

  uni.showModal({
    title: '确认删除',
    content: `确定要删除账本"${ledger.name}"吗？删除后数据将无法恢复。`,
    success: async (res) => {
      if (res.confirm) {
        try {
          await ledgerStore.deleteLedger(ledger.id)
          uni.showToast({ title: '删除成功', icon: 'success' })
        } catch (error: any) {
          uni.showToast({ title: error.message || '删除失败', icon: 'none' })
        }
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

.section {
  background: #fff;
  border-radius: 20rpx;
  padding: 30rpx;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30rpx;
}

.section-title {
  font-size: 32rpx;
  font-weight: 600;
  color: #333;
}

.add-btn {
  font-size: 28rpx;
  color: #667eea;
}

.loading, .empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 60rpx 0;
}

.empty-icon {
  font-size: 80rpx;
  margin-bottom: 20rpx;
}

.empty-text {
  font-size: 28rpx;
  color: #999;
  margin-bottom: 30rpx;
}

.btn-create {
  padding: 20rpx 40rpx;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: #fff;
  font-size: 28rpx;
  border-radius: 40rpx;
}

.ledger-list {
  display: flex;
  flex-direction: column;
  gap: 20rpx;
}

.ledger-item {
  display: flex;
  align-items: center;
  padding: 24rpx;
  background: #f8f9fa;
  border-radius: 16rpx;
  border: 2rpx solid transparent;
  transition: all 0.2s;
}

.ledger-item.active {
  border-color: #667eea;
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
}

.ledger-icon {
  width: 80rpx;
  height: 80rpx;
  background: #fff;
  border-radius: 16rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 40rpx;
  margin-right: 20rpx;
}

.ledger-info {
  flex: 1;
}

.ledger-name {
  font-size: 30rpx;
  font-weight: 500;
  color: #333;
  display: block;
}

.ledger-type {
  font-size: 24rpx;
  color: #999;
  display: block;
  margin-top: 6rpx;
}

.ledger-actions {
  display: flex;
  align-items: center;
  gap: 16rpx;
}

.current-badge {
  padding: 8rpx 16rpx;
  background: #667eea;
  color: #fff;
  font-size: 22rpx;
  border-radius: 20rpx;
}

.delete-btn {
  padding: 8rpx 16rpx;
  color: #ff4d4f;
  font-size: 24rpx;
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

.modal-type {
  margin-bottom: 30rpx;
}

.type-label {
  font-size: 28rpx;
  color: #666;
  display: block;
  margin-bottom: 16rpx;
}

.type-options {
  display: flex;
  gap: 20rpx;
}

.type-option {
  flex: 1;
  padding: 20rpx;
  background: #f5f5f5;
  border-radius: 12rpx;
  text-align: center;
  font-size: 26rpx;
  color: #666;
  border: 2rpx solid transparent;
}

.type-option.selected {
  background: linear-gradient(135deg, rgba(102, 126, 234, 0.1) 0%, rgba(118, 75, 162, 0.1) 100%);
  border-color: #667eea;
  color: #667eea;
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

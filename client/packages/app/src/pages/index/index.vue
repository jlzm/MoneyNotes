<template>
  <view class="container">
    <!-- 顶部统计卡片 -->
    <view class="stats-card">
      <view class="stats-header">
        <text class="stats-date">{{ currentMonth }}</text>
      </view>
      <view class="stats-body">
        <view class="stats-item">
          <text class="stats-label">支出</text>
          <text class="stats-value expense">¥{{ monthExpense.toFixed(2) }}</text>
        </view>
        <view class="stats-item">
          <text class="stats-label">收入</text>
          <text class="stats-value income">¥{{ monthIncome.toFixed(2) }}</text>
        </view>
        <view class="stats-item">
          <text class="stats-label">结余</text>
          <text class="stats-value">¥{{ (monthIncome - monthExpense).toFixed(2) }}</text>
        </view>
      </view>
    </view>

    <!-- 账单列表 -->
    <view class="bill-list">
      <view class="bill-section" v-for="(group, date) in groupedBills" :key="date">
        <view class="bill-date">
          <text class="date-text">{{ formatDate(date) }}</text>
          <text class="date-stats">支出 ¥{{ getDayExpense(group).toFixed(2) }}</text>
        </view>
        <view class="bill-item" v-for="bill in group" :key="bill.id || bill.localId">
          <view class="bill-icon">
            <text>{{ getCategoryIcon(bill.category.icon) }}</text>
          </view>
          <view class="bill-info">
            <text class="bill-category">{{ bill.category.name }}</text>
            <text class="bill-note" v-if="bill.note">{{ bill.note }}</text>
          </view>
          <text class="bill-amount" :class="bill.type">
            {{ bill.type === 'expense' ? '-' : '+' }}¥{{ bill.amount.toFixed(2) }}
          </text>
        </view>
      </view>

      <view class="empty-state" v-if="Object.keys(groupedBills).length === 0">
        <text class="empty-text">暂无账单记录</text>
        <text class="empty-hint">点击下方"记账"开始记录</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { computed, onMounted } from 'vue'
import { useBillStore } from '@/store/bill'

const billStore = useBillStore()

const currentMonth = computed(() => {
  const now = new Date()
  return `${now.getFullYear()}年${now.getMonth() + 1}月`
})

const monthIncome = computed(() => {
  return billStore.allBills
    .filter(b => b.type === 'income')
    .reduce((sum, b) => sum + b.amount, 0)
})

const monthExpense = computed(() => {
  return billStore.allBills
    .filter(b => b.type === 'expense')
    .reduce((sum, b) => sum + b.amount, 0)
})

const groupedBills = computed(() => {
  const groups: Record<string, any[]> = {}
  billStore.allBills.forEach(bill => {
    const date = bill.billDate
    if (!groups[date]) {
      groups[date] = []
    }
    groups[date].push(bill)
  })
  return groups
})

function formatDate(dateStr: string) {
  const date = new Date(dateStr)
  const today = new Date()
  const yesterday = new Date(today)
  yesterday.setDate(yesterday.getDate() - 1)

  if (dateStr === today.toISOString().split('T')[0]) {
    return '今天'
  } else if (dateStr === yesterday.toISOString().split('T')[0]) {
    return '昨天'
  }
  return `${date.getMonth() + 1}月${date.getDate()}日`
}

function getDayExpense(bills: any[]) {
  return bills
    .filter(b => b.type === 'expense')
    .reduce((sum, b) => sum + b.amount, 0)
}

function getCategoryIcon(icon?: string) {
  const iconMap: Record<string, string> = {
    food: '🍔',
    transport: '🚗',
    shopping: '🛒',
    entertainment: '🎮',
    housing: '🏠',
    medical: '💊',
    education: '📚',
    communication: '📱',
    salary: '💰',
    bonus: '🎁',
    investment: '📈',
    other: '📋'
  }
  return iconMap[icon || 'other'] || '📋'
}

onMounted(() => {
  billStore.loadLocalBills()
})
</script>

<style scoped>
.container {
  padding: 20rpx;
}

.stats-card {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 20rpx;
  padding: 30rpx;
  color: #fff;
  margin-bottom: 20rpx;
}

.stats-header {
  margin-bottom: 20rpx;
}

.stats-date {
  font-size: 32rpx;
  font-weight: 500;
}

.stats-body {
  display: flex;
  justify-content: space-between;
}

.stats-item {
  text-align: center;
}

.stats-label {
  font-size: 24rpx;
  opacity: 0.8;
  display: block;
  margin-bottom: 8rpx;
}

.stats-value {
  font-size: 36rpx;
  font-weight: 600;
}

.bill-list {
  background: #fff;
  border-radius: 20rpx;
  overflow: hidden;
}

.bill-section {
  border-bottom: 1rpx solid #f0f0f0;
}

.bill-date {
  display: flex;
  justify-content: space-between;
  padding: 20rpx 30rpx;
  background: #fafafa;
}

.date-text {
  font-size: 28rpx;
  color: #333;
}

.date-stats {
  font-size: 24rpx;
  color: #999;
}

.bill-item {
  display: flex;
  align-items: center;
  padding: 24rpx 30rpx;
  border-bottom: 1rpx solid #f5f5f5;
}

.bill-icon {
  width: 80rpx;
  height: 80rpx;
  background: #f5f5f5;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 36rpx;
  margin-right: 20rpx;
}

.bill-info {
  flex: 1;
}

.bill-category {
  font-size: 30rpx;
  color: #333;
  display: block;
}

.bill-note {
  font-size: 24rpx;
  color: #999;
  display: block;
  margin-top: 4rpx;
}

.bill-amount {
  font-size: 32rpx;
  font-weight: 500;
}

.bill-amount.expense {
  color: #333;
}

.bill-amount.income {
  color: #52c41a;
}

.empty-state {
  padding: 100rpx 0;
  text-align: center;
}

.empty-text {
  font-size: 32rpx;
  color: #999;
  display: block;
}

.empty-hint {
  font-size: 26rpx;
  color: #ccc;
  display: block;
  margin-top: 16rpx;
}
</style>

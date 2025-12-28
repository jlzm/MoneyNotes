<template>
  <view class="container">
    <!-- 顶部标题和周期切换 -->
    <view class="header">
      <text class="title">统计</text>
    </view>

    <!-- 周期选择器 -->
    <view class="period-tabs">
      <view
        class="period-tab"
        :class="{ active: currentPeriod === 'day' }"
        @click="changePeriod('day')"
      >
        <text>日</text>
      </view>
      <view
        class="period-tab"
        :class="{ active: currentPeriod === 'week' }"
        @click="changePeriod('week')"
      >
        <text>周</text>
      </view>
      <view
        class="period-tab"
        :class="{ active: currentPeriod === 'month' }"
        @click="changePeriod('month')"
      >
        <text>月</text>
      </view>
      <view
        class="period-tab"
        :class="{ active: currentPeriod === 'year' }"
        @click="changePeriod('year')"
      >
        <text>年</text>
      </view>
    </view>

    <!-- 日期导航 -->
    <view class="date-nav">
      <view class="nav-btn" @click="prevPeriod">
        <text>&lt;</text>
      </view>
      <text class="date-label">{{ dateLabel }}</text>
      <view class="nav-btn" @click="nextPeriod">
        <text>&gt;</text>
      </view>
    </view>

    <!-- 概览卡片 -->
    <view class="summary-card">
      <view class="summary-item">
        <text class="summary-label">收入</text>
        <text class="summary-value income">¥{{ summary.totalIncome.toFixed(2) }}</text>
      </view>
      <view class="summary-divider"></view>
      <view class="summary-item">
        <text class="summary-label">支出</text>
        <text class="summary-value expense">¥{{ summary.totalExpense.toFixed(2) }}</text>
      </view>
      <view class="summary-divider"></view>
      <view class="summary-item">
        <text class="summary-label">结余</text>
        <text class="summary-value" :class="summary.balance >= 0 ? 'income' : 'expense'">
          ¥{{ summary.balance.toFixed(2) }}
        </text>
      </view>
    </view>

    <!-- 类型切换 -->
    <view class="type-tabs">
      <view
        class="type-tab"
        :class="{ active: currentType === 'expense' }"
        @click="currentType = 'expense'"
      >
        <text>支出</text>
      </view>
      <view
        class="type-tab"
        :class="{ active: currentType === 'income' }"
        @click="currentType = 'income'"
      >
        <text>收入</text>
      </view>
    </view>

    <!-- 分类统计 -->
    <view class="category-section">
      <view class="section-header">
        <text class="section-title">分类统计</text>
        <text class="section-total">
          共 ¥{{ currentTypeTotal.toFixed(2) }}
        </text>
      </view>

      <!-- 分类列表 -->
      <view class="category-list" v-if="categoryStats.length > 0">
        <view class="category-item" v-for="cat in categoryStats" :key="cat.categoryId">
          <view class="category-left">
            <view class="category-icon">
              <text>{{ getCategoryIcon(cat.categoryIcon) }}</text>
            </view>
            <view class="category-info">
              <text class="category-name">{{ cat.categoryName }}</text>
              <text class="category-count">{{ cat.count }}笔</text>
            </view>
          </view>
          <view class="category-right">
            <text class="category-amount">¥{{ cat.amount.toFixed(2) }}</text>
            <text class="category-percent">{{ cat.percentage.toFixed(1) }}%</text>
          </view>
          <!-- 进度条 -->
          <view class="progress-bar">
            <view
              class="progress-fill"
              :style="{ width: cat.percentage + '%' }"
              :class="currentType"
            ></view>
          </view>
        </view>
      </view>

      <view class="empty-state" v-else>
        <text class="empty-text">暂无{{ currentType === 'expense' ? '支出' : '收入' }}记录</text>
      </view>
    </view>

    <!-- 趋势图表区域 -->
    <view class="trend-section">
      <view class="section-header">
        <text class="section-title">收支趋势</text>
      </view>

      <!-- 简易趋势图 -->
      <view class="trend-chart" v-if="trendData.length > 0">
        <view class="chart-bars">
          <view class="chart-bar-group" v-for="(item, index) in trendData" :key="index">
            <view class="bar-container">
              <view
                class="bar income"
                :style="{ height: getBarHeight(item.income) + 'rpx' }"
              ></view>
              <view
                class="bar expense"
                :style="{ height: getBarHeight(item.expense) + 'rpx' }"
              ></view>
            </view>
            <text class="bar-label">{{ formatTrendLabel(item.period) }}</text>
          </view>
        </view>
        <view class="chart-legend">
          <view class="legend-item">
            <view class="legend-dot income"></view>
            <text class="legend-text">收入</text>
          </view>
          <view class="legend-item">
            <view class="legend-dot expense"></view>
            <text class="legend-text">支出</text>
          </view>
        </view>
      </view>

      <view class="empty-state" v-else>
        <text class="empty-text">暂无趋势数据</text>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue'
import { useBillStore } from '@/store/bill'
import { useUserStore } from '@/store/user'

type PeriodType = 'day' | 'week' | 'month' | 'year'
type BillType = 'income' | 'expense'

interface CategoryStat {
  categoryId: string
  categoryName: string
  categoryIcon?: string
  type: BillType
  amount: number
  count: number
  percentage: number
}

interface TrendStat {
  period: string
  income: number
  expense: number
  balance: number
}

const billStore = useBillStore()
const userStore = useUserStore()

const currentPeriod = ref<PeriodType>('month')
const currentType = ref<BillType>('expense')
const currentDate = ref(new Date())

// 计算日期范围
const dateRange = computed(() => {
  const date = currentDate.value
  let start: Date
  let end: Date

  switch (currentPeriod.value) {
    case 'day':
      start = new Date(date.getFullYear(), date.getMonth(), date.getDate())
      end = new Date(date.getFullYear(), date.getMonth(), date.getDate())
      break
    case 'week':
      const dayOfWeek = date.getDay() || 7
      start = new Date(date)
      start.setDate(date.getDate() - dayOfWeek + 1)
      end = new Date(start)
      end.setDate(start.getDate() + 6)
      break
    case 'month':
      start = new Date(date.getFullYear(), date.getMonth(), 1)
      end = new Date(date.getFullYear(), date.getMonth() + 1, 0)
      break
    case 'year':
      start = new Date(date.getFullYear(), 0, 1)
      end = new Date(date.getFullYear(), 11, 31)
      break
    default:
      start = new Date(date.getFullYear(), date.getMonth(), 1)
      end = new Date(date.getFullYear(), date.getMonth() + 1, 0)
  }

  return {
    start: formatDateStr(start),
    end: formatDateStr(end)
  }
})

// 日期标签
const dateLabel = computed(() => {
  const date = currentDate.value
  switch (currentPeriod.value) {
    case 'day':
      return `${date.getFullYear()}年${date.getMonth() + 1}月${date.getDate()}日`
    case 'week':
      const weekStart = new Date(date)
      const dayOfWeek = date.getDay() || 7
      weekStart.setDate(date.getDate() - dayOfWeek + 1)
      const weekEnd = new Date(weekStart)
      weekEnd.setDate(weekStart.getDate() + 6)
      return `${weekStart.getMonth() + 1}月${weekStart.getDate()}日 - ${weekEnd.getMonth() + 1}月${weekEnd.getDate()}日`
    case 'month':
      return `${date.getFullYear()}年${date.getMonth() + 1}月`
    case 'year':
      return `${date.getFullYear()}年`
    default:
      return ''
  }
})

// 筛选当前周期的账单
const filteredBills = computed(() => {
  const { start, end } = dateRange.value
  return billStore.allBills.filter(bill => {
    return bill.billDate >= start && bill.billDate <= end
  })
})

// 概览统计
const summary = computed(() => {
  const bills = filteredBills.value
  const totalIncome = bills
    .filter(b => b.type === 'income')
    .reduce((sum, b) => sum + b.amount, 0)
  const totalExpense = bills
    .filter(b => b.type === 'expense')
    .reduce((sum, b) => sum + b.amount, 0)

  return {
    totalIncome,
    totalExpense,
    balance: totalIncome - totalExpense
  }
})

// 当前类型总额
const currentTypeTotal = computed(() => {
  return currentType.value === 'income'
    ? summary.value.totalIncome
    : summary.value.totalExpense
})

// 分类统计
const categoryStats = computed<CategoryStat[]>(() => {
  const bills = filteredBills.value.filter(b => b.type === currentType.value)
  const categoryMap = new Map<string, { name: string; icon?: string; amount: number; count: number }>()

  bills.forEach(bill => {
    const catId = bill.category.id
    const existing = categoryMap.get(catId)
    if (existing) {
      existing.amount += bill.amount
      existing.count += 1
    } else {
      categoryMap.set(catId, {
        name: bill.category.name,
        icon: bill.category.icon,
        amount: bill.amount,
        count: 1
      })
    }
  })

  const total = Array.from(categoryMap.values()).reduce((sum, c) => sum + c.amount, 0)

  const result: CategoryStat[] = Array.from(categoryMap.entries()).map(([id, data]) => ({
    categoryId: id,
    categoryName: data.name,
    categoryIcon: data.icon,
    type: currentType.value,
    amount: data.amount,
    count: data.count,
    percentage: total > 0 ? (data.amount / total) * 100 : 0
  }))

  return result.sort((a, b) => b.amount - a.amount)
})

// 趋势数据
const trendData = computed<TrendStat[]>(() => {
  const bills = filteredBills.value
  const trendMap = new Map<string, { income: number; expense: number }>()

  bills.forEach(bill => {
    let period: string
    const date = new Date(bill.billDate)

    switch (currentPeriod.value) {
      case 'day':
      case 'week':
        period = bill.billDate
        break
      case 'month':
        period = `${date.getDate()}`
        break
      case 'year':
        period = `${date.getMonth() + 1}月`
        break
      default:
        period = bill.billDate
    }

    const existing = trendMap.get(period)
    if (existing) {
      if (bill.type === 'income') {
        existing.income += bill.amount
      } else {
        existing.expense += bill.amount
      }
    } else {
      trendMap.set(period, {
        income: bill.type === 'income' ? bill.amount : 0,
        expense: bill.type === 'expense' ? bill.amount : 0
      })
    }
  })

  return Array.from(trendMap.entries())
    .map(([period, data]) => ({
      period,
      income: data.income,
      expense: data.expense,
      balance: data.income - data.expense
    }))
    .sort((a, b) => a.period.localeCompare(b.period))
    .slice(-10) // 最多显示10个数据点
})

// 趋势图最大值
const maxTrendValue = computed(() => {
  if (trendData.value.length === 0) return 1
  return Math.max(
    ...trendData.value.map(d => Math.max(d.income, d.expense)),
    1
  )
})

function formatDateStr(date: Date): string {
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}-${month}-${day}`
}

function changePeriod(period: PeriodType) {
  currentPeriod.value = period
  currentDate.value = new Date()
}

function prevPeriod() {
  const date = new Date(currentDate.value)
  switch (currentPeriod.value) {
    case 'day':
      date.setDate(date.getDate() - 1)
      break
    case 'week':
      date.setDate(date.getDate() - 7)
      break
    case 'month':
      date.setMonth(date.getMonth() - 1)
      break
    case 'year':
      date.setFullYear(date.getFullYear() - 1)
      break
  }
  currentDate.value = date
}

function nextPeriod() {
  const date = new Date(currentDate.value)
  switch (currentPeriod.value) {
    case 'day':
      date.setDate(date.getDate() + 1)
      break
    case 'week':
      date.setDate(date.getDate() + 7)
      break
    case 'month':
      date.setMonth(date.getMonth() + 1)
      break
    case 'year':
      date.setFullYear(date.getFullYear() + 1)
      break
  }
  currentDate.value = date
}

function getCategoryIcon(icon?: string): string {
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

function getBarHeight(value: number): number {
  const maxHeight = 150
  return Math.round((value / maxTrendValue.value) * maxHeight)
}

function formatTrendLabel(period: string): string {
  if (currentPeriod.value === 'year') {
    return period
  }
  if (currentPeriod.value === 'month') {
    return `${period}日`
  }
  // 对于日和周，显示简短日期
  const date = new Date(period)
  return `${date.getMonth() + 1}/${date.getDate()}`
}

onMounted(() => {
  billStore.loadLocalBills()
})
</script>

<style scoped>
.container {
  padding: 20rpx;
  background: #f5f5f5;
  min-height: 100vh;
}

.header {
  padding: 20rpx 0;
}

.title {
  font-size: 40rpx;
  font-weight: 600;
  color: #333;
}

/* 周期选择器 */
.period-tabs {
  display: flex;
  background: #fff;
  border-radius: 16rpx;
  padding: 8rpx;
  margin-bottom: 20rpx;
}

.period-tab {
  flex: 1;
  text-align: center;
  padding: 16rpx 0;
  font-size: 28rpx;
  color: #666;
  border-radius: 12rpx;
}

.period-tab.active {
  background: #667eea;
  color: #fff;
}

/* 日期导航 */
.date-nav {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20rpx;
  background: #fff;
  border-radius: 16rpx;
  margin-bottom: 20rpx;
}

.nav-btn {
  width: 60rpx;
  height: 60rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #f0f0f0;
  border-radius: 50%;
  font-size: 28rpx;
  color: #666;
}

.date-label {
  flex: 1;
  text-align: center;
  font-size: 30rpx;
  color: #333;
  font-weight: 500;
}

/* 概览卡片 */
.summary-card {
  display: flex;
  align-items: center;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 20rpx;
  padding: 40rpx 20rpx;
  margin-bottom: 20rpx;
}

.summary-item {
  flex: 1;
  text-align: center;
}

.summary-label {
  font-size: 24rpx;
  color: rgba(255, 255, 255, 0.8);
  display: block;
  margin-bottom: 12rpx;
}

.summary-value {
  font-size: 36rpx;
  font-weight: 600;
  color: #fff;
}

.summary-divider {
  width: 1rpx;
  height: 60rpx;
  background: rgba(255, 255, 255, 0.3);
}

/* 类型切换 */
.type-tabs {
  display: flex;
  background: #fff;
  border-radius: 16rpx;
  padding: 8rpx;
  margin-bottom: 20rpx;
}

.type-tab {
  flex: 1;
  text-align: center;
  padding: 16rpx 0;
  font-size: 28rpx;
  color: #666;
  border-radius: 12rpx;
}

.type-tab.active {
  background: #f0f0f0;
  color: #333;
  font-weight: 500;
}

/* 分类统计区域 */
.category-section {
  background: #fff;
  border-radius: 20rpx;
  padding: 30rpx;
  margin-bottom: 20rpx;
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

.section-total {
  font-size: 28rpx;
  color: #666;
}

/* 分类列表 */
.category-list {
  display: flex;
  flex-direction: column;
  gap: 24rpx;
}

.category-item {
  position: relative;
  padding-bottom: 16rpx;
}

.category-left {
  display: flex;
  align-items: center;
}

.category-icon {
  width: 72rpx;
  height: 72rpx;
  background: #f5f5f5;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 32rpx;
  margin-right: 20rpx;
}

.category-info {
  display: flex;
  flex-direction: column;
}

.category-name {
  font-size: 28rpx;
  color: #333;
}

.category-count {
  font-size: 22rpx;
  color: #999;
  margin-top: 4rpx;
}

.category-right {
  position: absolute;
  right: 0;
  top: 0;
  text-align: right;
}

.category-amount {
  font-size: 28rpx;
  color: #333;
  font-weight: 500;
  display: block;
}

.category-percent {
  font-size: 22rpx;
  color: #999;
  margin-top: 4rpx;
}

/* 进度条 */
.progress-bar {
  height: 8rpx;
  background: #f0f0f0;
  border-radius: 4rpx;
  margin-top: 16rpx;
  overflow: hidden;
}

.progress-fill {
  height: 100%;
  border-radius: 4rpx;
  transition: width 0.3s ease;
}

.progress-fill.expense {
  background: linear-gradient(90deg, #ff6b6b 0%, #ee5a52 100%);
}

.progress-fill.income {
  background: linear-gradient(90deg, #52c41a 0%, #73d13d 100%);
}

/* 趋势图区域 */
.trend-section {
  background: #fff;
  border-radius: 20rpx;
  padding: 30rpx;
}

.trend-chart {
  padding-top: 20rpx;
}

.chart-bars {
  display: flex;
  justify-content: space-around;
  align-items: flex-end;
  height: 200rpx;
  padding-bottom: 30rpx;
}

.chart-bar-group {
  display: flex;
  flex-direction: column;
  align-items: center;
  flex: 1;
}

.bar-container {
  display: flex;
  gap: 4rpx;
  align-items: flex-end;
  height: 150rpx;
}

.bar {
  width: 16rpx;
  border-radius: 4rpx 4rpx 0 0;
  min-height: 4rpx;
}

.bar.income {
  background: #52c41a;
}

.bar.expense {
  background: #ff6b6b;
}

.bar-label {
  font-size: 20rpx;
  color: #999;
  margin-top: 8rpx;
  white-space: nowrap;
}

.chart-legend {
  display: flex;
  justify-content: center;
  gap: 40rpx;
  margin-top: 20rpx;
}

.legend-item {
  display: flex;
  align-items: center;
  gap: 8rpx;
}

.legend-dot {
  width: 16rpx;
  height: 16rpx;
  border-radius: 50%;
}

.legend-dot.income {
  background: #52c41a;
}

.legend-dot.expense {
  background: #ff6b6b;
}

.legend-text {
  font-size: 24rpx;
  color: #666;
}

/* 空状态 */
.empty-state {
  padding: 60rpx 0;
  text-align: center;
}

.empty-text {
  font-size: 28rpx;
  color: #999;
}

/* 金额颜色 */
.income {
  color: #52c41a !important;
}

.expense {
  color: #ff6b6b !important;
}
</style>

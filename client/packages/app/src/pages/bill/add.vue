<template>
  <view class="container">
    <!-- 类型切换 -->
    <view class="type-tabs">
      <view
        class="tab-item"
        :class="{ active: billType === 'expense' }"
        @click="billType = 'expense'"
      >
        支出
      </view>
      <view
        class="tab-item"
        :class="{ active: billType === 'income' }"
        @click="billType = 'income'"
      >
        收入
      </view>
    </view>

    <!-- 金额输入 -->
    <view class="amount-section">
      <text class="currency">¥</text>
      <text class="amount">{{ amount || '0.00' }}</text>
    </view>

    <!-- 分类选择 -->
    <view class="category-section">
      <view class="section-title">选择分类</view>
      <view class="category-grid">
        <view
          class="category-item"
          :class="{ active: selectedCategory === cat.id }"
          v-for="cat in currentCategories"
          :key="cat.id"
          @click="selectedCategory = cat.id"
        >
          <view class="category-icon">{{ cat.icon }}</view>
          <text class="category-name">{{ cat.name }}</text>
        </view>
      </view>
    </view>

    <!-- 备注 -->
    <view class="note-section">
      <input
        class="note-input"
        type="text"
        placeholder="添加备注..."
        v-model="note"
      />
    </view>

    <!-- 日期选择 -->
    <view class="date-section">
      <picker mode="date" :value="billDate" @change="onDateChange">
        <view class="date-picker">
          <text class="date-label">日期</text>
          <text class="date-value">{{ billDate }}</text>
        </view>
      </picker>
    </view>

    <!-- 数字键盘 -->
    <view class="keyboard">
      <view class="keyboard-row">
        <view class="key" @click="inputNumber('7')">7</view>
        <view class="key" @click="inputNumber('8')">8</view>
        <view class="key" @click="inputNumber('9')">9</view>
        <view class="key" @click="inputNumber('backspace')">⌫</view>
      </view>
      <view class="keyboard-row">
        <view class="key" @click="inputNumber('4')">4</view>
        <view class="key" @click="inputNumber('5')">5</view>
        <view class="key" @click="inputNumber('6')">6</view>
        <view class="key" @click="inputNumber('+')">+</view>
      </view>
      <view class="keyboard-row">
        <view class="key" @click="inputNumber('1')">1</view>
        <view class="key" @click="inputNumber('2')">2</view>
        <view class="key" @click="inputNumber('3')">3</view>
        <view class="key" @click="inputNumber('-')">-</view>
      </view>
      <view class="keyboard-row">
        <view class="key" @click="inputNumber('.')">.</view>
        <view class="key" @click="inputNumber('0')">0</view>
        <view class="key key-confirm" @click="saveBill">完成</view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { useBillStore } from '@/store/bill'
import { useUserStore } from '@/store/user'

const billStore = useBillStore()
const userStore = useUserStore()

const billType = ref<'income' | 'expense'>('expense')
const amount = ref('')
const selectedCategory = ref('')
const note = ref('')
const billDate = ref(new Date().toISOString().split('T')[0])

const expenseCategories = [
  { id: '1', name: '餐饮', icon: '🍔' },
  { id: '2', name: '交通', icon: '🚗' },
  { id: '3', name: '购物', icon: '🛒' },
  { id: '4', name: '娱乐', icon: '🎮' },
  { id: '5', name: '居住', icon: '🏠' },
  { id: '6', name: '医疗', icon: '💊' },
  { id: '7', name: '教育', icon: '📚' },
  { id: '8', name: '通讯', icon: '📱' },
  { id: '9', name: '其他', icon: '📋' },
]

const incomeCategories = [
  { id: '10', name: '工资', icon: '💰' },
  { id: '11', name: '奖金', icon: '🎁' },
  { id: '12', name: '投资', icon: '📈' },
  { id: '13', name: '兼职', icon: '💼' },
  { id: '14', name: '红包', icon: '🧧' },
  { id: '15', name: '其他', icon: '📋' },
]

const currentCategories = computed(() =>
  billType.value === 'expense' ? expenseCategories : incomeCategories
)

function inputNumber(key: string) {
  if (key === 'backspace') {
    amount.value = amount.value.slice(0, -1)
    return
  }

  if (key === '.' && amount.value.includes('.')) {
    return
  }

  // 限制小数位数
  const parts = amount.value.split('.')
  if (parts[1] && parts[1].length >= 2) {
    return
  }

  amount.value += key
}

function onDateChange(e: any) {
  billDate.value = e.detail.value
}

function saveBill() {
  const amountNum = parseFloat(amount.value)
  if (!amountNum || amountNum <= 0) {
    uni.showToast({ title: '请输入金额', icon: 'none' })
    return
  }

  if (!selectedCategory.value) {
    uni.showToast({ title: '请选择分类', icon: 'none' })
    return
  }

  const category = currentCategories.value.find(c => c.id === selectedCategory.value)

  // 如果是访客模式，保存到本地
  if (userStore.isGuest) {
    billStore.addLocalBill({
      type: billType.value,
      amount: amountNum,
      category: {
        id: selectedCategory.value,
        name: category!.name,
        icon: category!.icon
      },
      note: note.value || undefined,
      billDate: billDate.value,
      createdAt: new Date().toISOString()
    })

    uni.showToast({ title: '保存成功', icon: 'success' })

    // 重置表单
    resetForm()

    // 返回首页
    uni.switchTab({ url: '/pages/index/index' })
  } else {
    // TODO: 调用API保存到服务器
    uni.showToast({ title: '保存成功', icon: 'success' })
    resetForm()
    uni.switchTab({ url: '/pages/index/index' })
  }
}

function resetForm() {
  amount.value = ''
  selectedCategory.value = ''
  note.value = ''
  billDate.value = new Date().toISOString().split('T')[0]
}
</script>

<style scoped>
.container {
  min-height: 100vh;
  background: #fff;
  display: flex;
  flex-direction: column;
}

.type-tabs {
  display: flex;
  padding: 20rpx;
  background: #f5f5f5;
}

.tab-item {
  flex: 1;
  text-align: center;
  padding: 20rpx;
  font-size: 30rpx;
  color: #666;
  border-radius: 10rpx;
}

.tab-item.active {
  background: #fff;
  color: #333;
  font-weight: 500;
}

.amount-section {
  padding: 40rpx 30rpx;
  display: flex;
  align-items: baseline;
}

.currency {
  font-size: 40rpx;
  color: #333;
  margin-right: 10rpx;
}

.amount {
  font-size: 72rpx;
  font-weight: 600;
  color: #333;
}

.category-section {
  padding: 20rpx 30rpx;
}

.section-title {
  font-size: 28rpx;
  color: #999;
  margin-bottom: 20rpx;
}

.category-grid {
  display: flex;
  flex-wrap: wrap;
}

.category-item {
  width: 20%;
  text-align: center;
  padding: 20rpx 0;
}

.category-item.active .category-icon {
  background: #667eea;
}

.category-icon {
  width: 80rpx;
  height: 80rpx;
  background: #f5f5f5;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto 10rpx;
  font-size: 36rpx;
}

.category-name {
  font-size: 24rpx;
  color: #666;
}

.note-section {
  padding: 20rpx 30rpx;
}

.note-input {
  width: 100%;
  padding: 20rpx;
  background: #f5f5f5;
  border-radius: 10rpx;
  font-size: 28rpx;
}

.date-section {
  padding: 20rpx 30rpx;
}

.date-picker {
  display: flex;
  justify-content: space-between;
  padding: 20rpx;
  background: #f5f5f5;
  border-radius: 10rpx;
}

.date-label {
  font-size: 28rpx;
  color: #666;
}

.date-value {
  font-size: 28rpx;
  color: #333;
}

.keyboard {
  margin-top: auto;
  background: #f5f5f5;
  padding: 10rpx;
}

.keyboard-row {
  display: flex;
}

.key {
  flex: 1;
  height: 100rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #fff;
  margin: 5rpx;
  border-radius: 10rpx;
  font-size: 36rpx;
  color: #333;
}

.key:active {
  background: #e0e0e0;
}

.key-confirm {
  flex: 2;
  background: #667eea;
  color: #fff;
}
</style>

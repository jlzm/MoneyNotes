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
        <!-- 添加分类按钮 -->
        <view class="category-item" @click="showAddModal = true">
          <view class="category-icon add-icon">+</view>
          <text class="category-name">添加</text>
        </view>
      </view>
    </view>

    <!-- 添加分类弹窗 -->
    <view class="modal-overlay" v-if="showAddModal" @click="showAddModal = false">
      <view class="modal-content" @click.stop>
        <view class="modal-header">
          <text class="modal-title">添加{{ billType === 'expense' ? '支出' : '收入' }}分类</text>
          <text class="modal-close" @click="showAddModal = false">×</text>
        </view>
        <view class="modal-body">
          <input
            class="category-name-input"
            type="text"
            placeholder="分类名称"
            v-model="newCategoryName"
          />
          <view class="icon-picker-title">选择图标</view>
          <view class="icon-picker-grid">
            <view
              class="icon-picker-item"
              :class="{ active: newCategoryIcon === icon }"
              v-for="icon in availableIcons"
              :key="icon"
              @click="newCategoryIcon = icon"
            >
              {{ categoryStore.getIconEmoji(icon) }}
            </view>
          </view>
        </view>
        <view class="modal-footer">
          <button class="btn-cancel" @click="showAddModal = false">取消</button>
          <button class="btn-confirm" @click="addNewCategory">确定</button>
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
import { ref, computed, onMounted } from 'vue'
import { useBillStore } from '@/store/bill'
import { useUserStore } from '@/store/user'
import { useCategoryStore, availableIcons } from '@/store/category'

const billStore = useBillStore()
const userStore = useUserStore()
const categoryStore = useCategoryStore()

const billType = ref<'income' | 'expense'>('expense')
const amount = ref('')
const selectedCategory = ref('')
const note = ref('')
const billDate = ref(new Date().toISOString().split('T')[0])

// 添加分类弹窗相关
const showAddModal = ref(false)
const newCategoryName = ref('')
const newCategoryIcon = ref('other')

// 使用 store 中的分类数据
const currentCategories = computed(() => {
  const categories = billType.value === 'expense'
    ? categoryStore.expenseCategories
    : categoryStore.incomeCategories

  // 转换为模板需要的格式
  return categories.map(cat => ({
    id: cat.id,
    name: cat.name,
    icon: categoryStore.getIconEmoji(cat.icon)
  }))
})

// 初始化分类数据
onMounted(() => {
  categoryStore.init()
})

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

// 添加新分类
function addNewCategory() {
  if (!newCategoryName.value.trim()) {
    uni.showToast({ title: '请输入分类名称', icon: 'none' })
    return
  }

  const newCat = categoryStore.addCategory({
    name: newCategoryName.value.trim(),
    icon: newCategoryIcon.value,
    type: billType.value,
    sortOrder: 50
  })

  // 自动选中新添加的分类
  selectedCategory.value = newCat.id

  // 重置并关闭弹窗
  newCategoryName.value = ''
  newCategoryIcon.value = 'other'
  showAddModal.value = false

  uni.showToast({ title: '添加成功', icon: 'success' })
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

/* 添加分类按钮 */
.add-icon {
  background: #e8e8e8;
  color: #999;
  font-size: 40rpx;
  font-weight: 300;
}

/* 弹窗样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 999;
}

.modal-content {
  width: 85%;
  max-width: 600rpx;
  background: #fff;
  border-radius: 20rpx;
  overflow: hidden;
}

.modal-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 30rpx;
  border-bottom: 1rpx solid #eee;
}

.modal-title {
  font-size: 32rpx;
  font-weight: 500;
  color: #333;
}

.modal-close {
  font-size: 48rpx;
  color: #999;
  line-height: 1;
}

.modal-body {
  padding: 30rpx;
}

.category-name-input {
  width: 100%;
  padding: 24rpx;
  background: #f5f5f5;
  border-radius: 10rpx;
  font-size: 28rpx;
  margin-bottom: 30rpx;
}

.icon-picker-title {
  font-size: 28rpx;
  color: #666;
  margin-bottom: 20rpx;
}

.icon-picker-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 16rpx;
}

.icon-picker-item {
  width: 80rpx;
  height: 80rpx;
  background: #f5f5f5;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 36rpx;
}

.icon-picker-item.active {
  background: #667eea;
}

.modal-footer {
  display: flex;
  border-top: 1rpx solid #eee;
}

.btn-cancel,
.btn-confirm {
  flex: 1;
  padding: 28rpx;
  font-size: 30rpx;
  border: none;
  background: #fff;
  border-radius: 0;
}

.btn-cancel {
  color: #666;
  border-right: 1rpx solid #eee;
}

.btn-confirm {
  color: #667eea;
  font-weight: 500;
}
</style>

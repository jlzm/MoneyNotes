<template>
  <view class="container">
    <!-- 顶部标题 -->
    <view class="header">
      <text class="title">分类管理</text>
    </view>

    <!-- 类型切换 -->
    <view class="type-tabs">
      <view
        class="tab-item"
        :class="{ active: currentType === 'expense' }"
        @click="currentType = 'expense'"
      >
        <text>支出</text>
      </view>
      <view
        class="tab-item"
        :class="{ active: currentType === 'income' }"
        @click="currentType = 'income'"
      >
        <text>收入</text>
      </view>
    </view>

    <!-- 分类列表 -->
    <view class="category-list">
      <view class="section-title">
        <text>{{ currentType === 'expense' ? '支出分类' : '收入分类' }}</text>
      </view>

      <view
        class="category-item"
        v-for="cat in currentCategories"
        :key="cat.id"
      >
        <view class="category-left">
          <view class="category-icon">
            <text>{{ categoryStore.getIconEmoji(cat.icon) }}</text>
          </view>
          <text class="category-name">{{ cat.name }}</text>
          <text class="category-tag" v-if="cat.isCustom">自定义</text>
        </view>
        <view class="category-right" v-if="cat.isCustom">
          <text class="action-btn edit" @click="editCategory(cat)">编辑</text>
          <text class="action-btn delete" @click="confirmDelete(cat)">删除</text>
        </view>
      </view>

      <!-- 添加按钮 -->
      <view class="add-btn" @click="showAddModal = true">
        <text class="add-icon">+</text>
        <text class="add-text">添加自定义分类</text>
      </view>
    </view>

    <!-- 添加/编辑弹窗 -->
    <view class="modal-mask" v-if="showAddModal" @click="closeModal">
      <view class="modal-content" @click.stop>
        <view class="modal-header">
          <text class="modal-title">{{ editingCategory ? '编辑分类' : '添加分类' }}</text>
          <text class="modal-close" @click="closeModal">✕</text>
        </view>

        <view class="modal-body">
          <!-- 分类名称 -->
          <view class="form-item">
            <text class="form-label">名称</text>
            <input
              class="form-input"
              type="text"
              placeholder="请输入分类名称"
              v-model="formData.name"
              maxlength="10"
            />
          </view>

          <!-- 图标选择 -->
          <view class="form-item">
            <text class="form-label">图标</text>
            <view class="icon-grid">
              <view
                class="icon-item"
                :class="{ active: formData.icon === icon }"
                v-for="icon in availableIcons"
                :key="icon"
                @click="formData.icon = icon"
              >
                <text>{{ categoryStore.getIconEmoji(icon) }}</text>
              </view>
            </view>
          </view>
        </view>

        <view class="modal-footer">
          <view class="btn btn-cancel" @click="closeModal">
            <text>取消</text>
          </view>
          <view class="btn btn-confirm" @click="saveCategory">
            <text>保存</text>
          </view>
        </view>
      </view>
    </view>
  </view>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useCategoryStore, availableIcons, type Category } from '@/store/category'

const categoryStore = useCategoryStore()

const currentType = ref<'income' | 'expense'>('expense')
const showAddModal = ref(false)
const editingCategory = ref<Category | null>(null)

const formData = ref({
  name: '',
  icon: 'other',
})

const currentCategories = computed(() => {
  return currentType.value === 'expense'
    ? categoryStore.expenseCategories
    : categoryStore.incomeCategories
})

function editCategory(cat: Category) {
  editingCategory.value = cat
  formData.value = {
    name: cat.name,
    icon: cat.icon,
  }
  showAddModal.value = true
}

function confirmDelete(cat: Category) {
  uni.showModal({
    title: '确认删除',
    content: `确定要删除分类"${cat.name}"吗？`,
    success: (res) => {
      if (res.confirm) {
        categoryStore.deleteCategory(cat.id)
        uni.showToast({ title: '删除成功', icon: 'success' })
      }
    }
  })
}

function closeModal() {
  showAddModal.value = false
  editingCategory.value = null
  formData.value = { name: '', icon: 'other' }
}

function saveCategory() {
  if (!formData.value.name.trim()) {
    uni.showToast({ title: '请输入分类名称', icon: 'none' })
    return
  }

  if (editingCategory.value) {
    // 编辑模式
    categoryStore.updateCategory(editingCategory.value.id, {
      name: formData.value.name.trim(),
      icon: formData.value.icon,
    })
    uni.showToast({ title: '修改成功', icon: 'success' })
  } else {
    // 添加模式
    categoryStore.addCategory({
      name: formData.value.name.trim(),
      icon: formData.value.icon,
      type: currentType.value,
      sortOrder: 50,
    })
    uni.showToast({ title: '添加成功', icon: 'success' })
  }

  closeModal()
}

onMounted(() => {
  categoryStore.init()
})
</script>

<style scoped>
.container {
  min-height: 100vh;
  background: #f5f5f5;
}

.header {
  padding: 30rpx;
  background: #fff;
}

.title {
  font-size: 40rpx;
  font-weight: 600;
  color: #333;
}

/* 类型切换 */
.type-tabs {
  display: flex;
  background: #fff;
  padding: 20rpx 30rpx;
  margin-bottom: 20rpx;
}

.tab-item {
  flex: 1;
  text-align: center;
  padding: 16rpx 0;
  font-size: 28rpx;
  color: #666;
  background: #f5f5f5;
  border-radius: 8rpx;
  margin: 0 10rpx;
}

.tab-item.active {
  background: #667eea;
  color: #fff;
}

/* 分类列表 */
.category-list {
  background: #fff;
  padding: 20rpx 30rpx;
}

.section-title {
  font-size: 28rpx;
  color: #999;
  margin-bottom: 20rpx;
}

.category-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 24rpx 0;
  border-bottom: 1rpx solid #f0f0f0;
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

.category-name {
  font-size: 30rpx;
  color: #333;
}

.category-tag {
  font-size: 20rpx;
  color: #667eea;
  background: rgba(102, 126, 234, 0.1);
  padding: 4rpx 12rpx;
  border-radius: 4rpx;
  margin-left: 16rpx;
}

.category-right {
  display: flex;
  gap: 20rpx;
}

.action-btn {
  font-size: 26rpx;
  padding: 8rpx 16rpx;
  border-radius: 6rpx;
}

.action-btn.edit {
  color: #667eea;
  background: rgba(102, 126, 234, 0.1);
}

.action-btn.delete {
  color: #ff4d4f;
  background: rgba(255, 77, 79, 0.1);
}

/* 添加按钮 */
.add-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 30rpx;
  margin-top: 20rpx;
  border: 2rpx dashed #ddd;
  border-radius: 12rpx;
}

.add-icon {
  font-size: 36rpx;
  color: #667eea;
  margin-right: 10rpx;
}

.add-text {
  font-size: 28rpx;
  color: #667eea;
}

/* 弹窗 */
.modal-mask {
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
  width: 90%;
  max-width: 600rpx;
  background: #fff;
  border-radius: 20rpx;
  overflow: hidden;
}

.modal-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 30rpx;
  border-bottom: 1rpx solid #f0f0f0;
}

.modal-title {
  font-size: 32rpx;
  font-weight: 500;
  color: #333;
}

.modal-close {
  font-size: 36rpx;
  color: #999;
  padding: 10rpx;
}

.modal-body {
  padding: 30rpx;
}

.form-item {
  margin-bottom: 30rpx;
}

.form-label {
  font-size: 28rpx;
  color: #666;
  margin-bottom: 16rpx;
  display: block;
}

.form-input {
  width: 100%;
  padding: 20rpx;
  background: #f5f5f5;
  border-radius: 10rpx;
  font-size: 28rpx;
}

/* 图标选择 */
.icon-grid {
  display: flex;
  flex-wrap: wrap;
  gap: 16rpx;
}

.icon-item {
  width: 80rpx;
  height: 80rpx;
  background: #f5f5f5;
  border-radius: 12rpx;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 36rpx;
  border: 2rpx solid transparent;
}

.icon-item.active {
  border-color: #667eea;
  background: rgba(102, 126, 234, 0.1);
}

.modal-footer {
  display: flex;
  padding: 20rpx 30rpx 30rpx;
  gap: 20rpx;
}

.btn {
  flex: 1;
  padding: 24rpx;
  text-align: center;
  border-radius: 12rpx;
  font-size: 30rpx;
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

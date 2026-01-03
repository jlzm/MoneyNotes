import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface Category {
  id: string
  name: string
  icon: string
  type: 'income' | 'expense'
  isCustom: boolean  // 是否为用户自定义
  sortOrder: number
}

// 默认分类（系统预设）
const defaultExpenseCategories: Category[] = [
  { id: 'sys_1', name: '餐饮', icon: 'food', type: 'expense', isCustom: false, sortOrder: 1 },
  { id: 'sys_2', name: '交通', icon: 'transport', type: 'expense', isCustom: false, sortOrder: 2 },
  { id: 'sys_3', name: '购物', icon: 'shopping', type: 'expense', isCustom: false, sortOrder: 3 },
  { id: 'sys_4', name: '娱乐', icon: 'entertainment', type: 'expense', isCustom: false, sortOrder: 4 },
  { id: 'sys_5', name: '居住', icon: 'housing', type: 'expense', isCustom: false, sortOrder: 5 },
  { id: 'sys_6', name: '医疗', icon: 'medical', type: 'expense', isCustom: false, sortOrder: 6 },
  { id: 'sys_7', name: '教育', icon: 'education', type: 'expense', isCustom: false, sortOrder: 7 },
  { id: 'sys_8', name: '通讯', icon: 'communication', type: 'expense', isCustom: false, sortOrder: 8 },
  { id: 'sys_9', name: '其他', icon: 'other', type: 'expense', isCustom: false, sortOrder: 99 },
]

const defaultIncomeCategories: Category[] = [
  { id: 'sys_10', name: '工资', icon: 'salary', type: 'income', isCustom: false, sortOrder: 1 },
  { id: 'sys_11', name: '奖金', icon: 'bonus', type: 'income', isCustom: false, sortOrder: 2 },
  { id: 'sys_12', name: '投资', icon: 'investment', type: 'income', isCustom: false, sortOrder: 3 },
  { id: 'sys_13', name: '兼职', icon: 'parttime', type: 'income', isCustom: false, sortOrder: 4 },
  { id: 'sys_14', name: '红包', icon: 'redpacket', type: 'income', isCustom: false, sortOrder: 5 },
  { id: 'sys_15', name: '其他', icon: 'other', type: 'income', isCustom: false, sortOrder: 99 },
]

// 图标映射
export const iconMap: Record<string, string> = {
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
  parttime: '💼',
  redpacket: '🧧',
  other: '📋',
  // 用户自定义图标
  travel: '✈️',
  pet: '🐱',
  beauty: '💄',
  sports: '⚽',
  gift: '🎀',
  insurance: '🛡️',
  tax: '📝',
  child: '👶',
  elder: '👴',
  social: '🍻',
  digital: '💻',
  clothing: '👔',
  book: '📖',
  movie: '🎬',
  music: '🎵',
  game: '🎲',
  fitness: '💪',
  coffee: '☕',
  fruit: '🍎',
  snack: '🍪',
}

// 可选图标列表
export const availableIcons = Object.keys(iconMap)

export const useCategoryStore = defineStore('category', () => {
  // 自定义分类
  const customCategories = ref<Category[]>([])
  const isLoading = ref(false)

  // 所有支出分类
  const expenseCategories = computed(() => {
    const customs = customCategories.value.filter(c => c.type === 'expense')
    return [...defaultExpenseCategories, ...customs].sort((a, b) => a.sortOrder - b.sortOrder)
  })

  // 所有收入分类
  const incomeCategories = computed(() => {
    const customs = customCategories.value.filter(c => c.type === 'income')
    return [...defaultIncomeCategories, ...customs].sort((a, b) => a.sortOrder - b.sortOrder)
  })

  // 获取图标 emoji
  function getIconEmoji(iconKey: string): string {
    return iconMap[iconKey] || '📋'
  }

  // 根据 ID 查找分类
  function findById(id: string): Category | undefined {
    return [...defaultExpenseCategories, ...defaultIncomeCategories, ...customCategories.value]
      .find(c => c.id === id)
  }

  // 添加自定义分类
  function addCategory(category: Omit<Category, 'id' | 'isCustom'>) {
    const newCategory: Category = {
      ...category,
      id: `custom_${Date.now()}`,
      isCustom: true,
    }
    customCategories.value.push(newCategory)
    saveToStorage()
    return newCategory
  }

  // 更新分类
  function updateCategory(id: string, updates: Partial<Omit<Category, 'id' | 'isCustom'>>) {
    const index = customCategories.value.findIndex(c => c.id === id)
    if (index !== -1) {
      customCategories.value[index] = {
        ...customCategories.value[index],
        ...updates,
      }
      saveToStorage()
    }
  }

  // 删除分类
  function deleteCategory(id: string) {
    const index = customCategories.value.findIndex(c => c.id === id)
    if (index !== -1) {
      customCategories.value.splice(index, 1)
      saveToStorage()
    }
  }

  // 持久化到本地存储
  function saveToStorage() {
    uni.setStorageSync('customCategories', JSON.stringify(customCategories.value))
  }

  // 从本地存储加载
  function loadFromStorage() {
    try {
      const stored = uni.getStorageSync('customCategories')
      if (stored) {
        customCategories.value = JSON.parse(stored)
      }
    } catch (e) {
      console.error('Failed to load custom categories:', e)
    }
  }

  // 初始化
  function init() {
    loadFromStorage()
  }

  return {
    customCategories,
    isLoading,
    expenseCategories,
    incomeCategories,
    getIconEmoji,
    findById,
    addCategory,
    updateCategory,
    deleteCategory,
    init,
  }
})

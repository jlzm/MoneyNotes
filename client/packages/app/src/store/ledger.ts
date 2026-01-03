import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { getLedgers, createLedger, deleteLedger as deleteLedgerApi, USE_MOCK, type Ledger } from '@money-notes/api'

export const useLedgerStore = defineStore('ledger', () => {
  const ledgers = ref<Ledger[]>([])
  const currentLedgerId = ref<string | null>(null)
  const loading = ref(false)

  const currentLedger = computed(() => {
    return ledgers.value.find(l => l.id === currentLedgerId.value)
  })

  const personalLedgers = computed(() => {
    return ledgers.value.filter(l => l.type === 'personal')
  })

  const groupLedgers = computed(() => {
    return ledgers.value.filter(l => l.type === 'group')
  })

  async function fetchLedgers() {
    if (USE_MOCK) {
      // Mock 数据
      ledgers.value = [{
        id: 'default',
        name: '默认账本',
        type: 'personal',
        currency: 'CNY',
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString()
      }]
      if (!currentLedgerId.value) {
        currentLedgerId.value = 'default'
      }
      return
    }

    loading.value = true
    try {
      const response = await getLedgers()
      if (response.data) {
        ledgers.value = response.data.items
        // 自动选择第一个账本
        if (!currentLedgerId.value && ledgers.value.length > 0) {
          currentLedgerId.value = ledgers.value[0].id
        }
      }
    } catch (error) {
      console.error('Failed to fetch ledgers:', error)
    } finally {
      loading.value = false
    }
  }

  async function createNewLedger(name: string, type: 'personal' | 'group' = 'personal') {
    if (USE_MOCK) {
      const newLedger: Ledger = {
        id: Date.now().toString(),
        name,
        type,
        currency: 'CNY',
        createdAt: new Date().toISOString(),
        updatedAt: new Date().toISOString()
      }
      ledgers.value.push(newLedger)
      return newLedger
    }

    const response = await createLedger({ name, type })
    if (response.data) {
      ledgers.value.push(response.data)
      return response.data
    }
    throw new Error(response.message)
  }

  function setCurrentLedger(id: string) {
    currentLedgerId.value = id
  }

  async function deleteLedger(id: string) {
    if (USE_MOCK) {
      ledgers.value = ledgers.value.filter(l => l.id !== id)
      return
    }

    await deleteLedgerApi(id)
    ledgers.value = ledgers.value.filter(l => l.id !== id)
  }

  function init() {
    // 从本地存储恢复
    const savedId = uni.getStorageSync('currentLedgerId')
    if (savedId) {
      currentLedgerId.value = savedId
    }
    fetchLedgers()
  }

  // 监听变化保存到本地
  function saveCurrentLedgerId() {
    if (currentLedgerId.value) {
      uni.setStorageSync('currentLedgerId', currentLedgerId.value)
    }
  }

  return {
    ledgers,
    currentLedgerId,
    currentLedger,
    personalLedgers,
    groupLedgers,
    loading,
    fetchLedgers,
    createNewLedger,
    setCurrentLedger,
    deleteLedger,
    init,
    saveCurrentLedgerId
  }
})

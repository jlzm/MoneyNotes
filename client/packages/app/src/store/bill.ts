import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface Bill {
  id: string
  type: 'income' | 'expense'
  amount: number
  category: {
    id: string
    name: string
    icon?: string
  }
  note?: string
  billDate: string
  createdAt: string
}

export interface LocalBill extends Omit<Bill, 'id'> {
  localId: string
  synced: boolean
}

export const useBillStore = defineStore('bill', () => {
  const bills = ref<Bill[]>([])
  const localBills = ref<LocalBill[]>([])
  const isLoading = ref(false)

  const allBills = computed(() => {
    const remote = bills.value.map(b => ({ ...b, synced: true }))
    const local = localBills.value.filter(b => !b.synced)
    return [...remote, ...local].sort((a, b) =>
      new Date(b.billDate).getTime() - new Date(a.billDate).getTime()
    )
  })

  const todayBills = computed(() => {
    const today = new Date().toISOString().split('T')[0]
    return allBills.value.filter(b => b.billDate === today)
  })

  const todayIncome = computed(() =>
    todayBills.value
      .filter(b => b.type === 'income')
      .reduce((sum, b) => sum + b.amount, 0)
  )

  const todayExpense = computed(() =>
    todayBills.value
      .filter(b => b.type === 'expense')
      .reduce((sum, b) => sum + b.amount, 0)
  )

  function setBills(newBills: Bill[]) {
    bills.value = newBills
  }

  function addLocalBill(bill: Omit<LocalBill, 'localId' | 'synced'>) {
    const localBill: LocalBill = {
      ...bill,
      localId: `local_${Date.now()}`,
      synced: false
    }
    localBills.value.push(localBill)

    // 持久化本地账单
    saveLocalBills()
  }

  function saveLocalBills() {
    uni.setStorageSync('localBills', JSON.stringify(localBills.value))
  }

  function loadLocalBills() {
    try {
      const stored = uni.getStorageSync('localBills')
      if (stored) {
        localBills.value = JSON.parse(stored)
      }
    } catch (e) {
      console.error('Failed to load local bills:', e)
    }
  }

  function markAsSynced(localId: string, remoteId: string) {
    const index = localBills.value.findIndex(b => b.localId === localId)
    if (index !== -1) {
      localBills.value.splice(index, 1)
      saveLocalBills()
    }
  }

  return {
    bills,
    localBills,
    isLoading,
    allBills,
    todayBills,
    todayIncome,
    todayExpense,
    setBills,
    addLocalBill,
    loadLocalBills,
    markAsSynced
  }
})

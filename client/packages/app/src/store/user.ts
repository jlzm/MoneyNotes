import { defineStore } from 'pinia'
import { ref, computed } from 'vue'

export interface User {
  id: string
  email: string
  nickname?: string
  avatar?: string
}

export const useUserStore = defineStore('user', () => {
  const user = ref<User | null>(null)
  const accessToken = ref<string | null>(null)
  const refreshToken = ref<string | null>(null)
  const isGuest = ref(true)

  const isLoggedIn = computed(() => !!accessToken.value && !isGuest.value)
  const userInfo = computed(() => user.value)

  function init() {
    // 从本地存储恢复用户状态
    try {
      const storedUser = uni.getStorageSync('user')
      const storedAccessToken = uni.getStorageSync('accessToken')
      const storedRefreshToken = uni.getStorageSync('refreshToken')

      if (storedUser && storedAccessToken) {
        user.value = JSON.parse(storedUser)
        accessToken.value = storedAccessToken
        refreshToken.value = storedRefreshToken
        isGuest.value = false
      }
    } catch (e) {
      console.error('Failed to restore user state:', e)
    }
  }

  function setUser(userData: User, tokens: { accessToken: string; refreshToken: string }) {
    user.value = userData
    accessToken.value = tokens.accessToken
    refreshToken.value = tokens.refreshToken
    isGuest.value = false

    // 持久化存储
    uni.setStorageSync('user', JSON.stringify(userData))
    uni.setStorageSync('accessToken', tokens.accessToken)
    uni.setStorageSync('refreshToken', tokens.refreshToken)
  }

  function logout() {
    user.value = null
    accessToken.value = null
    refreshToken.value = null
    isGuest.value = true

    // 清除存储
    uni.removeStorageSync('user')
    uni.removeStorageSync('accessToken')
    uni.removeStorageSync('refreshToken')
  }

  function updateAccessToken(newToken: string) {
    accessToken.value = newToken
    uni.setStorageSync('accessToken', newToken)
  }

  return {
    user,
    accessToken,
    refreshToken,
    isGuest,
    isLoggedIn,
    userInfo,
    init,
    setUser,
    logout,
    updateAccessToken
  }
})

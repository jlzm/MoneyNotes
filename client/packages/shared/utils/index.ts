/**
 * 格式化金额
 */
export function formatMoney(amount: number, currency = 'CNY'): string {
  const formatted = Math.abs(amount).toFixed(2)
  const prefix = amount < 0 ? '-' : ''

  const currencySymbols: Record<string, string> = {
    CNY: '¥',
    USD: '$',
    EUR: '€',
    GBP: '£',
    JPY: '¥',
  }

  return `${prefix}${currencySymbols[currency] || currency}${formatted}`
}

/**
 * 格式化日期
 */
export function formatDate(
  date: string | Date,
  format: 'full' | 'date' | 'time' | 'relative' = 'date'
): string {
  const d = typeof date === 'string' ? new Date(date) : date
  const now = new Date()

  if (format === 'relative') {
    const diff = now.getTime() - d.getTime()
    const days = Math.floor(diff / (1000 * 60 * 60 * 24))

    if (days === 0) return '今天'
    if (days === 1) return '昨天'
    if (days < 7) return `${days}天前`
    if (days < 30) return `${Math.floor(days / 7)}周前`
    if (days < 365) return `${Math.floor(days / 30)}个月前`
    return `${Math.floor(days / 365)}年前`
  }

  const year = d.getFullYear()
  const month = String(d.getMonth() + 1).padStart(2, '0')
  const day = String(d.getDate()).padStart(2, '0')
  const hours = String(d.getHours()).padStart(2, '0')
  const minutes = String(d.getMinutes()).padStart(2, '0')

  switch (format) {
    case 'full':
      return `${year}-${month}-${day} ${hours}:${minutes}`
    case 'time':
      return `${hours}:${minutes}`
    case 'date':
    default:
      return `${year}-${month}-${day}`
  }
}

/**
 * 获取当前月份范围
 */
export function getCurrentMonthRange(): { start: string; end: string } {
  const now = new Date()
  const start = new Date(now.getFullYear(), now.getMonth(), 1)
  const end = new Date(now.getFullYear(), now.getMonth() + 1, 0)

  return {
    start: formatDate(start),
    end: formatDate(end),
  }
}

/**
 * 验证邮箱格式
 */
export function isValidEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  return emailRegex.test(email)
}

/**
 * 生成唯一ID
 */
export function generateId(): string {
  return `${Date.now()}_${Math.random().toString(36).substring(2, 9)}`
}

/**
 * 防抖函数
 */
export function debounce<T extends (...args: any[]) => any>(
  fn: T,
  delay: number
): (...args: Parameters<T>) => void {
  let timer: ReturnType<typeof setTimeout> | null = null

  return (...args: Parameters<T>) => {
    if (timer) clearTimeout(timer)
    timer = setTimeout(() => fn(...args), delay)
  }
}

/**
 * 节流函数
 */
export function throttle<T extends (...args: any[]) => any>(
  fn: T,
  delay: number
): (...args: Parameters<T>) => void {
  let lastTime = 0

  return (...args: Parameters<T>) => {
    const now = Date.now()
    if (now - lastTime >= delay) {
      lastTime = now
      fn(...args)
    }
  }
}

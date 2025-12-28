interface RequestConfig {
  baseURL?: string
  timeout?: number
  headers?: Record<string, string>
}

interface RequestOptions {
  method?: 'GET' | 'POST' | 'PUT' | 'DELETE'
  data?: any
  headers?: Record<string, string>
}

interface ApiResponse<T = any> {
  code: number
  message: string
  data?: T
}

const defaultConfig: RequestConfig = {
  baseURL: '/api/v1',
  timeout: 30000,
}

let accessToken: string | null = null

export function setAccessToken(token: string | null) {
  accessToken = token
}

export async function request<T = any>(
  url: string,
  options: RequestOptions = {}
): Promise<ApiResponse<T>> {
  const { method = 'GET', data, headers = {} } = options

  // 添加认证头
  if (accessToken) {
    headers['Authorization'] = `Bearer ${accessToken}`
  }

  return new Promise((resolve, reject) => {
    uni.request({
      url: `${defaultConfig.baseURL}${url}`,
      method,
      data,
      header: {
        'Content-Type': 'application/json',
        ...headers,
      },
      timeout: defaultConfig.timeout,
      success: (res) => {
        const response = res.data as ApiResponse<T>

        if (response.code === 0) {
          resolve(response)
        } else if (response.code === 10002 || response.code === 20003) {
          // Token 过期，尝试刷新
          // TODO: 实现 token 刷新逻辑
          reject(new Error('Unauthorized'))
        } else {
          reject(new Error(response.message || 'Request failed'))
        }
      },
      fail: (err) => {
        reject(new Error(err.errMsg || 'Network error'))
      },
    })
  })
}

export function get<T = any>(url: string, params?: Record<string, any>) {
  const queryString = params
    ? '?' + Object.entries(params)
        .filter(([, v]) => v !== undefined && v !== null)
        .map(([k, v]) => `${k}=${encodeURIComponent(v)}`)
        .join('&')
    : ''

  return request<T>(`${url}${queryString}`, { method: 'GET' })
}

export function post<T = any>(url: string, data?: any) {
  return request<T>(url, { method: 'POST', data })
}

export function put<T = any>(url: string, data?: any) {
  return request<T>(url, { method: 'PUT', data })
}

export function del<T = any>(url: string) {
  return request<T>(url, { method: 'DELETE' })
}

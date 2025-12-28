import { post, setAccessToken } from './request'

export interface LoginRequest {
  email: string
  password: string
}

export interface RegisterRequest {
  email: string
  password: string
  nickname?: string
}

export interface AuthResponse {
  user: {
    id: string
    email: string
    nickname?: string
    avatar?: string
    createdAt: string
  }
  accessToken: string
  refreshToken: string
  expiresIn: number
}

export async function login(data: LoginRequest) {
  const response = await post<AuthResponse>('/auth/login', data)
  if (response.data) {
    setAccessToken(response.data.accessToken)
  }
  return response
}

export async function register(data: RegisterRequest) {
  const response = await post<AuthResponse>('/auth/register', data)
  if (response.data) {
    setAccessToken(response.data.accessToken)
  }
  return response
}

export async function refreshToken(token: string) {
  const response = await post<{ accessToken: string; expiresIn: number }>(
    '/auth/refresh',
    { refreshToken: token }
  )
  if (response.data) {
    setAccessToken(response.data.accessToken)
  }
  return response
}

export function logout() {
  setAccessToken(null)
  return post('/auth/logout')
}

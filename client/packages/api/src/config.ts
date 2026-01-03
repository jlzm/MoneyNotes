// API 配置
// 设置 USE_MOCK = true 使用模拟数据，false 使用真实 API

export const USE_MOCK = false

// 检测运行环境
const isH5 = typeof window !== 'undefined'
const isMiniProgram = typeof uni !== 'undefined' && !isH5

// API 基础路径
// - H5 (开发/生产): 使用相对路径，通过 nginx 代理
// - 小程序: 直连服务器
export const API_BASE_URL = isMiniProgram
  ? 'http://120.76.238.8:3000/api/v1'
  : '/api/v1'

import { get, post, put, del } from './request'

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
  user: {
    id: string
    nickname?: string
  }
  createdAt: string
}

export interface CreateBillRequest {
  ledgerId: string
  categoryId: string
  type: 'income' | 'expense'
  amount: number
  note?: string
  billDate: string
}

export interface BillListResponse {
  items: Bill[]
  pagination: {
    page: number
    pageSize: number
    total: number
    totalPages: number
  }
}

export interface BillStatistics {
  totalIncome: number
  totalExpense: number
  balance: number
  byCategory: CategoryStatistics[]
}

export interface CategoryStatistics {
  categoryId: string
  categoryName: string
  categoryIcon?: string
  type: 'income' | 'expense'
  amount: number
  count: number
  percentage: number
}

export interface DailyStatistics {
  date: string
  income: number
  expense: number
}

export interface TrendStatistics {
  period: string
  income: number
  expense: number
  balance: number
}

export interface FullStatisticsResponse {
  summary: BillStatistics
  daily: DailyStatistics[]
  trend: TrendStatistics[]
}

export function getBills(params: {
  ledgerId: string
  startDate?: string
  endDate?: string
  type?: 'income' | 'expense'
  categoryId?: string
  page?: number
  pageSize?: number
}) {
  return get<BillListResponse>('/bills', params)
}

export function getBill(id: string) {
  return get<Bill>(`/bills/${id}`)
}

export function createBill(data: CreateBillRequest) {
  return post<Bill>('/bills', data)
}

export function updateBill(id: string, data: Partial<CreateBillRequest>) {
  return put<Bill>(`/bills/${id}`, data)
}

export function deleteBill(id: string) {
  return del(`/bills/${id}`)
}

export function getBillStatistics(params: {
  ledgerId: string
  startDate?: string
  endDate?: string
  period?: 'day' | 'week' | 'month' | 'year'
  billType?: 'income' | 'expense'
}) {
  return get<FullStatisticsResponse>('/bills/statistics', params)
}

export function getCategoryStatistics(params: {
  ledgerId: string
  startDate?: string
  endDate?: string
  billType?: 'income' | 'expense'
}) {
  return get<CategoryStatistics[]>('/bills/statistics/category', params)
}

export function getTrendStatistics(params: {
  ledgerId: string
  startDate: string
  endDate: string
  groupBy?: 'day' | 'week' | 'month' | 'year'
}) {
  return get<TrendStatistics[]>('/bills/statistics/trend', params)
}

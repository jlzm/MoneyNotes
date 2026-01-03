import { get, post, put, del } from './request'

export interface Ledger {
  id: string
  name: string
  description?: string
  type: 'personal' | 'group'
  currency: string
  userId?: string
  groupId?: string
  createdAt: string
  updatedAt: string
}

export interface CreateLedgerRequest {
  name: string
  description?: string
  type?: 'personal' | 'group'
  currency?: string
  groupId?: string
}

export function getLedgers(params?: { type?: 'personal' | 'group'; groupId?: string }) {
  return get<{ items: Ledger[] }>('/ledgers', params)
}

export function getLedger(id: string) {
  return get<Ledger>(`/ledgers/${id}`)
}

export function createLedger(data: CreateLedgerRequest) {
  return post<Ledger>('/ledgers', data)
}

export function updateLedger(id: string, data: Partial<CreateLedgerRequest>) {
  return put<Ledger>(`/ledgers/${id}`, data)
}

export function deleteLedger(id: string) {
  return del(`/ledgers/${id}`)
}

export function getDefaultLedger() {
  return get<Ledger>('/ledgers/default')
}

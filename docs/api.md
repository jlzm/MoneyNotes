# API 设计文档

## 基础信息

- **Base URL**: `/api/v1`
- **认证方式**: JWT Bearer Token
- **内容类型**: `application/json`

## 通用响应格式

### 成功响应

```json
{
  "code": 0,
  "message": "success",
  "data": { ... }
}
```

### 错误响应

```json
{
  "code": 10001,
  "message": "错误描述",
  "errors": [...]  // 可选，字段级错误
}
```

### 错误码定义

| 错误码 | 说明 |
|--------|------|
| 0 | 成功 |
| 10001 | 参数校验错误 |
| 10002 | 未授权 |
| 10003 | 禁止访问 |
| 10004 | 资源不存在 |
| 10005 | 服务器内部错误 |
| 20001 | 用户已存在 |
| 20002 | 用户名或密码错误 |
| 20003 | Token 过期 |
| 30001 | 群组不存在 |
| 30002 | 无权限操作 |
| 30003 | 邀请码无效 |

---

## 认证模块 `/auth`

### POST /auth/register - 用户注册

**请求体:**
```json
{
  "email": "user@example.com",
  "password": "password123",
  "nickname": "用户昵称"
}
```

**响应:**
```json
{
  "code": 0,
  "data": {
    "user": {
      "id": "uuid",
      "email": "user@example.com",
      "nickname": "用户昵称"
    },
    "access_token": "eyJ...",
    "refresh_token": "eyJ...",
    "expires_in": 3600
  }
}
```

### POST /auth/login - 用户登录

**请求体:**
```json
{
  "email": "user@example.com",
  "password": "password123"
}
```

**响应:** 同注册

### POST /auth/refresh - 刷新 Token

**请求体:**
```json
{
  "refresh_token": "eyJ..."
}
```

**响应:**
```json
{
  "code": 0,
  "data": {
    "access_token": "eyJ...",
    "expires_in": 3600
  }
}
```

### POST /auth/logout - 退出登录

**Header:** `Authorization: Bearer <token>`

**响应:**
```json
{
  "code": 0,
  "message": "success"
}
```

---

## 用户模块 `/users`

### GET /users/me - 获取当前用户信息

**Header:** `Authorization: Bearer <token>`

**响应:**
```json
{
  "code": 0,
  "data": {
    "id": "uuid",
    "email": "user@example.com",
    "nickname": "用户昵称",
    "avatar": "https://...",
    "created_at": "2025-01-01T00:00:00Z"
  }
}
```

### PUT /users/me - 更新用户信息

**Header:** `Authorization: Bearer <token>`

**请求体:**
```json
{
  "nickname": "新昵称",
  "avatar": "https://..."
}
```

### PUT /users/me/password - 修改密码

**请求体:**
```json
{
  "old_password": "old123",
  "new_password": "new456"
}
```

---

## 账本模块 `/ledgers`

### GET /ledgers - 获取账本列表

**Header:** `Authorization: Bearer <token>`

**查询参数:**
- `type`: `personal` | `group` (可选)

**响应:**
```json
{
  "code": 0,
  "data": {
    "items": [
      {
        "id": "uuid",
        "name": "日常开支",
        "type": "personal",
        "currency": "CNY",
        "created_at": "2025-01-01T00:00:00Z"
      }
    ]
  }
}
```

### POST /ledgers - 创建账本

**请求体:**
```json
{
  "name": "日常开支",
  "description": "日常生活支出",
  "type": "personal",
  "currency": "CNY"
}
```

### GET /ledgers/:id - 获取账本详情

### PUT /ledgers/:id - 更新账本

### DELETE /ledgers/:id - 删除账本

---

## 账单模块 `/bills`

### GET /bills - 获取账单列表

**查询参数:**
- `ledger_id`: 账本ID (必填)
- `start_date`: 开始日期 (YYYY-MM-DD)
- `end_date`: 结束日期 (YYYY-MM-DD)
- `type`: `income` | `expense`
- `category_id`: 分类ID
- `page`: 页码 (默认 1)
- `page_size`: 每页数量 (默认 20)

**响应:**
```json
{
  "code": 0,
  "data": {
    "items": [
      {
        "id": "uuid",
        "type": "expense",
        "amount": 25.50,
        "category": {
          "id": "uuid",
          "name": "餐饮",
          "icon": "food"
        },
        "note": "午餐",
        "bill_date": "2025-01-15",
        "user": {
          "id": "uuid",
          "nickname": "张三"
        },
        "created_at": "2025-01-15T12:30:00Z"
      }
    ],
    "pagination": {
      "page": 1,
      "page_size": 20,
      "total": 100,
      "total_pages": 5
    }
  }
}
```

### POST /bills - 创建账单

**请求体:**
```json
{
  "ledger_id": "uuid",
  "category_id": "uuid",
  "type": "expense",
  "amount": 25.50,
  "note": "午餐",
  "bill_date": "2025-01-15"
}
```

### GET /bills/:id - 获取账单详情

### PUT /bills/:id - 更新账单

### DELETE /bills/:id - 删除账单

### GET /bills/statistics - 账单统计

**查询参数:**
- `ledger_id`: 账本ID (必填)
- `start_date`: 开始日期
- `end_date`: 结束日期
- `group_by`: `day` | `week` | `month` | `year` | `category`

**响应:**
```json
{
  "code": 0,
  "data": {
    "total_income": 10000.00,
    "total_expense": 5000.00,
    "balance": 5000.00,
    "items": [
      {
        "date": "2025-01",
        "income": 10000.00,
        "expense": 5000.00
      }
    ],
    "by_category": [
      {
        "category_id": "uuid",
        "category_name": "餐饮",
        "amount": 1500.00,
        "percentage": 30.0
      }
    ]
  }
}
```

---

## 分类模块 `/categories`

### GET /categories - 获取分类列表

**查询参数:**
- `ledger_id`: 账本ID (可选，不传返回系统默认分类)
- `type`: `income` | `expense`

**响应:**
```json
{
  "code": 0,
  "data": {
    "items": [
      {
        "id": "uuid",
        "name": "餐饮",
        "icon": "food",
        "type": "expense",
        "children": [
          {
            "id": "uuid",
            "name": "早餐",
            "icon": "breakfast"
          }
        ]
      }
    ]
  }
}
```

### POST /categories - 创建自定义分类

### PUT /categories/:id - 更新分类

### DELETE /categories/:id - 删除分类

---

## 群组模块 `/groups`

### GET /groups - 获取我的群组列表

**响应:**
```json
{
  "code": 0,
  "data": {
    "items": [
      {
        "id": "uuid",
        "name": "我的家庭",
        "description": "家庭共享账本",
        "member_count": 3,
        "my_role": "owner",
        "created_at": "2025-01-01T00:00:00Z"
      }
    ]
  }
}
```

### POST /groups - 创建群组

**请求体:**
```json
{
  "name": "我的家庭",
  "description": "家庭共享账本"
}
```

**响应:**
```json
{
  "code": 0,
  "data": {
    "id": "uuid",
    "name": "我的家庭",
    "invite_code": "ABC123"
  }
}
```

### GET /groups/:id - 获取群组详情

**响应:**
```json
{
  "code": 0,
  "data": {
    "id": "uuid",
    "name": "我的家庭",
    "description": "家庭共享账本",
    "owner": {
      "id": "uuid",
      "nickname": "张三"
    },
    "members": [
      {
        "user_id": "uuid",
        "nickname": "李四",
        "role": "member",
        "joined_at": "2025-01-02T00:00:00Z"
      }
    ],
    "ledgers": [
      {
        "id": "uuid",
        "name": "家庭账本"
      }
    ]
  }
}
```

### PUT /groups/:id - 更新群组信息

**权限:** owner, admin

### DELETE /groups/:id - 解散群组

**权限:** owner

### POST /groups/:id/invite-code - 重置邀请码

**权限:** owner, admin

### POST /groups/join - 通过邀请码加入群组

**请求体:**
```json
{
  "invite_code": "ABC123"
}
```

### DELETE /groups/:id/members/:user_id - 移除成员

**权限:** owner, admin (不能移除 owner)

### PUT /groups/:id/members/:user_id/role - 修改成员角色

**权限:** owner

**请求体:**
```json
{
  "role": "admin"
}
```

### POST /groups/:id/leave - 退出群组

**限制:** owner 不能退出，需先转让

### POST /groups/:id/transfer - 转让群组

**权限:** owner

**请求体:**
```json
{
  "new_owner_id": "uuid"
}
```

---

## 群组账本 `/groups/:group_id/ledgers`

### POST /groups/:group_id/ledgers - 创建群组账本

**权限:** owner, admin

**请求体:**
```json
{
  "name": "家庭日常开支",
  "description": "家庭日常生活支出"
}
```

群组账本的账单操作复用 `/bills` 接口，通过 `ledger_id` 关联。

# 数据库设计

## 概述

本项目支持 MySQL 和 MongoDB 两种数据库，通过 Repository Pattern 实现切换。

## 实体关系图 (ERD)

```
┌──────────────┐       ┌──────────────────┐       ┌──────────────┐
│    users     │       │  group_members   │       │    groups    │
├──────────────┤       ├──────────────────┤       ├──────────────┤
│ id (PK)      │──┐    │ id (PK)          │    ┌──│ id (PK)      │
│ email        │  │    │ user_id (FK)     │────┘  │ name         │
│ password     │  └───>│ group_id (FK)    │───────│ owner_id(FK) │
│ nickname     │       │ role             │       │ created_at   │
│ avatar       │       │ joined_at        │       └──────────────┘
│ created_at   │       └──────────────────┘              │
│ updated_at   │                                         │
└──────┬───────┘                                         │
       │                                                 │
       │         ┌──────────────┐                        │
       │         │   ledgers    │                        │
       │         ├──────────────┤                        │
       │         │ id (PK)      │                        │
       └────────>│ user_id (FK) │ (个人账本)              │
                 │ group_id(FK) │<───────────────────────┘ (群组账本)
                 │ name         │
                 │ type         │ (personal/group)
                 │ created_at   │
                 └──────┬───────┘
                        │
                        │
       ┌────────────────┴────────────────┐
       │                                 │
       ▼                                 ▼
┌──────────────┐                 ┌──────────────┐
│    bills     │                 │  categories  │
├──────────────┤                 ├──────────────┤
│ id (PK)      │                 │ id (PK)      │
│ ledger_id(FK)│                 │ ledger_id(FK)│ (可为空=系统默认)
│ category_id  │────────────────>│ name         │
│ user_id (FK) │ (记录人)        │ icon         │
│ type         │ (income/expense)│ type         │ (income/expense)
│ amount       │                 │ parent_id    │ (支持子分类)
│ note         │                 │ sort_order   │
│ bill_date    │                 └──────────────┘
│ created_at   │
│ updated_at   │
└──────────────┘
```

## MySQL Schema

### users 表

```sql
CREATE TABLE users (
    id VARCHAR(36) PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    nickname VARCHAR(100),
    avatar VARCHAR(500),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_email (email)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
```

### groups 表

```sql
CREATE TABLE `groups` (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    owner_id VARCHAR(36) NOT NULL,
    invite_code VARCHAR(20) UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (owner_id) REFERENCES users(id) ON DELETE CASCADE,
    INDEX idx_owner (owner_id),
    INDEX idx_invite_code (invite_code)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
```

### group_members 表

```sql
CREATE TABLE group_members (
    id VARCHAR(36) PRIMARY KEY,
    group_id VARCHAR(36) NOT NULL,
    user_id VARCHAR(36) NOT NULL,
    role ENUM('owner', 'admin', 'member') DEFAULT 'member',
    joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (group_id) REFERENCES `groups`(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE KEY uk_group_user (group_id, user_id),
    INDEX idx_user (user_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
```

### ledgers 表

```sql
CREATE TABLE ledgers (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    type ENUM('personal', 'group') DEFAULT 'personal',
    user_id VARCHAR(36),           -- 个人账本所有者
    group_id VARCHAR(36),          -- 群组账本所属群组
    currency VARCHAR(10) DEFAULT 'CNY',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES `groups`(id) ON DELETE CASCADE,
    INDEX idx_user (user_id),
    INDEX idx_group (group_id),
    CHECK (
        (type = 'personal' AND user_id IS NOT NULL AND group_id IS NULL) OR
        (type = 'group' AND group_id IS NOT NULL)
    )
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
```

### categories 表

```sql
CREATE TABLE categories (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    icon VARCHAR(100),
    type ENUM('income', 'expense') NOT NULL,
    parent_id VARCHAR(36),         -- 父分类ID，支持二级分类
    ledger_id VARCHAR(36),         -- NULL 表示系统默认分类
    sort_order INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES categories(id) ON DELETE SET NULL,
    FOREIGN KEY (ledger_id) REFERENCES ledgers(id) ON DELETE CASCADE,
    INDEX idx_ledger (ledger_id),
    INDEX idx_type (type)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
```

### bills 表

```sql
CREATE TABLE bills (
    id VARCHAR(36) PRIMARY KEY,
    ledger_id VARCHAR(36) NOT NULL,
    category_id VARCHAR(36) NOT NULL,
    user_id VARCHAR(36) NOT NULL,  -- 记录人
    type ENUM('income', 'expense') NOT NULL,
    amount DECIMAL(12, 2) NOT NULL,
    note TEXT,
    bill_date DATE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (ledger_id) REFERENCES ledgers(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE RESTRICT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    INDEX idx_ledger (ledger_id),
    INDEX idx_user (user_id),
    INDEX idx_date (bill_date),
    INDEX idx_ledger_date (ledger_id, bill_date)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
```

### refresh_tokens 表

```sql
CREATE TABLE refresh_tokens (
    id VARCHAR(36) PRIMARY KEY,
    user_id VARCHAR(36) NOT NULL,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    INDEX idx_user (user_id),
    INDEX idx_token (token_hash)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;
```

## MongoDB Schema

### users 集合

```javascript
{
  _id: ObjectId,
  email: String,           // unique index
  password_hash: String,
  nickname: String,
  avatar: String,
  created_at: Date,
  updated_at: Date
}
```

### groups 集合

```javascript
{
  _id: ObjectId,
  name: String,
  description: String,
  owner_id: ObjectId,      // ref: users
  invite_code: String,     // unique index
  members: [               // 嵌入式文档
    {
      user_id: ObjectId,
      role: String,        // 'owner' | 'admin' | 'member'
      joined_at: Date
    }
  ],
  created_at: Date,
  updated_at: Date
}
```

### ledgers 集合

```javascript
{
  _id: ObjectId,
  name: String,
  description: String,
  type: String,            // 'personal' | 'group'
  user_id: ObjectId,       // 个人账本
  group_id: ObjectId,      // 群组账本
  currency: String,
  created_at: Date,
  updated_at: Date
}
```

### categories 集合

```javascript
{
  _id: ObjectId,
  name: String,
  icon: String,
  type: String,            // 'income' | 'expense'
  parent_id: ObjectId,     // 父分类
  ledger_id: ObjectId,     // null = 系统默认
  sort_order: Number,
  created_at: Date
}
```

### bills 集合

```javascript
{
  _id: ObjectId,
  ledger_id: ObjectId,
  category_id: ObjectId,
  user_id: ObjectId,       // 记录人
  type: String,            // 'income' | 'expense'
  amount: Decimal128,
  note: String,
  bill_date: Date,
  created_at: Date,
  updated_at: Date
}

// 索引
db.bills.createIndex({ ledger_id: 1, bill_date: -1 })
db.bills.createIndex({ user_id: 1 })
```

## 默认分类数据

```javascript
// 支出分类
const expenseCategories = [
  { name: '餐饮', icon: 'food', sort_order: 1 },
  { name: '交通', icon: 'transport', sort_order: 2 },
  { name: '购物', icon: 'shopping', sort_order: 3 },
  { name: '娱乐', icon: 'entertainment', sort_order: 4 },
  { name: '居住', icon: 'housing', sort_order: 5 },
  { name: '医疗', icon: 'medical', sort_order: 6 },
  { name: '教育', icon: 'education', sort_order: 7 },
  { name: '通讯', icon: 'communication', sort_order: 8 },
  { name: '其他', icon: 'other', sort_order: 99 },
];

// 收入分类
const incomeCategories = [
  { name: '工资', icon: 'salary', sort_order: 1 },
  { name: '奖金', icon: 'bonus', sort_order: 2 },
  { name: '投资', icon: 'investment', sort_order: 3 },
  { name: '兼职', icon: 'part-time', sort_order: 4 },
  { name: '红包', icon: 'red-packet', sort_order: 5 },
  { name: '其他', icon: 'other', sort_order: 99 },
];
```

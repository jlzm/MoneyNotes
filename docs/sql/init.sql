-- Money Notes 数据库初始化脚本
-- 在 Docker 首次启动时自动执行

SET NAMES utf8mb4;
SET CHARACTER SET utf8mb4;

-- ================================
-- 用户表
-- ================================
CREATE TABLE IF NOT EXISTS users (
    id VARCHAR(36) PRIMARY KEY,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    nickname VARCHAR(100),
    avatar VARCHAR(500),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_email (email)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ================================
-- 群组表
-- ================================
CREATE TABLE IF NOT EXISTS `groups` (
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
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ================================
-- 群组成员表
-- ================================
CREATE TABLE IF NOT EXISTS group_members (
    id VARCHAR(36) PRIMARY KEY,
    group_id VARCHAR(36) NOT NULL,
    user_id VARCHAR(36) NOT NULL,
    role ENUM('owner', 'admin', 'member') DEFAULT 'member',
    joined_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (group_id) REFERENCES `groups`(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    UNIQUE KEY uk_group_user (group_id, user_id),
    INDEX idx_user (user_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ================================
-- 账本表
-- ================================
CREATE TABLE IF NOT EXISTS ledgers (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    type ENUM('personal', 'group') DEFAULT 'personal',
    currency VARCHAR(10) DEFAULT 'CNY',
    user_id VARCHAR(36),
    group_id VARCHAR(36),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES `groups`(id) ON DELETE CASCADE,
    INDEX idx_user (user_id),
    INDEX idx_group (group_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ================================
-- 分类表
-- ================================
CREATE TABLE IF NOT EXISTS categories (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    icon VARCHAR(50),
    type ENUM('income', 'expense') NOT NULL,
    parent_id VARCHAR(36),
    ledger_id VARCHAR(36),
    sort_order INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES categories(id) ON DELETE SET NULL,
    FOREIGN KEY (ledger_id) REFERENCES ledgers(id) ON DELETE CASCADE,
    INDEX idx_type (type),
    INDEX idx_ledger (ledger_id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ================================
-- 账单表
-- ================================
CREATE TABLE IF NOT EXISTS bills (
    id VARCHAR(36) PRIMARY KEY,
    ledger_id VARCHAR(36) NOT NULL,
    category_id VARCHAR(36) NOT NULL,
    user_id VARCHAR(36) NOT NULL,
    type ENUM('income', 'expense') NOT NULL,
    amount DECIMAL(15, 2) NOT NULL,
    note TEXT,
    bill_date DATE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (ledger_id) REFERENCES ledgers(id) ON DELETE CASCADE,
    FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE RESTRICT,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    INDEX idx_ledger_date (ledger_id, bill_date),
    INDEX idx_user (user_id),
    INDEX idx_category (category_id),
    INDEX idx_type (type)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- ================================
-- 默认分类数据
-- ================================
INSERT INTO categories (id, name, icon, type, sort_order) VALUES
-- 支出分类
(UUID(), '餐饮', 'food', 'expense', 1),
(UUID(), '交通', 'transport', 'expense', 2),
(UUID(), '购物', 'shopping', 'expense', 3),
(UUID(), '娱乐', 'entertainment', 'expense', 4),
(UUID(), '居住', 'housing', 'expense', 5),
(UUID(), '医疗', 'medical', 'expense', 6),
(UUID(), '教育', 'education', 'expense', 7),
(UUID(), '通讯', 'communication', 'expense', 8),
(UUID(), '其他支出', 'other', 'expense', 99),
-- 收入分类
(UUID(), '工资', 'salary', 'income', 1),
(UUID(), '奖金', 'bonus', 'income', 2),
(UUID(), '投资', 'investment', 'income', 3),
(UUID(), '兼职', 'parttime', 'income', 4),
(UUID(), '其他收入', 'other', 'income', 99);

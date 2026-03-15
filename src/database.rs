//! # 数据库集成模块
//!
//! 这个模块演示了 Rust 中数据库集成的基础实践，包括 SQLx 异步数据库操作、
//! 连接池管理、联表查询映射、事务边界和仓储风格封装。
//!
//! ## 当前教学路线
//! - 本模块统一采用 **SQLite + SQLx** 语义
//! - 默认连接串、示例文案、连接池类型与 SQL 占位符均保持一致
//! - 为了让示例可重复运行，演示默认使用内存数据库

use std::time::Duration;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, Row, SqlitePool};
use thiserror::Error;

/// 数据库模块统一错误类型。
#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("数据库操作失败: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("数据校验失败: {0}")]
    Validation(String),
}

/// 现代化数据库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub connection_timeout: u64,
    pub idle_timeout: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            // SQLite 内存库配合单连接，便于教学演示且可重复运行。
            url: "sqlite::memory:".to_string(),
            max_connections: 1,
            connection_timeout: 30,
            idle_timeout: 600,
        }
    }
}

/// 用户数据结构
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: Option<NaiveDateTime>,
    pub is_active: bool,
}

/// 帖子数据结构
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

/// 用于联表查询的中间映射结构。
#[derive(Debug, sqlx::FromRow)]
struct PostWithUserRow {
    post_id: i64,
    post_user_id: i64,
    post_title: String,
    post_content: String,
    post_created_at: Option<NaiveDateTime>,
    post_updated_at: Option<NaiveDateTime>,
    user_id: i64,
    user_username: String,
    user_email: String,
    user_created_at: Option<NaiveDateTime>,
    user_is_active: bool,
}

/// 用户统计数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub post_count: u32,
    pub total_content_length: u32,
    pub created_at: Option<NaiveDateTime>,
    pub is_active: bool,
}

/// 现代化数据库管理器
#[derive(Debug, Clone)]
pub struct DatabaseManager {
    pool: SqlitePool,
}

impl DatabaseManager {
    /// 创建新的数据库管理器
    pub async fn new(config: DatabaseConfig) -> Result<Self, DatabaseError> {
        let pool = SqlitePoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(Duration::from_secs(config.connection_timeout))
            .idle_timeout(Some(Duration::from_secs(config.idle_timeout)))
            .connect(&config.url)
            .await?;

        Ok(Self { pool })
    }

    /// 初始化数据库表结构
    pub async fn init_schema(&self) -> Result<(), DatabaseError> {
        // SQLite 表结构：字段类型、默认值和主键策略都与当前连接池实现保持一致。
        let create_users_table = r#"
            CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                is_active INTEGER NOT NULL DEFAULT 1
            );
        "#;

        let create_posts_table = r#"
            CREATE TABLE IF NOT EXISTS posts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                user_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP,
                FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
            );
        "#;

        sqlx::query("PRAGMA foreign_keys = ON;")
            .execute(&self.pool)
            .await?;
        sqlx::query(create_users_table).execute(&self.pool).await?;
        sqlx::query(create_posts_table).execute(&self.pool).await?;

        println!("✅ SQLite 数据库表结构初始化完成");
        Ok(())
    }

    /// 创建用户
    pub async fn create_user(&self, username: &str, email: &str) -> Result<User, DatabaseError> {
        let result = sqlx::query(
            r#"
                INSERT INTO users (username, email)
                VALUES (?, ?)
            "#,
        )
        .bind(username)
        .bind(email)
        .execute(&self.pool)
        .await?;

        let user = self.fetch_user_by_id(result.last_insert_rowid()).await?;
        println!("✅ 用户创建成功: {}", username);
        Ok(user)
    }

    /// 根据 ID 获取用户
    pub async fn get_user(&self, id: i64) -> Result<Option<User>, DatabaseError> {
        let user = sqlx::query_as::<_, User>(
            r#"
                SELECT id, username, email, created_at, is_active
                FROM users
                WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    /// 获取所有用户
    pub async fn get_all_users(&self) -> Result<Vec<User>, DatabaseError> {
        let users = sqlx::query_as::<_, User>(
            r#"
                SELECT id, username, email, created_at, is_active
                FROM users
                ORDER BY created_at DESC, id DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    /// 更新用户
    pub async fn update_user(
        &self,
        id: i64,
        username: &str,
        email: &str,
    ) -> Result<User, DatabaseError> {
        sqlx::query(
            r#"
                UPDATE users
                SET username = ?, email = ?
                WHERE id = ?
            "#,
        )
        .bind(username)
        .bind(email)
        .bind(id)
        .execute(&self.pool)
        .await?;

        let user = self.fetch_user_by_id(id).await?;
        println!("✅ 用户更新成功: {}", username);
        Ok(user)
    }

    /// 删除用户
    pub async fn delete_user(&self, id: i64) -> Result<bool, DatabaseError> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        let deleted = result.rows_affected() > 0;
        if deleted {
            println!("✅ 用户删除成功: {}", id);
        }

        Ok(deleted)
    }

    /// 创建帖子
    pub async fn create_post(
        &self,
        user_id: i64,
        title: &str,
        content: &str,
    ) -> Result<Post, DatabaseError> {
        let result = sqlx::query(
            r#"
                INSERT INTO posts (user_id, title, content)
                VALUES (?, ?, ?)
            "#,
        )
        .bind(user_id)
        .bind(title)
        .bind(content)
        .execute(&self.pool)
        .await?;

        let post = self.fetch_post_by_id(result.last_insert_rowid()).await?;
        println!("✅ 帖子创建成功: {}", title);
        Ok(post)
    }

    /// 获取用户的帖子
    pub async fn get_user_posts(&self, user_id: i64) -> Result<Vec<Post>, DatabaseError> {
        let posts = sqlx::query_as::<_, Post>(
            r#"
                SELECT id, user_id, title, content, created_at, updated_at
                FROM posts
                WHERE user_id = ?
                ORDER BY created_at DESC, id DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }

    /// 获取所有帖子（包含用户信息）
    pub async fn get_all_posts_with_users(&self) -> Result<Vec<(Post, User)>, DatabaseError> {
        let rows = sqlx::query_as::<_, PostWithUserRow>(
            r#"
                SELECT
                    p.id AS post_id,
                    p.user_id AS post_user_id,
                    p.title AS post_title,
                    p.content AS post_content,
                    p.created_at AS post_created_at,
                    p.updated_at AS post_updated_at,
                    u.id AS user_id,
                    u.username AS user_username,
                    u.email AS user_email,
                    u.created_at AS user_created_at,
                    u.is_active AS user_is_active
                FROM posts p
                INNER JOIN users u ON p.user_id = u.id
                ORDER BY p.created_at DESC, p.id DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let posts_with_users = rows
            .into_iter()
            .map(|row| {
                let post = Post {
                    id: row.post_id,
                    user_id: row.post_user_id,
                    title: row.post_title,
                    content: row.post_content,
                    created_at: row.post_created_at,
                    updated_at: row.post_updated_at,
                };
                let user = User {
                    id: row.user_id,
                    username: row.user_username,
                    email: row.user_email,
                    created_at: row.user_created_at,
                    is_active: row.user_is_active,
                };
                (post, user)
            })
            .collect();

        Ok(posts_with_users)
    }

    /// 事务操作示例：创建用户并附带一篇默认帖子。
    pub async fn create_user_with_default_post(
        &self,
        username: &str,
        email: &str,
    ) -> Result<(User, Post), DatabaseError> {
        let mut tx = self.pool.begin().await?;

        let user_insert = sqlx::query(
            r#"
                INSERT INTO users (username, email)
                VALUES (?, ?)
            "#,
        )
        .bind(username)
        .bind(email)
        .execute(&mut *tx)
        .await?;
        let user_id = user_insert.last_insert_rowid();

        let post_insert = sqlx::query(
            r#"
                INSERT INTO posts (user_id, title, content)
                VALUES (?, ?, ?)
            "#,
        )
        .bind(user_id)
        .bind("欢迎来到我的博客！")
        .bind(format!("这是 {} 的第一篇帖子", username))
        .execute(&mut *tx)
        .await?;
        let post_id = post_insert.last_insert_rowid();

        let user = sqlx::query_as::<_, User>(
            r#"
                SELECT id, username, email, created_at, is_active
                FROM users
                WHERE id = ?
            "#,
        )
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await?;

        let post = sqlx::query_as::<_, Post>(
            r#"
                SELECT id, user_id, title, content, created_at, updated_at
                FROM posts
                WHERE id = ?
            "#,
        )
        .bind(post_id)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        println!("✅ 事务操作成功: 用户和默认帖子已创建");
        Ok((user, post))
    }

    /// 批量操作示例：整批写入在一个事务中完成，避免半成功状态。
    pub async fn bulk_create_users(
        &self,
        users: &[(&str, &str)],
    ) -> Result<Vec<User>, DatabaseError> {
        let mut tx = self.pool.begin().await?;
        let mut created_users = Vec::with_capacity(users.len());

        for (username, email) in users {
            let insert_result = sqlx::query(
                r#"
                    INSERT INTO users (username, email)
                    VALUES (?, ?)
                "#,
            )
            .bind(username)
            .bind(email)
            .execute(&mut *tx)
            .await?;

            let user = sqlx::query_as::<_, User>(
                r#"
                    SELECT id, username, email, created_at, is_active
                    FROM users
                    WHERE id = ?
                "#,
            )
            .bind(insert_result.last_insert_rowid())
            .fetch_one(&mut *tx)
            .await?;

            created_users.push(user);
        }

        tx.commit().await?;
        println!("✅ 批量创建用户完成: {} 个用户", created_users.len());
        Ok(created_users)
    }

    /// 复杂查询示例
    pub async fn get_active_user_stats(&self) -> Result<Vec<UserStats>, DatabaseError> {
        let rows = sqlx::query(
            r#"
                SELECT
                    u.id,
                    u.username,
                    u.email,
                    COUNT(p.id) AS post_count,
                    COALESCE(SUM(LENGTH(p.content)), 0) AS total_content_length,
                    u.created_at,
                    u.is_active
                FROM users u
                LEFT JOIN posts p ON u.id = p.user_id
                WHERE u.is_active = 1
                GROUP BY u.id, u.username, u.email, u.created_at, u.is_active
                ORDER BY post_count DESC, u.id DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        let user_stats = rows
            .into_iter()
            .map(|row| UserStats {
                user_id: row.get("id"),
                username: row.get("username"),
                email: row.get("email"),
                post_count: row.get::<i64, _>("post_count") as u32,
                total_content_length: row.get::<i64, _>("total_content_length") as u32,
                created_at: row.get("created_at"),
                is_active: row.get("is_active"),
            })
            .collect();

        Ok(user_stats)
    }

    async fn fetch_user_by_id(&self, id: i64) -> Result<User, DatabaseError> {
        sqlx::query_as::<_, User>(
            r#"
                SELECT id, username, email, created_at, is_active
                FROM users
                WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(DatabaseError::from)
    }

    async fn fetch_post_by_id(&self, id: i64) -> Result<Post, DatabaseError> {
        sqlx::query_as::<_, Post>(
            r#"
                SELECT id, user_id, title, content, created_at, updated_at
                FROM posts
                WHERE id = ?
            "#,
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(DatabaseError::from)
    }
}

/// 数据库连接池演示
pub async fn demonstrate_connection_pool() {
    println!("🌊 数据库连接池演示:");

    let config = DatabaseConfig::default();
    let db_manager = DatabaseManager::new(config).await;

    match db_manager {
        Ok(manager) => {
            if let Err(error) = manager.init_schema().await {
                println!("❌ 数据库初始化失败: {}", error);
                return;
            }

            let users = vec![
                ("张三", "zhangsan@example.com"),
                ("李四", "lisi@example.com"),
                ("王五", "wangwu@example.com"),
            ];

            let created_users = match manager.bulk_create_users(&users).await {
                Ok(users) => users,
                Err(error) => {
                    println!("❌ 批量创建用户失败: {}", error);
                    return;
                }
            };

            for (i, user) in created_users.iter().enumerate() {
                let titles = ["第一篇帖子", "第二篇帖子", "第三篇帖子"];
                for title in &titles[i % titles.len()..] {
                    let content = format!("这是关于 {} 的内容", title);
                    if let Err(error) = manager.create_post(user.id, title, &content).await {
                        println!("❌ 创建帖子失败: {}", error);
                    }
                }
            }

            println!("\n👥 所有用户:");
            match manager.get_all_users().await {
                Ok(users) => {
                    for user in users {
                        println!("  - {} ({})", user.username, user.email);
                    }
                }
                Err(error) => println!("❌ 获取用户失败: {}", error),
            }

            println!("\n📝 帖子与作者映射:");
            match manager.get_all_posts_with_users().await {
                Ok(posts) => {
                    for (post, user) in posts {
                        println!("  - {} -> {}", post.title, user.username);
                    }
                }
                Err(error) => println!("❌ 获取帖子与作者失败: {}", error),
            }

            println!("\n📊 用户统计:");
            match manager.get_active_user_stats().await {
                Ok(stats) => {
                    for stat in stats {
                        println!(
                            "  - {}: {}篇帖子, {}字符",
                            stat.username, stat.post_count, stat.total_content_length
                        );
                    }
                }
                Err(error) => println!("❌ 获取统计失败: {}", error),
            }

            println!("\n🔄 事务操作演示:");
            match manager
                .create_user_with_default_post("新用户", "newuser@example.com")
                .await
            {
                Ok((user, post)) => {
                    println!("  ✅ 用户: {}", user.username);
                    println!("  ✅ 帖子: {}", post.title);
                }
                Err(error) => println!("❌ 事务失败: {}", error),
            }
        }
        Err(error) => {
            println!("❌ 数据库连接失败: {}", error);
            println!("💡 当前示例统一使用 SQLite；如需落地到文件库，可将连接串改为 sqlite://demo.db");
        }
    }
}

/// ORM 风格操作演示
pub async fn demonstrate_orm_operations() {
    println!("🔧 ORM 风格操作演示:");

    #[derive(Debug)]
    struct UserRepository {
        db: DatabaseManager,
    }

    impl UserRepository {
        fn new(db: DatabaseManager) -> Self {
            Self { db }
        }

        async fn find_by_username(&self, username: &str) -> Result<Option<User>, DatabaseError> {
            let users = self.db.get_all_users().await?;
            Ok(users.into_iter().find(|user| user.username == username))
        }

        async fn find_by_email(&self, email: &str) -> Result<Option<User>, DatabaseError> {
            let users = self.db.get_all_users().await?;
            Ok(users.into_iter().find(|user| user.email == email))
        }

        async fn create_user_with_validation(
            &self,
            username: &str,
            email: &str,
        ) -> Result<User, DatabaseError> {
            if username.len() < 3 {
                return Err(DatabaseError::Validation("用户名至少需要 3 个字符".to_string()));
            }

            if !email.contains('@') {
                return Err(DatabaseError::Validation("邮箱格式无效".to_string()));
            }

            if self.find_by_username(username).await?.is_some() {
                return Err(DatabaseError::Validation("用户名已存在".to_string()));
            }

            if self.find_by_email(email).await?.is_some() {
                return Err(DatabaseError::Validation("邮箱已被注册".to_string()));
            }

            self.db.create_user(username, email).await
        }
    }

    let config = DatabaseConfig::default();
    let db = match DatabaseManager::new(config).await {
        Ok(manager) => manager,
        Err(error) => {
            println!("❌ ORM 示例初始化失败: {}", error);
            return;
        }
    };

    if let Err(error) = db.init_schema().await {
        println!("❌ ORM 示例建表失败: {}", error);
        return;
    }

    let repository = UserRepository::new(db);
    match repository
        .create_user_with_validation("仓储用户", "repository@example.com")
        .await
    {
        Ok(user) => println!("  ✅ 已通过仓储创建用户: {}", user.username),
        Err(error) => println!("  ❌ 仓储创建失败: {}", error),
    }

    println!("💾 ORM 风格操作示例：");
    println!("  - find_by_username: 按用户名查找");
    println!("  - find_by_email: 按邮箱查找");
    println!("  - create_user_with_validation: 验证后创建用户");
    println!("  ✅ ORM 基础设施已准备就绪");
}

/// 运行数据库集成示例
pub async fn run_database_examples() {
    println!("🎯 === SQLite 数据库集成示例 ===");
    println!();

    demonstrate_connection_pool().await;
    println!();

    demonstrate_orm_operations().await;

    println!("\n✅ 所有数据库集成示例运行完成！");
    println!("💡 这些示例展示了使用 SQLite + SQLx 的基础实践");
}

//! # 数据库集成模块
//!
//! 这个模块演示了Rust中数据库集成的最佳实践，包括SQLx异步数据库操作、
//! 事务处理、连接池管理、ORM使用等数据库开发的关键要素。
//! 采用了现代化的Rust 2021/2024最佳实践。

use serde::{Deserialize, Serialize};

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
            url: "sqlite:memory:".to_string(), // 默认使用内存数据库
            max_connections: 10,
            connection_timeout: 30,
            idle_timeout: 600,
        }
    }
}

/// 用户数据结构
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Option<i64>,
    pub username: String,
    pub email: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
}

/// 帖子数据结构
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: Option<i64>,
    pub user_id: i64,
    pub title: String,
    pub content: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// 现代化数据库管理器
#[derive(Debug)]
pub struct DatabaseManager {
    pool: sqlx::PgPool,
}

impl DatabaseManager {
    /// 创建新的数据库管理器
    pub async fn new(config: DatabaseConfig) -> Result<Self, sqlx::Error> {
        let pool = sqlx::PgPool::connect(&config.url).await?;
        Ok(Self { pool })
    }
    
    /// 初始化数据库表结构
    pub async fn init_schema(&self) -> Result<(), sqlx::Error> {
        // PostgreSQL表结构
        let create_users_table = r#"
            CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                username TEXT UNIQUE NOT NULL,
                email TEXT UNIQUE NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                is_active BOOLEAN DEFAULT 1
            );
        "#;
        
        let create_posts_table = r#"
            CREATE TABLE IF NOT EXISTS posts (
                id SERIAL PRIMARY KEY,
                user_id INTEGER NOT NULL REFERENCES users(id),
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
        "#;
        
        // 执行表创建
        sqlx::query(create_users_table).execute(&self.pool).await?;
        sqlx::query(create_posts_table).execute(&self.pool).await?;
        
        println!("✅ 数据库表结构初始化完成");
        Ok(())
    }
    
    /// 创建用户
    pub async fn create_user(&self, username: &str, email: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (username, email) VALUES (?, ?) RETURNING *",
            username,
            email
        )
        .fetch_one(&self.pool)
        .await?;
        
        println!("✅ 用户创建成功: {}", username);
        Ok(user)
    }
    
    /// 根据ID获取用户
    pub async fn get_user(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(user)
    }
    
    /// 获取所有用户
    pub async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(User, "SELECT * FROM users ORDER BY created_at DESC")
            .fetch_all(&self.pool)
            .await?;
        
        Ok(users)
    }
    
    /// 更新用户
    pub async fn update_user(&self, id: i64, username: &str, email: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            "UPDATE users SET username = ?, email = ? WHERE id = ? RETURNING *",
            username,
            email,
            id
        )
        .fetch_one(&self.pool)
        .await?;
        
        println!("✅ 用户更新成功: {}", username);
        Ok(user)
    }
    
    /// 删除用户
    pub async fn delete_user(&self, id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
            .execute(&self.pool)
            .await?;
        
        let deleted = result.rows_affected() > 0;
        if deleted {
            println!("✅ 用户删除成功: {}", id);
        }
        
        Ok(deleted)
    }
    
    /// 创建帖子
    pub async fn create_post(&self, user_id: i64, title: &str, content: &str) -> Result<Post, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            "INSERT INTO posts (user_id, title, content) VALUES (?, ?, ?) RETURNING *",
            user_id,
            title,
            content
        )
        .fetch_one(&self.pool)
        .await?;
        
        println!("✅ 帖子创建成功: {}", title);
        Ok(post)
    }
    
    /// 获取用户的帖子
    pub async fn get_user_posts(&self, user_id: i64) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as!(
            Post,
            "SELECT * FROM posts WHERE user_id = ? ORDER BY created_at DESC",
            user_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(posts)
    }
    
    /// 获取所有帖子（包含用户信息）
    pub async fn get_all_posts_with_users(&self) -> Result<Vec<(Post, User)>, sqlx::Error> {
        let posts = sqlx::query!(
            r#"
                SELECT p.id, p.user_id, p.title, p.content, p.created_at, p.updated_at,
                       u.id, u.username, u.email, u.created_at, u.is_active
                FROM posts p
                INNER JOIN users u ON p.user_id = u.id
                ORDER BY p.created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        let posts_with_users: Vec<(Post, User)> = posts
            .into_iter()
            .map(|row| {
                let post = Post {
                    id: row.id,
                    user_id: row.user_id,
                    title: row.title,
                    content: row.content,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                };
                let user = User {
                    id: row.id,
                    username: row.username,
                    email: row.email,
                    created_at: row.created_at,
                    is_active: row.is_active,
                };
                (post, user)
            })
            .collect();
        
        Ok(posts_with_users)
    }
    
    /// 事务操作示例
    pub async fn create_user_with_default_post(&self, username: &str, email: &str) -> Result<(User, Post), Box<dyn std::error::Error>> {
        let mut tx = self.pool.begin().await?;
        
        // 在事务中创建用户
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (username, email) VALUES (?, ?) RETURNING *",
            username,
            email
        )
        .fetch_one(&mut *tx)
        .await?;
        
        // 为用户创建默认帖子
        let post = sqlx::query_as!(
            Post,
            "INSERT INTO posts (user_id, title, content) VALUES (?, ?, ?) RETURNING *",
            user.id.unwrap(),
            "欢迎来到我的博客！",
            format!("这是 {} 的第一篇帖子", username)
        )
        .fetch_one(&mut *tx)
        .await?;
        
        tx.commit().await?;
        
        println!("✅ 事务操作成功: 用户和默认帖子已创建");
        Ok((user, post))
    }
    
    /// 批量操作示例
    pub async fn bulk_create_users(&self, users: &[(&str, &str)]) -> Result<Vec<User>, Box<dyn std::error::Error>> {
        let mut created_users = Vec::new();
        
        for (username, email) in users {
            match self.create_user(username, email).await {
                Ok(user) => created_users.push(user),
                Err(e) => {
                    println!("❌ 创建用户 {} 失败: {}", username, e);
                    return Err(e.into());
                }
            }
        }
        
        println!("✅ 批量创建用户完成: {} 个用户", created_users.len());
        Ok(created_users)
    }
    
    /// 复杂查询示例
    pub async fn get_active_user_stats(&self) -> Result<Vec<UserStats>, sqlx::Error> {
        let stats = sqlx::query!(
            r#"
                SELECT u.id, u.username, u.email,
                       COUNT(p.id) as post_count,
                       COALESCE(SUM(LENGTH(p.content)), 0) as total_content_length,
                       u.created_at,
                       u.is_active
                FROM users u
                LEFT JOIN posts p ON u.id = p.user_id
                WHERE u.is_active = 1
                GROUP BY u.id, u.username, u.email, u.created_at, u.is_active
                ORDER BY post_count DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        let user_stats: Vec<UserStats> = stats
            .into_iter()
            .map(|row| UserStats {
                user_id: row.id,
                username: row.username,
                email: row.email,
                post_count: row.post_count as u32,
                total_content_length: row.total_content_length as u32,
                created_at: row.created_at,
                is_active: row.is_active,
            })
            .collect();
        
        Ok(user_stats)
    }
}

/// 用户统计数据结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStats {
    pub user_id: i64,
    pub username: String,
    pub email: String,
    pub post_count: u32,
    pub total_content_length: u32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub is_active: bool,
}

/// 数据库连接池演示
pub async fn demonstrate_connection_pool() {
    println!("🌊 数据库连接池演示:");
    
    let config = DatabaseConfig {
        url: "sqlite:example.db".to_string(),
        max_connections: 5,
        connection_timeout: 10,
        idle_timeout: 300,
    };
    
    let db_manager = DatabaseManager::new(config).await;
    
    match db_manager {
        Ok(manager) => {
            // 初始化数据库结构
            if let Err(e) = manager.init_schema().await {
                println!("❌ 数据库初始化失败: {}", e);
                return;
            }
            
            // 创建测试用户
            let users = vec![
                ("张三", "zhangsan@example.com"),
                ("李四", "lisi@example.com"),
                ("王五", "wangwu@example.com"),
            ];
            
            let created_users = match manager.bulk_create_users(&users).await {
                Ok(users) => users,
                Err(e) => {
                    println!("❌ 批量创建用户失败: {}", e);
                    return;
                }
            };
            
            // 为每个用户创建帖子
            for (i, user) in created_users.iter().enumerate() {
                let titles = ["第一篇帖子", "第二篇帖子", "第三篇帖子"];
                for title in &titles[i % titles.len()..] {
                    let content = format!("这是关于 {} 的内容", title);
                    if let Err(e) = manager.create_post(user.id.unwrap(), title, &content).await {
                        println!("❌ 创建帖子失败: {}", e);
                    }
                }
            }
            
            // 显示所有用户
            println!("\n👥 所有用户:");
            match manager.get_all_users().await {
                Ok(users) => {
                    for user in users {
                        println!("  - {} ({})", user.username, user.email);
                    }
                }
                Err(e) => println!("❌ 获取用户失败: {}", e),
            }
            
            // 显示用户统计
            println!("\n📊 用户统计:");
            match manager.get_active_user_stats().await {
                Ok(stats) => {
                    for stat in stats {
                        println!("  - {}: {}篇帖子, {}字符", 
                                stat.username, 
                                stat.post_count, 
                                stat.total_content_length);
                    }
                }
                Err(e) => println!("❌ 获取统计失败: {}", e),
            }
            
            // 演示事务操作
            println!("\n🔄 事务操作演示:");
            match manager.create_user_with_default_post("新用户", "newuser@example.com").await {
                Ok((user, post)) => {
                    println!("  ✅ 用户: {}", user.username);
                    println!("  ✅ 帖子: {}", post.title);
                }
                Err(e) => println!("❌ 事务失败: {}", e),
            }
        }
        Err(e) => {
            println!("❌ 数据库连接失败: {}", e);
            println!("💡 确保已安装SQLite或PostgreSQL");
        }
    }
}

/// ORM风格操作演示
pub async fn demonstrate_orm_operations() {
    println!("🔧 ORM风格操作演示:");
    
    // 模拟简单的ORM操作
    #[derive(Debug)]
    struct UserRepository {
        db: DatabaseManager,
    }
    
    impl UserRepository {
        fn new(db: DatabaseManager) -> Self {
            Self { db }
        }
        
        async fn find_by_username(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
            let users = self.db.get_all_users().await?;
            Ok(users.into_iter().find(|u| u.username == username))
        }
        
        async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
            let users = self.db.get_all_users().await?;
            Ok(users.into_iter().find(|u| u.email == email))
        }
        
        async fn create_user_with_validation(&self, username: &str, email: &str) -> Result<User, String> {
            // 业务逻辑验证
            if username.len() < 3 {
                return Err("用户名太短".to_string());
            }
            
            if !email.contains('@') {
                return Err("邮箱格式无效".to_string());
            }
            
            // 检查用户名是否已存在
            if let Some(_) = self.find_by_username(username).await.map_err(|e| e.to_string())? {
                return Err("用户名已存在".to_string());
            }
            
            // 检查邮箱是否已存在
            if let Some(_) = self.find_by_email(email).await.map_err(|e| e.to_string())? {
                return Err("邮箱已被注册".to_string());
            }
            
            // 创建用户
            self.db.create_user(username, email)
                .await
                .map_err(|e| e.to_string())
        }
    }
    
    println!("💾 ORM风格操作示例：");
    println!("  - find_by_username: 按用户名查找");
    println!("  - find_by_email: 按邮箱查找");
    println!("  - create_user_with_validation: 验证后创建用户");
    
    // 这里可以添加具体的ORM操作演示
    println!("  ✅ ORM基础设施已准备就绪");
}

/// 运行数据库集成示例
pub async fn run_database_examples() {
    println!("🎯 === 现代化数据库集成示例 ===");
    println!();
    
    demonstrate_connection_pool().await;
    println!();
    
    demonstrate_orm_operations().await;
    
    println!("\n✅ 所有数据库集成示例运行完成！");
    println!("💡 这些示例展示了现代Rust数据库开发的最佳实践");
}

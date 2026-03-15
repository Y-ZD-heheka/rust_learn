//! # 模块和包管理模块
//!
//! 这个模块演示了Rust的模块系统和包管理。
//! 采用了现代化的Rust 2021/2024最佳实践。

#![allow(dead_code)]

/// 现代化模块定义 - 演示农场管理系统
pub mod farm {
    
    pub mod crops {
        use std::collections::HashMap;
        
        /// 现代化作物管理
        #[derive(Debug)]
        pub struct CropManager {
            crops: HashMap<String, usize>,
        }
        
        impl CropManager {
            pub fn new() -> Self {
                Self {
                    crops: HashMap::new(),
                }
            }
            
            pub fn plant(&mut self, crop: &str, quantity: usize) {
                *self.crops.entry(crop.to_string()).or_insert(0) += quantity;
                println!("🌱 种植作物: {} x {}", crop, quantity);
            }
            
            pub fn harvest(&mut self, crop: &str, quantity: usize) -> Option<usize> {
                if let Some(crop_quantity) = self.crops.get_mut(crop) {
                    if *crop_quantity >= quantity {
                        *crop_quantity -= quantity;
                        println!("🌾 收获作物: {} x {}", crop, quantity);
                        return Some(quantity);
                    }
                }
                None
            }
            
            pub fn get_status(&self) -> String {
                let mut status = String::new();
                for (crop, quantity) in &self.crops {
                    if *quantity > 0 {
                        status.push_str(&format!("{}: {}, ", crop, quantity));
                    }
                }
                status
            }
        }
        
        /// 蔬菜子模块
        pub mod vegetables {
            use super::CropManager;
            
            pub fn plant_tomato() {
                println!("🍅 种植番茄");
            }
            
            pub fn plant_carrot() {
                println!("🥕 种植胡萝卜");
            }
            
            pub fn manage_vegetables(manager: &mut CropManager) {
                manager.plant("tomato", 50);
                manager.plant("carrot", 30);
                manager.harvest("tomato", 10);
            }
        }
        
        /// 水果子模块
        pub mod fruits {
            pub fn plant_apple() {
                println!("🍎 种植苹果");
            }
            
            pub fn plant_strawberry() {
                println!("🍓 种植草莓");
            }
            
            pub fn get_fruit_seasons() -> std::collections::HashMap<String, &'static str> {
                let mut seasons = std::collections::HashMap::new();
                seasons.insert("apple".to_string(), "秋季");
                seasons.insert("strawberry".to_string(), "春季");
                seasons
            }
        }
    }
    
    pub mod animals {
        /// 现代化动物管理
        #[derive(Debug, Clone)]
        pub struct Animal {
            pub name: String,
            pub species: String,
            pub age: u8,
            pub fed: bool,
        }
        
        impl Animal {
            pub fn new(name: &str, species: &str, age: u8) -> Self {
                Self {
                    name: name.to_string(),
                    species: species.to_string(),
                    age,
                    fed: false,
                }
            }
            
            pub fn feed(&mut self) {
                self.fed = true;
                println!("🐕 喂食动物: {} ({})", self.name, self.species);
            }
            
            pub fn get_info(&self) -> String {
                format!("{} 是 {}，年龄 {} 岁，{}已喂食",
                        self.name, self.species, self.age,
                        if self.fed { "" } else { "尚未" })
            }
        }
    }
    
    /// 全局作物数据（类似const泛型）
    pub static CROPS_DATA: &[&'static str] = &["wheat", "corn", "soybean"];
}

/// 现代化use关键字使用示例
pub fn modern_use_patterns() {
    println!("📦 现代化use模式：");
    
    // 使用use引入特定项目
    use farm::crops::vegetables::{plant_tomato, plant_carrot};
    use farm::crops::fruits::{plant_apple, plant_strawberry};
    use farm::crops::CropManager;
    use farm::animals::Animal;
    
    // 使用as重命名避免冲突
    use farm::crops::fruits as fruit_crops;
    use farm::crops::vegetables as veg_crops;
    
    plant_tomato();
    plant_carrot();
    plant_apple();
    plant_strawberry();
    
    // 演示现代化作物管理
    let mut manager = CropManager::new();
    veg_crops::manage_vegetables(&mut manager);
    
    let fruit_seasons = fruit_crops::get_fruit_seasons();
    println!("🍎 水果季节: {:?}", fruit_seasons);
    
    println!("🌾 作物状态: {}", manager.get_status());
    
    // 演示现代化动物管理
    let mut dog = Animal::new("Buddy", "金毛寻回犬", 3);
    dog.feed();
    println!("🐕 {}", dog.get_info());
    
    // 演示绝对路径和相对路径
    crate::modules::farm::crops::vegetables::plant_tomato(); // 绝对路径
    farm::crops::fruits::plant_apple(); // 相对路径
}

/// 现代化重新导出和模块访问控制
pub mod modern_exports {
    /// 公开API接口
    pub mod api {
        /// 现代化农场管理系统
        pub struct FarmAPI;
        
        impl FarmAPI {
            pub fn new() -> Self {
                println!("🏭 初始化现代化农场API");
                Self
            }
            
            /// 健康检查
            pub fn health_check() -> bool {
                println!("🔍 执行系统健康检查");
                true // 假设系统正常
            }
        }
    }
    
    /// 重新导出模块
    pub use api::FarmAPI;
    
    /// 模块级常量
    pub const MAX_CROPS: usize = 1000;
    pub const MIN_ANIMAL_AGE: u8 = 1;
}

/// 现代化包和Crate概念演示
pub fn modern_packages_and_crates() {
    println!("📦 现代化包和Crate概念：");

    // 这个函数在rust_learn crate的modules模块中
    println!("📚 当前在 rust_learn crate 的 modules 模块中");
    println!("🧭 示例仅展示模块路径与导出关系，不串联执行其他主题示例");
    println!("   - crate::basics::run_basics_examples()");
    println!("   - crate::ownership::run_ownership_examples()");

    // 演示现代API使用
    let _api = modern_exports::FarmAPI::new();
    let is_healthy = modern_exports::FarmAPI::health_check();

    println!("🔍 系统健康: {}", if is_healthy { "正常" } else { "异常" });

    // 使用模块级常量
    println!("📊 最大作物数量: {}", modern_exports::MAX_CROPS);
    println!("🐕 最小动物年龄: {} 岁", modern_exports::MIN_ANIMAL_AGE);
}

/// 现代化条件编译
pub fn advanced_logging() {
    println!("📝 使用基础日志记录");
}

#[cfg(target_os = "windows")]
pub fn platform_specific() {
    println!("🪟 运行在 Windows 平台");
    #[cfg(debug_assertions)]
    println!("🔧 调试版本：启用开发特性");
}

#[cfg(target_os = "linux")]
pub fn platform_specific() {
    println!("🐧 运行在 Linux 平台");
    #[cfg(feature = "systemd")]
    println!("🔧 集成 systemd 支持");
}

#[cfg(target_os = "macos")]
pub fn platform_specific() {
    println!("🍎 运行在 macOS 平台");
    #[cfg(feature = "cocoa")]
    println!("🔧 集成 Cocoa 支持");
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
pub fn platform_specific() {
    println!("🌐 运行在其他平台");
}

/// 现代化模块使用策略
pub fn modular_design_patterns() {
    println!("🎯 现代化模块设计模式：");
    
    use farm::crops::CropManager;
    use farm::animals::Animal;
    
    // 1. 组合模式 - 将CropManager和Animal结合
    #[derive(Debug)]
    pub struct Farm {
        crop_manager: CropManager,
        animals: Vec<Animal>,
        name: String,
    }
    
    impl Farm {
        pub fn new(name: &str) -> Self {
            println!("🏡 创建农场: {}", name);
            Self {
                crop_manager: CropManager::new(),
                animals: Vec::new(),
                name: name.to_string(),
            }
        }
        
        pub fn add_animal(&mut self, animal: Animal) {
            self.animals.push(animal);
            println!("➕ 添加动物: {}", self.animals.last().unwrap().name);
        }
        
        pub fn farm_status(&self) -> String {
            format!("农场 '{}' - 作物: {}, 动物数量: {}",
                    self.name,
                    self.crop_manager.get_status(),
                    self.animals.len())
        }
    }
    
    // 使用组合模式
    let mut farm = Farm::new("现代化家庭农场");
    
    // 添加作物
    farm.crop_manager.plant("小麦", 100);
    farm.crop_manager.plant("玉米", 80);
    
    // 添加动物
    farm.add_animal(Animal::new("Max", "牧羊犬", 5));
    farm.add_animal(Animal::new("Bella", "奶牛", 3));
    
    println!("📊 {}", farm.farm_status());
    
    // 2. 使用trait进行松散耦合
    trait FarmOperations {
        fn operate(&self) -> String;
    }
    
    impl FarmOperations for CropManager {
        fn operate(&self) -> String {
            format!("作物管理系统运行中: {}", self.get_status())
        }
    }
    
    impl FarmOperations for Vec<Animal> {
        fn operate(&self) -> String {
            format!("动物管理系统运行中，有{}只动物", self.len())
        }
    }
    
    println!("🔧 {}", farm.crop_manager.operate());
    println!("🔧 {}", farm.animals.operate());
}

/// 演示企业级项目组织结构
pub fn enterprise_project_structure() {
    println!("🏢 企业级项目组织结构：");
    
    // 模拟电商平台项目结构
    pub mod ecommerce {
        use std::collections::HashMap;
        use chrono::Utc;
        
        /// 用户实体
        #[derive(Debug, Clone)]
        pub struct User {
            pub id: u64,
            pub username: String,
            pub email: String,
            pub created_at: chrono::DateTime<chrono::Utc>,
        }
        
        /// 产品实体
        #[derive(Debug, Clone)]
        pub struct Product {
            pub id: u64,
            pub name: String,
            pub price: f64,
            pub category: String,
            pub stock: u32,
        }
        
        /// 用户仓库
        pub trait UserRepository {
            fn find_by_id(&self, id: u64) -> Option<User>;
            fn find_by_email(&self, email: &str) -> Option<User>;
            fn save(&mut self, user: User) -> Result<User, String>;
        }
        
        /// 产品仓库
        pub trait ProductRepository {
            fn find_by_id(&self, id: u64) -> Option<Product>;
            fn save(&mut self, product: Product) -> Result<Product, String>;
        }
        
        /// 内存用户仓库实现
        pub struct InMemoryUserRepository {
            users: HashMap<u64, User>,
            next_id: u64,
        }
        
        impl InMemoryUserRepository {
            pub fn new() -> Self {
                Self {
                    users: HashMap::new(),
                    next_id: 1,
                }
            }
        }
        
        impl UserRepository for InMemoryUserRepository {
            fn find_by_id(&self, id: u64) -> Option<User> {
                self.users.get(&id).cloned()
            }
            
            fn find_by_email(&self, email: &str) -> Option<User> {
                self.users.values().find(|u| u.email == email).cloned()
            }
            
            fn save(&mut self, mut user: User) -> Result<User, String> {
                if user.id == 0 {
                    user.id = self.next_id;
                    self.next_id += 1;
                }
                
                user.created_at = Utc::now();
                self.users.insert(user.id, user.clone());
                Ok(user)
            }
        }
        
        /// 内存产品仓库实现
        pub struct InMemoryProductRepository {
            products: HashMap<u64, Product>,
            next_id: u64,
        }
        
        impl InMemoryProductRepository {
            pub fn new() -> Self {
                Self {
                    products: HashMap::new(),
                    next_id: 1,
                }
            }
        }
        
        impl ProductRepository for InMemoryProductRepository {
            fn find_by_id(&self, id: u64) -> Option<Product> {
                self.products.get(&id).cloned()
            }
            
            fn save(&mut self, mut product: Product) -> Result<Product, String> {
                if product.id == 0 {
                    product.id = self.next_id;
                    self.next_id += 1;
                }
                
                self.products.insert(product.id, product.clone());
                Ok(product)
            }
        }
        
        /// 用户服务
        pub struct UserService<R> {
            repository: R,
        }
        
        impl<R: UserRepository> UserService<R> {
            pub fn new(repository: R) -> Self {
                Self { repository }
            }
            
            pub fn create_user(&mut self, username: String, email: String) -> Result<User, String> {
                if self.repository.find_by_email(&email).is_some() {
                    return Err("邮箱已存在".to_string());
                }
                
                let user = User {
                    id: 0,
                    username,
                    email,
                    created_at: Utc::now(),
                };
                
                self.repository.save(user)
            }
        }
        
        /// 产品服务
        pub struct ProductService<R> {
            repository: R,
        }
        
        impl<R: ProductRepository> ProductService<R> {
            pub fn new(repository: R) -> Self {
                Self { repository }
            }
            
            pub fn create_product(&mut self, name: String, price: f64, category: String, stock: u32) -> Result<Product, String> {
                if price <= 0.0 {
                    return Err("价格必须大于0".to_string());
                }
                
                let product = Product {
                    id: 0,
                    name,
                    price,
                    category,
                    stock,
                };
                
                self.repository.save(product)
            }
        }
    }
    
    // 演示企业级项目使用
    println!("🏗️ 演示电商平台项目结构:");
    
    // 创建仓库层
    let user_repo = ecommerce::InMemoryUserRepository::new();
    let product_repo = ecommerce::InMemoryProductRepository::new();
    
    // 创建服务层
    let mut user_service = ecommerce::UserService::new(user_repo);
    let mut product_service = ecommerce::ProductService::new(product_repo);
    
    // 模拟用户注册
    match user_service.create_user("张三".to_string(), "zhangsan@example.com".to_string()) {
        Ok(user) => println!("✅ 用户注册成功: {} (ID: {})", user.username, user.id),
        Err(e) => println!("❌ 用户注册失败: {}", e),
    }
    
    // 模拟产品创建
    match product_service.create_product(
        "iPhone 15".to_string(),
        8999.0,
        "手机".to_string(),
        50,
    ) {
        Ok(product) => println!("✅ 产品创建成功: {} (ID: {})", product.name, product.id),
        Err(e) => println!("❌ 产品创建失败: {}", e),
    }
    
    println!("📊 企业级项目结构演示完成");
}

/// 微服务架构内部模块
pub mod microservices_internal {
    /// 用户服务
    pub mod user_service {
        #[derive(Debug, Clone)]
        pub struct User {
            pub id: u64,
            pub username: String,
            pub email: String,
        }
        
        pub struct UserService {
            users: std::collections::HashMap<u64, User>,
            next_id: u64,
        }
        
        impl UserService {
            pub fn new() -> Self {
                Self {
                    users: std::collections::HashMap::new(),
                    next_id: 1,
                }
            }
            
            pub fn create_user(&mut self, username: String, email: String) -> User {
                let id = self.next_id;
                self.next_id += 1;
                let user = User { id, username, email };
                self.users.insert(id, user.clone());
                user
            }
            
            pub fn get_user(&self, id: u64) -> Option<&User> {
                self.users.get(&id)
            }
        }
    }
    
    /// 产品服务
    pub mod product_service {
        #[derive(Debug, Clone)]
        pub struct Product {
            pub id: u64,
            pub name: String,
            pub price: f64,
        }
        
        pub struct ProductService {
            products: std::collections::HashMap<u64, Product>,
            next_id: u64,
        }
        
        impl ProductService {
            pub fn new() -> Self {
                Self {
                    products: std::collections::HashMap::new(),
                    next_id: 1,
                }
            }
            
            pub fn create_product(&mut self, name: String, price: f64) -> Product {
                let id = self.next_id;
                self.next_id += 1;
                let product = Product { id, name, price };
                self.products.insert(id, product.clone());
                product
            }
            
            pub fn get_product(&self, id: u64) -> Option<&Product> {
                self.products.get(&id)
            }
        }
    }
    
    /// API网关
    pub mod api_gateway {
        use super::*;
        
        pub struct ApiGateway {
            user_service: user_service::UserService,
            product_service: product_service::ProductService,
        }
        
        impl ApiGateway {
            pub fn new() -> Self {
                Self {
                    user_service: user_service::UserService::new(),
                    product_service: product_service::ProductService::new(),
                }
            }
            
            /// 获取用户完整信息
            pub fn get_user_profile(&self, user_id: u64) -> Option<String> {
                if let Some(user) = self.user_service.get_user(user_id) {
                    Some(format!("用户: {} ({})", user.username, user.email))
                } else {
                    None
                }
            }
        }
    }
}

/// 演示微服务架构项目组织
pub fn microservices_architecture() {
    println!("🏗️ 微服务架构项目组织：");

    use microservices_internal::*;

    // 演示微服务架构
    println!("🔧 演示微服务架构:");

    let gateway = api_gateway::ApiGateway::new();

    // 当前 API 网关未暴露写入流程，因此这里只演示“查询空结果”的行为
    println!("ℹ️ 当前示例仅包含查询接口，尚未接入创建用户流程");

    match gateway.get_user_profile(1) {
        Some(profile) => println!("👤 {}", profile),
        None => println!("👤 未找到用户 1（符合当前只读演示行为）"),
    }

    println!("📊 微服务架构演示完成");
}

/// 演示包和特性管理
pub fn package_features_management() {
    println!("📦 包和特性管理：");
    
    // 特性编译示例
    println!("🔧 特性编译示例:");
    
    #[allow(dead_code)]
    fn advanced_logging_demo() {
        println!("📝 基础日志功能");
        println!("[INFO] 程序启动");
        println!("[INFO] 程序就绪");
    }
    
    advanced_logging_demo();
    
    // 平台特定功能
    #[cfg(target_os = "windows")]
    fn platform_specific_demo() {
        println!("🪟 Windows 平台特定功能:");
        println!("   - Windows API 访问");
    }
    
    #[cfg(target_os = "linux")]
    fn platform_specific_demo() {
        println!("🐧 Linux 平台特定功能:");
        println!("   - Unix 系统调用");
    }
    
    #[cfg(target_os = "macos")]
    fn platform_specific_demo() {
        println!("🍎 macOS 平台特定功能:");
        println!("   - Cocoa 框架集成");
    }
    
    platform_specific_demo();
    
    // 开发模式功能
    #[cfg(debug_assertions)]
    fn development_mode_demo() {
        println!("🔧 开发模式功能:");
        println!("   - 详细的错误信息");
    }
    
    #[cfg(not(debug_assertions))]
    fn development_mode_demo() {
        println!("🚀 生产模式功能:");
        println!("   - 优化后的性能");
    }
    
    development_mode_demo();
    
    println!("📊 包和特性管理演示完成");
}

/// 运行模块和包管理示例
pub fn run_modules_examples() {
    println!("🎯 === 现代化模块和包管理示例 ===");
    println!();
    
    modern_use_patterns();
    println!();
    
    modern_packages_and_crates();
    println!();
    
    advanced_logging();
    println!();
    
    platform_specific();
    println!();
    
    modular_design_patterns();
    
    println!("\n✅ 所有模块和包管理示例运行完成！");
}

/// 运行所有模块示例
pub fn run_all_module_examples() {
    println!("🎯 === 全面模块和包管理示例 ===");
    println!();
    
    println!("=== 基础模块示例 ===");
    run_modules_examples();
    println!();
    
    println!("=== 企业级项目组织 ===");
    enterprise_project_structure();
    println!();
    
    println!("=== 微服务架构 ===");
    microservices_architecture();
    println!();
    
    println!("=== 包和特性管理 ===");
    package_features_management();
    
    println!("\n✅ 所有模块和包管理示例运行完成！");
}
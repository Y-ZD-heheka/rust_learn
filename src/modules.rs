//! # 模块和包管理模块
//!
//! 这个模块演示了Rust的模块系统和包管理。
//! 采用了现代化的Rust 2021/2024最佳实践。

/// 现代化模块定义 - 演示农场管理系统
pub mod farm {
    use std::collections::HashMap;
    
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
            use std::collections::HashMap;
            
            pub fn plant_apple() {
                println!("🍎 种植苹果");
            }
            
            pub fn plant_strawberry() {
                println!("🍓 种植草莓");
            }
            
            pub fn get_fruit_seasons() -> HashMap<String, &'static str> {
                let mut seasons = HashMap::new();
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
    use std::collections::HashMap;
    
    /// 公开API接口
    pub mod api {
        use super::*;
        
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
    println!("📚 当前在rust_learn crate的modules模块中");
    
    // 调用其他模块的函数 - 使用现代化路径
    use crate::basics::run_basics_examples;
    use crate::ownership::run_ownership_examples;
    
    run_basics_examples();
    run_ownership_examples();
    
    // 演示现代API使用
    let api = modern_exports::FarmAPI::new();
    let is_healthy = modern_exports::FarmAPI::health_check();
    
    println!("🔍 系统健康: {}", if is_healthy { "正常" } else { "异常" });
    
    // 使用模块级常量
    println!("📊 最大作物数量: {}", modern_exports::MAX_CROPS);
    println!("🐕 最小动物年龄: {} 岁", modern_exports::MIN_ANIMAL_AGE);
}

/// 现代化条件编译
#[cfg(feature = "advanced_logging")]
pub fn advanced_logging() {
    println!("📝 启用高级日志记录功能");
}

#[cfg(not(feature = "advanced_logging"))]
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
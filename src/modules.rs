//! # 模块和包管理模块
//!
//! 这个模块演示了Rust的模块系统和包管理。

/// 演示模块的定义和使用
pub mod garden {
    pub mod vegetables {
        pub fn plant() {
            println!("Planting vegetables");
        }

        pub mod tomato {
            pub fn plant() {
                println!("Planting tomato");
            }
        }
    }

    pub mod flowers {
        pub fn plant() {
            println!("Planting flowers");
        }
    }
}

/// 使用use关键字引入模块
pub fn using_modules() {
    // 绝对路径
    crate::modules::garden::vegetables::plant();

    // 相对路径
    garden::vegetables::plant();

    // 使用use引入
    use garden::vegetables::tomato;
    tomato::plant();

    // 使用as重命名
    use garden::flowers as fl;
    fl::plant();
}

/// 演示pub use重新导出
pub mod outer_module {
    pub mod inner_module {
        pub fn inner_function() {
            println!("inner function");
        }
    }

    // 重新导出
    pub use self::inner_module::inner_function as public_function;
}

/// 演示包和crate的概念
pub fn packages_and_crates() {
    // 这个函数在rust_learn crate的modules模块中
    println!("This is in the modules module of rust_learn crate");

    // 调用其他模块的函数
    crate::basics::run_basics_examples();
}

/// 演示条件编译
#[cfg(target_os = "windows")]
pub fn platform_specific() {
    println!("Running on Windows");
}

#[cfg(target_os = "linux")]
pub fn platform_specific() {
    println!("Running on Linux");
}

#[cfg(not(any(target_os = "windows", target_os = "linux")))]
pub fn platform_specific() {
    println!("Running on other OS");
}

/// 运行模块和包管理示例
pub fn run_modules_examples() {
    println!("=== 模块和包管理示例 ===");
    using_modules();
    packages_and_crates();
    platform_specific();
}
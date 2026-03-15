use colored::Colorize;

use super::{Priority, Task, TaskManager};

/// 运行任务管理器演示
pub fn run_task_manager_demo() -> anyhow::Result<()> {
    println!("{}", "🎯 Task Manager Demo".bold().green());
    println!("{}", "═══════════════════════════════════════".dimmed());

    let mut manager = TaskManager::new()?;

    // 添加示例任务
    println!("\n{}", "Adding sample tasks...".cyan());

    let task1 = Task::new(0, "Complete Rust project", Priority::High)
        .with_description("Finish the task manager implementation")
        .with_tags(vec!["rust".to_string(), "project".to_string()]);
    let id1 = manager.add_task(task1)?;
    println!("  Added task #{}: Complete Rust project", id1);

    let task2 = Task::new(0, "Review pull requests", Priority::Medium)
        .with_tags(vec!["code-review".to_string()]);
    let id2 = manager.add_task(task2)?;
    println!("  Added task #{}: Review pull requests", id2);

    let task3 = Task::new(0, "Fix critical bug in production", Priority::Urgent)
        .with_description("Users are reporting crashes")
        .with_tags(vec!["bug".to_string(), "production".to_string()]);
    let id3 = manager.add_task(task3)?;
    println!("  Added task #{}: Fix critical bug in production", id3);

    let task4 =
        Task::new(0, "Update documentation", Priority::Low).with_tags(vec!["docs".to_string()]);
    let id4 = manager.add_task(task4)?;
    println!("  Added task #{}: Update documentation", id4);

    // 完成任务
    println!("\n{}", "Completing a task...".cyan());
    manager.complete_task(id2)?;
    println!("  ✅ Completed task #{}: Review pull requests", id2);

    // 列出所有任务
    println!("\n{}", "All Tasks:".bold().underline());
    for task in manager.list_tasks(None) {
        println!("  {}", task.display());
    }

    // 显示统计
    manager.get_statistics().display();

    // 搜索任务
    println!("\n{}", "Search results for 'rust':".bold().underline());
    for task in manager.search_tasks("rust") {
        println!("  {}", task.display());
    }

    // 清理
    println!("\n{}", "Cleaning up demo tasks...".cyan());
    let _ = manager.delete_task(id1);
    let _ = manager.delete_task(id2);
    let _ = manager.delete_task(id3);
    let _ = manager.delete_task(id4);
    println!("  ✅ Demo completed!");

    Ok(())
}

use colored::Colorize;

/// 任务统计信息
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskStatistics {
    pub total: usize,
    pub completed: usize,
    pub pending: usize,
    pub in_progress: usize,
    pub urgent: usize,
}

impl TaskStatistics {
    /// 显示统计信息
    pub fn display(&self) {
        println!("\n{}", "📊 Task Statistics".bold().underline());
        println!("  {} {}", "Total:".bold(), self.total);
        println!(
            "  {} {} ({:.1}%)",
            "Completed:".green().bold(),
            self.completed,
            if self.total > 0 {
                (self.completed as f64 / self.total as f64) * 100.0
            } else {
                0.0
            }
        );
        println!("  {} {}", "Pending:".yellow().bold(), self.pending);
        println!("  {} {}", "In Progress:".blue().bold(), self.in_progress);

        if self.urgent > 0 {
            println!("  {} {}", "⚠️  Urgent:".red().bold(), self.urgent);
        }
    }
}

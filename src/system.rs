use axum::Json;
use serde::Serialize;
use sysinfo::{System};

#[derive(Serialize)]
pub struct Status {
    cpu_usage: f32,
    memory_total: u64,
    memory_used: u64,
    memory_usage_percent: f32,
    uptime: u64,
}

pub async fn get_status() -> Json<Status> {
    Json(collect_status())
}

pub fn collect_status() -> Status {
    let mut sys = System::new_all();

    sys.refresh_cpu_all();
    std::thread::sleep(std::time::Duration::from_millis(500));
    sys.refresh_cpu_all();

    sys.refresh_memory();

    let cpu_usage = sys
        .cpus()
        .iter()
        .map(|c| c.cpu_usage())
        .sum::<f32>() / sys.cpus().len() as f32;

    let memory_total = sys.total_memory();
    let memory_used = sys.used_memory();
    let memory_usage_percent = (memory_used as f32 / memory_total as f32) * 100.0;

    Status {
        cpu_usage,
        memory_total,
        memory_used,
        memory_usage_percent,
        uptime: sysinfo::System::uptime(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_fields_are_valid() {
        let status = collect_status();
        assert!(status.cpu_usage >= 0.0);
        assert!(status.memory_total > 0);
        assert!(status.memory_used <= status.memory_total);
        assert!(status.memory_usage_percent <= 100.0);
    }
}
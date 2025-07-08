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
    let mut sys = System::new_all();

    // Первый вызов — инициализация
    sys.refresh_cpu_all();

    // Дать немного времени для сбора статистики
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    // Второй вызов — обновление значений
    sys.refresh_cpu_all();

    let cpu_usage = sys
        .cpus()
        .iter()
        .map(|c| c.cpu_usage())
        .sum::<f32>() / sys.cpus().len() as f32;

    sys.refresh_memory();

    let memory_total = sys.total_memory();
    let memory_used = sys.used_memory();

    let memory_usage_percent = (memory_used as f32 / memory_total as f32) * 100.0;

    Json(Status {
        cpu_usage,
        memory_total,
        memory_used,
        memory_usage_percent,
        uptime: sysinfo::System::uptime(),
    })
}
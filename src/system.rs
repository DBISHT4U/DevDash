use sysinfo::System;

pub struct SystemStats {
    pub hostname: String,
    pub os_name: String,
    pub kernel: String,
    pub uptime_secs: u64,
    pub cpu_name: String,
    pub cpu_count: usize,
    pub cpu_usage: f32,
    pub cpu_history: Vec<f64>,
    pub mem_total_gb: f64,
    pub mem_used_gb: f64,
    pub mem_percent: f64,
    pub mem_history: Vec<f64>,
    pub swap_total_gb: f64,
    pub swap_used_gb: f64,
    pub disks: Vec<DiskInfo>,
    pub net_rx_bytes: u64,
    pub net_tx_bytes: u64,
    pub processes: usize,
    pub top_processes: Vec<ProcessInfo>,
    pub battery_percent: Option<f32>,
    pub battery_charging: Option<bool>,
}

pub struct DiskInfo {
    pub name: String,
    pub mount: String,
    pub total_gb: f64,
    pub used_gb: f64,
    pub percent: f64,
}

pub struct ProcessInfo {
    pub name: String,
    pub cpu: f32,
    pub mem_mb: f64,
    pub pid: u32,
}

impl SystemStats {
    pub fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();
        std::thread::sleep(std::time::Duration::from_millis(200));
        sys.refresh_all();

        let hostname = System::host_name().unwrap_or_else(|| "unknown".to_string());
        let os_name = format!(
            "{} {}",
            System::name().unwrap_or_default(),
            System::os_version().unwrap_or_default()
        );
        let kernel = System::kernel_version().unwrap_or_else(|| "unknown".to_string());
        let uptime_secs = System::uptime();

        // CPU
        let cpu_usage = sys.global_cpu_usage();
        let cpu_name = sys
            .cpus()
            .first()
            .map(|c| c.brand().to_string())
            .unwrap_or_else(|| "Unknown CPU".to_string());
        let cpu_count = sys.cpus().len();

        // Memory
        let mem_total_gb = sys.total_memory() as f64 / 1_073_741_824.0;
        let mem_used_gb = sys.used_memory() as f64 / 1_073_741_824.0;
        let mem_percent = if mem_total_gb > 0.0 {
            (mem_used_gb / mem_total_gb) * 100.0
        } else {
            0.0
        };

        let swap_total_gb = sys.total_swap() as f64 / 1_073_741_824.0;
        let swap_used_gb = sys.used_swap() as f64 / 1_073_741_824.0;

        // Disks
        let disks: Vec<DiskInfo> = sysinfo::Disks::new_with_refreshed_list()
            .iter()
            .map(|d| {
                let total = d.total_space() as f64 / 1_073_741_824.0;
                let available = d.available_space() as f64 / 1_073_741_824.0;
                let used = total - available;
                DiskInfo {
                    name: d.name().to_string_lossy().to_string(),
                    mount: d.mount_point().to_string_lossy().to_string(),
                    total_gb: total,
                    used_gb: used,
                    percent: if total > 0.0 {
                        (used / total) * 100.0
                    } else {
                        0.0
                    },
                }
            })
            .collect();

        // Network
        let networks = sysinfo::Networks::new_with_refreshed_list();
        let mut net_rx: u64 = 0;
        let mut net_tx: u64 = 0;
        for (_name, data) in &networks {
            net_rx += data.received();
            net_tx += data.transmitted();
        }

        // Processes
        let processes = sys.processes().len();
        let mut top_procs: Vec<ProcessInfo> = sys
            .processes()
            .values()
            .map(|p| ProcessInfo {
                name: p.name().to_string_lossy().to_string(),
                cpu: p.cpu_usage(),
                mem_mb: p.memory() as f64 / 1_048_576.0,
                pid: p.pid().as_u32(),
            })
            .collect();
        top_procs.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap_or(std::cmp::Ordering::Equal));
        top_procs.truncate(8);

        // Battery
        let (battery_percent, battery_charging) = get_battery_info();

        SystemStats {
            hostname,
            os_name,
            kernel,
            uptime_secs,
            cpu_name,
            cpu_count,
            cpu_usage,
            cpu_history: vec![cpu_usage as f64],
            mem_total_gb,
            mem_used_gb,
            mem_percent,
            mem_history: vec![mem_percent],
            swap_total_gb,
            swap_used_gb,
            disks,
            net_rx_bytes: net_rx,
            net_tx_bytes: net_tx,
            processes,
            top_processes: top_procs,
            battery_percent,
            battery_charging,
        }
    }

    pub fn refresh(&mut self) {
        let fresh = Self::collect();
        self.cpu_usage = fresh.cpu_usage;
        self.mem_used_gb = fresh.mem_used_gb;
        self.mem_percent = fresh.mem_percent;
        self.swap_used_gb = fresh.swap_used_gb;
        self.disks = fresh.disks;
        self.net_rx_bytes = fresh.net_rx_bytes;
        self.net_tx_bytes = fresh.net_tx_bytes;
        self.processes = fresh.processes;
        self.top_processes = fresh.top_processes;
        self.battery_percent = fresh.battery_percent;
        self.battery_charging = fresh.battery_charging;

        // Keep history (last 60 ticks)
        self.cpu_history.push(self.cpu_usage as f64);
        if self.cpu_history.len() > 60 {
            self.cpu_history.remove(0);
        }
        self.mem_history.push(self.mem_percent);
        if self.mem_history.len() > 60 {
            self.mem_history.remove(0);
        }
    }
}

fn get_battery_info() -> (Option<f32>, Option<bool>) {
    match battery::Manager::new() {
        Ok(manager) => match manager.batteries() {
            Ok(mut batteries) => {
                if let Some(Ok(bat)) = batteries.next() {
                    let pct = bat.state_of_charge().get::<battery::units::ratio::percent>();
                    let charging = bat.state() == battery::State::Charging;
                    (Some(pct), Some(charging))
                } else {
                    (None, None)
                }
            }
            Err(_) => (None, None),
        },
        Err(_) => (None, None),
    }
}

pub fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let mins = (secs % 3600) / 60;
    if days > 0 {
        format!("{}d {}h {}m", days, hours, mins)
    } else if hours > 0 {
        format!("{}h {}m", hours, mins)
    } else {
        format!("{}m", mins)
    }
}

pub fn format_bytes(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1} GB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.1} MB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KB", bytes as f64 / 1024.0)
    } else {
        format!("{} B", bytes)
    }
}

use std::sync::{Arc, OnceLock};

use wayle_battery::BatteryService;
use wayle_sysinfo::SysinfoService;

static BATTERY_SERVICE: OnceLock<Arc<BatteryService>> = OnceLock::new();
pub fn battery_service() -> Arc<BatteryService> {
    BATTERY_SERVICE
        .get()
        .expect("BatteryService not initialised")
        .clone()
}

static SYSINFO_SERVICE: OnceLock<Arc<SysinfoService>> = OnceLock::new();
pub fn sysinfo_service() -> Arc<SysinfoService> {
    SYSINFO_SERVICE
        .get()
        .expect("BatteryService not initialised")
        .clone()
}

pub async fn init_services() -> anyhow::Result<()> {
    BATTERY_SERVICE
        .set(Arc::new(BatteryService::new().await?))
        .ok();
    SYSINFO_SERVICE
        .set(Arc::new(SysinfoService::builder().build()))
        .ok();

    Ok(())
}

use std::sync::{Arc, OnceLock};

use wayle_battery::BatteryService;

static BATTERY_SERVICE: OnceLock<Arc<BatteryService>> = OnceLock::new();
pub fn battery_service() -> Arc<BatteryService> {
    BATTERY_SERVICE
        .get()
        .expect("BatteryService not initialised")
        .clone()
}

pub async fn init_services() -> anyhow::Result<()> {
    BATTERY_SERVICE
        .set(Arc::new(BatteryService::new().await?))
        .ok();

    Ok(())
}

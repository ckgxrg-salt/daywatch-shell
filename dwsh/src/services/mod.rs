use std::sync::{Arc, OnceLock};
use tokio::sync::OnceCell;

use wayle_battery::BatteryService;
use wayle_sysinfo::SysinfoService;
use wayle_systray::SystemTrayService;

static BATTERY_SERVICE: OnceCell<Arc<BatteryService>> = OnceCell::const_new();
pub async fn battery_service() -> Arc<BatteryService> {
    BATTERY_SERVICE
        .get_or_init(|| async { Arc::new(BatteryService::new().await.unwrap()) })
        .await
        .clone()
}

static SYSINFO_SERVICE: OnceLock<Arc<SysinfoService>> = OnceLock::new();
pub fn sysinfo_service() -> Arc<SysinfoService> {
    SYSINFO_SERVICE
        .get_or_init(|| Arc::new(SysinfoService::builder().build()))
        .clone()
}

static TRAY_SERVICE: OnceCell<Arc<SystemTrayService>> = OnceCell::const_new();
pub async fn tray_service() -> Arc<SystemTrayService> {
    TRAY_SERVICE
        .get_or_init(|| async { SystemTrayService::new().await.unwrap() })
        .await
        .clone()
}

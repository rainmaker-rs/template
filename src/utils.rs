{% if template -%}
{% if template_wifi -%}
use rainmaker::components::{
    persistent_storage::NvsPartition,
    protocomm::ProtocommSecurity,
    wifi::{WifiClientConfig, WifiMgr},
    wifi_prov::{WiFiProvMgrBle, WifiProvBleConfig},
};
use rainmaker::Rainmaker;
use std::sync::{Arc, Mutex};

{% endif -%}

{%- if template_logger -%}
pub fn initializse_logger() {
    {% if linux -%}
    {% if esp -%}
    #[cfg(target_os = "linux")]
    {% endif -%}
    simple_logger::init_with_level(log::Level::Info).unwrap();

    {%- endif %}

    {%- if esp -%}
    {% if linux %}

    #[cfg(target_os = "espidf")]
    {% endif -%}
    esp_idf_svc::log::EspLogger::initialize_default();

    {%- endif %}
}

{% endif -%}

{%- if template_wifi -%}
pub fn connect_wifi(
    rmaker: &Rainmaker,
    name: &str
) -> anyhow::Result<()> {
    let nvs_partition = NvsPartition::new("nvs")?;
    wifi_arc_mutex = Arc::new(Mutex::new(WifiMgr::new()?));

    let prov_config = WifiProvBleConfig {
        service_name: format!("PROV_{}", name),
        ..Default::default()
    };
    let mut prov_mgr = WiFiProvMgrBle::new(
        wifi_arc_mutex.clone(),
        prov_config,
        nvs_partition,
        ProtocommSecurity::new_sec1(None),
    )?;
    if let Some((ssid, password)) = prov_mgr.is_provisioned() {
        log::info!("Node already provisioned. Trying to connect");
        let mut wifi = wifi_arc_mutex.lock().unwrap();
        let config = WifiClientConfig {
            ssid,
            password,
            ..Default::default()
        };
        wifi.set_client_config(config)?;
        wifi.start()?;
        wifi.assured_connect();
        drop(prov_mgr);
    } else {
        log::info!("Node not provisioned. Starting WiFi provisioning.");
        rmaker.reg_user_mapping_ep(&mut prov_mgr);
        prov_mgr.start()?;
        prov_mgr.wait_for_provisioning();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::new(30, 0));
            log::info!("Stopping WiFi provisioning");
            drop(prov_mgr);
        });
    }

    // WiFi disconnects if dropped
    std::mem::forget(wifi_arc_mutex);

    Ok(())
}
{% endif -%}
{% endif -%}

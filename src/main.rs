{% if template -%}
mod utils;

use rainmaker::{node::{self, Node}, Rainmaker};
{% endif -%}

{% if template -%}
fn main() -> anyhow::Result<()> {
{% else -%}
fn main() {
{% endif -%}
    {% if esp -%}
    // It is necessary to call this function once while running on ESP platform. 
    // Otherwise some patches to the runtime implemented by esp-idf-sys might not link properly.
    // See https://github.com/esp-rs/esp-idf-template/issues/71
    {% if linux -%}
    #[cfg(target_os = "espidf")]
    {% endif -%}
    esp_idf_svc::sys::link_patches();

    {% endif -%}

    {% if template -%}
    {% if template_logger -%}
    utils::initializse_logger();
    
    {% endif -%}

    let rmaker = Rainmaker::init()?;

    let mut node = Node::new(rmaker.get_node_id());
    node.set_info(node::Info{
        name: String::from("{{ node_name }}"),
        fw_version: String::from("1.0"),
    });

    /* Add custom devices here */

    {% if template_wifi -%}
    utils::connect_wifi(rmaker, "{{ prov_name }}")?;
    {% endif -%}
    rmaker.register_node(node);

    log::info!("Starting RainMaker Agent...");
    rmaker.start()?;

    Ok(())

    {% else -%}
    println!("Hello, World");
    {%- endif %}
}

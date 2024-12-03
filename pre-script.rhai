let esp_targets = #{
    esp32: #{
        esp_arch: "xtensa",
        esp_target: "xtensa-esp32-espidf",
    },
    esp32c2: #{
        esp_arch: "riscv",
        esp_target: "riscv32imc-esp-espidf",
    },
    esp32c3: #{
        esp_arch: "riscv",
        esp_target: "riscv32imc-esp-espidf",
    },
    esp32c6: #{
        esp_arch: "riscv",
        esp_target: "riscv32imac-esp-espidf",
    },
    esp32h2: #{
        esp_arch: "riscv",
        esp_target: "riscv32imac-esp-espidf",
    },
    esp32s2: #{
        esp_arch: "xtensa",
        esp_target: "xtensa-esp32s2-espidf",
    },
    esp32s3: #{
        esp_arch: "xtensa",
        esp_target: "xtensa-esp32s3-espidf",
    },
};

if variable::is_set("mcu"){
  let esp_target = variable::get("mcu");
  let esp_target_properties = esp_targets.get(esp_target);
  for key in esp_target_properties.keys() {
      variable::set(key, esp_target_properties.get(key));
  }
}

if variable::is_set("linux"){
  variable::set("linux_target", "x86_64-unknown-linux-gnu")
}

if variable::get("template"){
  if !variable::get("template_advanced") {
    variable::set("template_logger", true);
    variable::set("template_wifi", true);
  }
}

variable::set("node_name", to_title_case(variable::get("project-name")));
variable::set("prov_name", to_shouty_snake_case(variable::get("project-name")));

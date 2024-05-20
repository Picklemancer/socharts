use plist::Value;

pub type GetValue = fn(&Value, f32) -> f64;

fn get_property_value(mut result: &Value, path: Vec<&str>) -> f64 {
    for key in path.iter() {
        match result {
            Value::Dictionary(value) => {
                result = value.get(key).expect(&format!("{} key missing", key));
                continue;
            }
            Value::Array(value) => {
                if let Ok(index) = key.parse::<usize>() {
                    result = value.get(index).expect(&format!("{} index missing", index));
                }

                // if key.contains("=") {
                //     let splitted = key.split("=").collect();
                //     result = value
                //         .iter()
                //         .find(|x| x.get(splitted[0]) == splitted[1])
                //         .expect("x filter missing");
                // }

                // result = value
                //     .get(key.parse::<usize>().unwrap())
                //     .expect("x key missing");

                continue;
            }
            _ => panic!("path not correct"),
        }
    }
    match result {
        // Value::Boolean(value) => value,
        // Value::String(value) => value,
        Value::Real(value) => *value,
        Value::Integer(value) => value.as_unsigned().unwrap() as f64,
        _ => panic!("value not number"),
    }
}

pub fn get_property(os_version: &str, key: &str) -> Option<GetValue> {
    match os_version {
        os_version if os_version.starts_with("13.") => match key {
            "core0_freq" => Some(|result, _| {
                get_property_value(
                    result,
                    vec!["processor", "clusters", "0", "cpus", "0", "freq_hz"],
                ) / 1000000.0
            }),
            "core1_freq" => Some(|result, _| {
                get_property_value(
                    result,
                    vec!["processor", "clusters", "0", "cpus", "1", "freq_hz"],
                ) / 1000000.0
            }),
            "cores0_freq" => Some(|result, _| {
                get_property_value(result, vec!["processor", "clusters", "0", "freq_hz"])
                    / 1000000.0
            }),
            "cores1_freq" => Some(|result, _| {
                get_property_value(result, vec!["processor", "clusters", "1", "freq_hz"])
                    / 1000000.0
            }),
            "cores2_freq" => Some(|result, _| {
                get_property_value(result, vec!["processor", "clusters", "2", "freq_hz"])
                    / 1000000.0
            }),
            "gpu_freq" => Some(|result, _| get_property_value(result, vec!["gpu", "freq_hz"])),

            "cores0_usage" => Some(|result, _| {
                (1.0 - get_property_value(result, vec!["processor", "clusters", "0", "idle_ratio"]))
                    * 100.0
            }),
            "cores1_usage" => Some(|result, _| {
                (1.0 - get_property_value(result, vec!["processor", "clusters", "1", "idle_ratio"]))
                    * 100.0
            }),
            "cores2_usage" => Some(|result, _| {
                (1.0 - get_property_value(result, vec!["processor", "clusters", "2", "idle_ratio"]))
                    * 100.0
            }),
            "gpu_usage" => Some(|result, _| {
                (1.0 - get_property_value(result, vec!["gpu", "idle_ratio"])) * 100.0
            }),

            "soc_power" => {
                Some(|result, _| get_property_value(result, vec!["processor", "combined_power"]))
            }
            "cpu_power" => {
                Some(|result, _| get_property_value(result, vec!["processor", "cpu_power"]))
            }
            "gpu_power" => {
                Some(|result, _| get_property_value(result, vec!["processor", "gpu_power"]))
            }
            "ane_power" => {
                Some(|result, _| get_property_value(result, vec!["processor", "ane_power"]))
            }

            _ => None,
        },

        os_version if os_version.starts_with("12.") => match key {
            "gpu_freq" => Some(|result, _| get_property_value(result, vec!["gpu", "freq_hz"])),

            "gpu_usage" => Some(|result, _| {
                (1.0 - get_property_value(result, vec!["gpu", "idle_ratio"])) * 100.0
            }),

            "soc_power" => Some(|result, duration| {
                get_property_value(result, vec!["processor", "package_energy"]) / duration as f64
            }),
            "cpu_power" => Some(|result, duration| {
                get_property_value(result, vec!["processor", "cpu_energy"]) / duration as f64
            }),
            "cores0_power" => Some(|result, _| {
                get_property_value(result, vec!["processor", "clusters", "0", "power"])
            }),
            "cores1_power" => Some(|result, _| {
                get_property_value(result, vec!["processor", "clusters", "1", "power"])
            }),
            "gpu_power" => Some(|result, duration| {
                get_property_value(result, vec!["processor", "gpu_energy"]) / duration as f64
            }),
            "dram_power" => Some(|result, duration| {
                get_property_value(result, vec!["processor", "dram_energy"]) / duration as f64
            }),
            "ane_power" => Some(|result, duration| {
                get_property_value(result, vec!["processor", "ane_energy"]) / duration as f64
            }),
            _ => None,
        },
        _ => None,
    }
}

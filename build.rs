use std::{fs, path::Path};
use std::collections::BTreeMap;

fn to_pascal_case(s: &str) -> String {
    let mut result = String::new();
    result.reserve(s.len());
    for c in s.chars() {
        if c == '-' {
            result.push('_');
            continue;
        }
        result.push(c);
    }
    result
}

fn main() {
    const OUT_DIR: &str = "./src/data";

    // Parse settings.json as enum
    if let Ok(settings_json) = fs::read_to_string("settings.json") {
        let parsed: serde_json::Value = serde_json::from_str(&settings_json)
            .expect("Failed to parse settings.json");

        let mut enum_def = String::from(
            "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n#[repr(u32)]\npub enum EColors {\n",
        );

        let colors = parsed["colors"]
            .as_object()
            .expect("Expected 'colors' to be an object");

        let sorted_colors: BTreeMap<_, _> = colors.iter().collect();

        for (name, value) in sorted_colors {
            let variant = to_pascal_case(name);
            let hex_str = value.as_str().unwrap().trim_start_matches('#');
            let int_value = u32::from_str_radix(hex_str, 16).expect("Invalid hex");

            enum_def.push_str(&format!("    {} = 0x{:06X},\n", variant, int_value));
        }

        enum_def.push_str("}\n");


        let dest_path = Path::new(&OUT_DIR).join("settings.rs");

        fs::write(dest_path, enum_def).expect("Failed to write output");
        println!("cargo:rerun-if-changed=data.json");
    }

    // Parse emojis.json as enum
    if let Ok(emojis_json) = fs::read_to_string("data/emojis.json") {
        let parsed: serde_json::Value = serde_json::from_str(&emojis_json)
            .expect("Failed to parse emoji.json");

        let mut enum_def = String::from("");

        if let Some(static_emojis) = parsed["static"].as_object() {
            enum_def.push_str(
                "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n#[repr(u64)]\npub enum EStatic {\n",
            );

            let sorted_static: BTreeMap<_, _> = static_emojis.iter().collect();

            for (name, value) in sorted_static {
                let variant = to_pascal_case(name);
                let id_str = value.as_str().unwrap();

                enum_def.push_str(&format!("    {} = {},\n", variant, id_str));
            }

            enum_def.push_str("}\n");
        }

        if let Some(static_emojis) = parsed["animated"].as_object() {
            enum_def.push_str(
                "\n#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n#[repr(u64)]\npub enum EAnimated {\n",
            );

            let sorted_static: BTreeMap<_, _> = static_emojis.iter().collect();

            for (name, value) in sorted_static {
                let variant = to_pascal_case(name);
                let id_str = value.as_str().unwrap();

                enum_def.push_str(&format!("    {} = {},\n", variant, id_str));
            }

            enum_def.push_str("}\n");
        }

        let dest_path = Path::new(&OUT_DIR).join("emojis.rs");

        fs::write(dest_path, enum_def).expect("Failed to write output");
        println!("cargo:rerun-if-changed=data.json");
    }

    // Parse guild.json as enum
    if let Ok(guild_json) = fs::read_to_string("data/guild.json") {
        let parsed: serde_json::Value = serde_json::from_str(&guild_json)
            .expect("Failed to parse guild.json");

        let mut enum_def = String::from("");

        if let Some(channels) = parsed["channels"].as_object() {
            enum_def.push_str(
                "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n#[repr(u64)]\npub enum EChannels {\n",
            );

            let sorted_colors: BTreeMap<_, _> = channels.iter().collect();

            for (name, value) in sorted_colors {
                let variant = to_pascal_case(name);
                let id_str = value.as_str().unwrap();

                enum_def.push_str(&format!("    {} = {},\n", variant, id_str));
            }

            enum_def.push_str("}\n");
        }

        if let Some(channels) = parsed["roles"].as_object() {
            enum_def.push_str(
                "#[derive(Debug, Clone, Copy, PartialEq, Eq)]\n#[repr(u64)]\npub enum ERoles {\n",
            );

            let sorted_colors: BTreeMap<_, _> = channels.iter().collect();

            for (name, value) in sorted_colors {
                let variant = to_pascal_case(name);
                let id_str = value.as_str().unwrap();

                enum_def.push_str(&format!("    {} = {},\n", variant, id_str));
            }

            enum_def.push_str("}\n");
        }

        let dest_path = Path::new(&OUT_DIR).join("guild.rs");

        fs::write(dest_path, enum_def).expect("Failed to write output");
        println!("cargo:rerun-if-changed=data.json");
    }

    // Parse fab.json as enum
    if let Ok(guild_json) = fs::read_to_string("data/fab.json") {
        let parsed: serde_json::Value = serde_json::from_str(&guild_json)
            .expect("Failed to parse fab.json");

        let map = parsed.as_object().expect("Expected JSON object");

        let mut enum_def = String::from("#[derive(Debug)]\npub enum EProduct {\n");
        let mut match_body = String::from("impl EProduct {\n    pub fn info(&self) -> ProductInfo {\n        match self {\n");

        // Define struct ProductInfo
        let mut struct_def = String::from(
            "#[derive(Debug)]\npub struct ProductInfo {\n    pub product_name: &'static str,\n    pub product_desc: &'static str,\n    pub thumb_link: &'static str,\n    pub product_url: &'static str,\n    pub doc_url: &'static str,\n}\n\n",
        );

        let mut variants = BTreeMap::new();

        for (key, val) in map {
            let variant = to_pascal_case(key);
            variants.insert(variant.clone(), key.clone());

            let obj = val.as_object().expect("Expected inner object");

            let product_name = obj["product-name"].as_str().unwrap();
            let product_desc = obj["product-desc"].as_str().unwrap();
            let thumb_link = obj["thumb-link"].as_str().unwrap();
            let product_url = obj["product-url"].as_str().unwrap();
            let doc_url = obj["doc-url"].as_str().unwrap();

            enum_def.push_str(&format!("    {},\n", variant));

            match_body.push_str(&format!(
                "            EProduct::{} => ProductInfo {{\n                product_name: {:?},\n                product_desc: {:?},\n                thumb_link: {:?},\n                product_url: {:?},\n                doc_url: {:?},\n            }},\n",
                variant, product_name, product_desc, thumb_link, product_url, doc_url
            ));
        }

        enum_def.push_str("}\n\n");
        match_body.push_str("        }\n    }\n}\n");

        let output = format!("{}\n{}{}", struct_def, enum_def, match_body);

        let dest_path = Path::new(&OUT_DIR).join("fab.rs");

        fs::write(dest_path, output).expect("Failed to write output");
        println!("cargo:rerun-if-changed=data.json");
    }
}

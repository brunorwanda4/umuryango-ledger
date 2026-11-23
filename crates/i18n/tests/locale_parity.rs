use std::collections::HashSet;
use umuryango_i18n::keys::Locale;
use umuryango_i18n::loader::{EN_JSON, RW_JSON};
use umuryango_i18n::{t, t_args};

fn extract_placeholders(template: &str) -> HashSet<String> {
    let mut placeholders = HashSet::new();
    let mut chars = template.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch == '{' {
            let mut name = String::new();
            // Skip optional '$' if any
            if let Some(&'$') = chars.peek() {
                chars.next();
            }
            for inner in chars.by_ref() {
                if inner == '}' {
                    break;
                }
                name.push(inner);
            }
            if !name.is_empty() {
                placeholders.insert(name);
            }
        }
    }

    placeholders
}

fn collect_leaf_paths(
    value: &serde_json::Value,
    current_path: &str,
    leaves: &mut Vec<(String, String)>,
) {
    match value {
        serde_json::Value::Object(map) => {
            for (key, val) in map {
                let next_path = if current_path.is_empty() {
                    key.clone()
                } else {
                    format!("{}.{}", current_path, key)
                };
                collect_leaf_paths(val, &next_path, leaves);
            }
        }
        serde_json::Value::String(s) => {
            leaves.push((current_path.to_string(), s.clone()));
        }
        _ => {}
    }
}

#[test]
fn test_100_percent_key_parity() {
    let en_root: serde_json::Value =
        serde_json::from_str(EN_JSON).expect("locales/en.json must be valid JSON");
    let rw_root: serde_json::Value =
        serde_json::from_str(RW_JSON).expect("locales/rw.json must be valid JSON");

    let mut en_leaves = Vec::new();
    let mut rw_leaves = Vec::new();

    collect_leaf_paths(&en_root, "", &mut en_leaves);
    collect_leaf_paths(&rw_root, "", &mut rw_leaves);

    let en_keys: HashSet<String> = en_leaves.iter().map(|(k, _)| k.clone()).collect();
    let rw_keys: HashSet<String> = rw_leaves.iter().map(|(k, _)| k.clone()).collect();

    // 1. Assert all en keys exist in rw
    let missing_in_rw: Vec<_> = en_keys.difference(&rw_keys).collect();
    assert!(
        missing_in_rw.is_empty(),
        "The following keys exist in en.json but are missing in rw.json: {:?}",
        missing_in_rw
    );

    // 2. Assert all rw keys exist in en
    let missing_in_en: Vec<_> = rw_keys.difference(&en_keys).collect();
    assert!(
        missing_in_en.is_empty(),
        "The following keys exist in rw.json but are missing in en.json: {:?}",
        missing_in_en
    );
}

#[test]
fn test_placeholder_variable_matching() {
    let en_root: serde_json::Value =
        serde_json::from_str(EN_JSON).expect("locales/en.json must be valid JSON");
    let rw_root: serde_json::Value =
        serde_json::from_str(RW_JSON).expect("locales/rw.json must be valid JSON");

    let mut en_leaves = Vec::new();
    let mut rw_leaves = Vec::new();

    collect_leaf_paths(&en_root, "", &mut en_leaves);
    collect_leaf_paths(&rw_root, "", &mut rw_leaves);

    let en_map: std::collections::HashMap<_, _> = en_leaves.into_iter().collect();
    let rw_map: std::collections::HashMap<_, _> = rw_leaves.into_iter().collect();

    for (key, en_str) in &en_map {
        let rw_str = rw_map
            .get(key)
            .unwrap_or_else(|| panic!("Key {} missing in rw", key));

        let en_placeholders = extract_placeholders(en_str);
        let rw_placeholders = extract_placeholders(rw_str);

        assert_eq!(
            en_placeholders, rw_placeholders,
            "Placeholder mismatch for key '{}': en has {:?}, rw has {:?}",
            key, en_placeholders, rw_placeholders
        );
    }
}

#[test]
fn test_error_and_notification_interpolations() {
    // 1. Insufficient custody error interpolation
    let en_err = t_args(
        Locale::En,
        "errors.insufficient_custody",
        &[
            ("wallet", "MoMo Main"),
            ("available", "5,000"),
            ("requested", "10,000"),
        ],
    );
    assert_eq!(
        en_err,
        "Wallet 'MoMo Main' only holds 5,000 RWF, which is not enough for 10,000 RWF."
    );

    let rw_err = t_args(
        Locale::Rw,
        "errors.insufficient_custody",
        &[
            ("wallet", "MoMo Main"),
            ("available", "5,000"),
            ("requested", "10,000"),
        ],
    );
    assert_eq!(
        rw_err,
        "Konti 'MoMo Main' ifite 5,000 RWF gusa; ntihagije ngo ukoreshe 10,000 RWF."
    );

    // 2. Handover completed notification interpolation
    let en_handover = t_args(
        Locale::En,
        "notifications.handover_completed",
        &[
            ("actor", "Gisa"),
            ("amount", "100,000"),
            ("from_wallet", "BK Treasury"),
            ("to_wallet", "Cash in Hand"),
        ],
    );
    assert_eq!(
        en_handover,
        "Gisa transferred 100,000 RWF from BK Treasury to Cash in Hand."
    );

    let rw_handover = t_args(
        Locale::Rw,
        "notifications.handover_completed",
        &[
            ("actor", "Gisa"),
            ("amount", "100,000"),
            ("from_wallet", "BK Treasury"),
            ("to_wallet", "Cash in Hand"),
        ],
    );
    assert_eq!(
        rw_handover,
        "Gisa yohereje 100,000 RWF kuva kuri 'BK Treasury' ajya kuri 'Cash in Hand'."
    );

    // 3. Simple translation
    assert_eq!(t(Locale::En, "common.save"), "Save");
    assert_eq!(t(Locale::Rw, "common.save"), "Bika");
}

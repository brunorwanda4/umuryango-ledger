use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

fn encode_ico(images: &[(u32, &[u8])]) -> Vec<u8> {
    let mut out = Vec::new();
    // ICONDIR Header
    out.extend_from_slice(&0u16.to_le_bytes()); // Reserved
    out.extend_from_slice(&1u16.to_le_bytes()); // Type: 1 = Icon
    out.extend_from_slice(&(images.len() as u16).to_le_bytes()); // Image count

    let mut offset = 6 + (images.len() * 16) as u32;

    for (size, bytes) in images {
        let w = if *size >= 256 { 0u8 } else { *size as u8 };
        let h = if *size >= 256 { 0u8 } else { *size as u8 };
        out.push(w);
        out.push(h);
        out.push(0); // Color count
        out.push(0); // Reserved
        out.extend_from_slice(&1u16.to_le_bytes()); // Color planes
        out.extend_from_slice(&32u16.to_le_bytes()); // Bits per pixel
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes()); // Res size
        out.extend_from_slice(&offset.to_le_bytes()); // Offset
        offset += bytes.len() as u32;
    }

    for (_, bytes) in images {
        out.extend_from_slice(bytes);
    }

    out
}

fn encode_icns(chunks: &[(&[u8; 4], &[u8])]) -> Vec<u8> {
    let mut payload = Vec::new();
    for (tag, bytes) in chunks {
        payload.extend_from_slice(*tag);
        let len = (8 + bytes.len()) as u32;
        payload.extend_from_slice(&len.to_be_bytes());
        payload.extend_from_slice(bytes);
    }
    let mut out = Vec::new();
    out.extend_from_slice(b"icns");
    let total_len = (8 + payload.len()) as u32;
    out.extend_from_slice(&total_len.to_be_bytes());
    out.extend_from_slice(&payload);
    out
}

fn main() {
    println!("cargo:rerun-if-changed=../../assets/branding/app-icon.png");
    println!("cargo:rerun-if-changed=assets/branding/app-icon.png");

    // Locate workspace root
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = if manifest_dir
        .join("../../assets/branding/app-icon.png")
        .exists()
    {
        manifest_dir.join("../..")
    } else if manifest_dir.join("assets/branding/app-icon.png").exists() {
        manifest_dir.clone()
    } else {
        manifest_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    };

    let icon_source = workspace_root.join("assets/branding/app-icon.png");

    if icon_source.exists() {
        if let Ok(src_bytes) = fs::read(&icon_source) {
            if let Ok(img) = image::load_from_memory(&src_bytes) {
                // Ensure output directories
                let gpui_icons_dir = manifest_dir.join("assets/icons");
                let mobile_dir = manifest_dir.join("assets/mobile");
                let web_public_dir = workspace_root.join("web/public");
                let branding_dir = workspace_root.join("assets/branding");

                let _ = fs::create_dir_all(&gpui_icons_dir);
                let _ = fs::create_dir_all(mobile_dir.join("mipmap-mdpi"));
                let _ = fs::create_dir_all(mobile_dir.join("mipmap-hdpi"));
                let _ = fs::create_dir_all(mobile_dir.join("mipmap-xhdpi"));
                let _ = fs::create_dir_all(mobile_dir.join("mipmap-xxhdpi"));
                let _ = fs::create_dir_all(mobile_dir.join("mipmap-xxxhdpi"));
                let _ = fs::create_dir_all(mobile_dir.join("ios"));
                let _ = fs::create_dir_all(&web_public_dir);
                let _ = fs::create_dir_all(&branding_dir);

                // Helper to resize and get PNG bytes
                let resize_png = |size: u32| -> Vec<u8> {
                    let resized =
                        img.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
                    let mut buf = Vec::new();
                    let mut cursor = Cursor::new(&mut buf);
                    let _ = resized.write_to(&mut cursor, image::ImageFormat::Png);
                    buf
                };

                let png_16 = resize_png(16);
                let png_32 = resize_png(32);
                let png_48 = resize_png(48);
                let png_64 = resize_png(64);
                let png_72 = resize_png(72);
                let png_96 = resize_png(96);
                let png_120 = resize_png(120);
                let png_128 = resize_png(128);
                let png_144 = resize_png(144);
                let png_152 = resize_png(152);
                let png_167 = resize_png(167);
                let png_180 = resize_png(180);
                let png_192 = resize_png(192);
                let png_256 = resize_png(256);
                let png_512 = resize_png(512);
                let png_1024 = resize_png(1024);

                // 1. GPUI Desktop Icons
                let _ = fs::write(gpui_icons_dir.join("icon-16.png"), &png_16);
                let _ = fs::write(gpui_icons_dir.join("icon-32.png"), &png_32);
                let _ = fs::write(gpui_icons_dir.join("icon-48.png"), &png_48);
                let _ = fs::write(gpui_icons_dir.join("icon-64.png"), &png_64);
                let _ = fs::write(gpui_icons_dir.join("icon-128.png"), &png_128);
                let _ = fs::write(gpui_icons_dir.join("icon-256.png"), &png_256);
                let _ = fs::write(gpui_icons_dir.join("icon-512.png"), &png_512);
                let _ = fs::write(gpui_icons_dir.join("icon-1024.png"), &png_1024);

                // Multi-size Windows app.ico (16, 32, 48, 256)
                let app_ico =
                    encode_ico(&[(16, &png_16), (32, &png_32), (48, &png_48), (256, &png_256)]);
                let app_ico_path = gpui_icons_dir.join("app.ico");
                let _ = fs::write(&app_ico_path, &app_ico);

                // macOS AppIcon.icns (128, 256, 512, 1024)
                let app_icns = encode_icns(&[
                    (b"ic07", &png_128),
                    (b"ic08", &png_256),
                    (b"ic09", &png_512),
                    (b"ic10", &png_1024),
                ]);
                let _ = fs::write(gpui_icons_dir.join("AppIcon.icns"), &app_icns);

                // 2. Android Mipmap Icons
                let _ = fs::write(mobile_dir.join("mipmap-mdpi/ic_launcher.png"), &png_48);
                let _ = fs::write(mobile_dir.join("mipmap-hdpi/ic_launcher.png"), &png_72);
                let _ = fs::write(mobile_dir.join("mipmap-xhdpi/ic_launcher.png"), &png_96);
                let _ = fs::write(mobile_dir.join("mipmap-xxhdpi/ic_launcher.png"), &png_144);
                let _ = fs::write(mobile_dir.join("mipmap-xxxhdpi/ic_launcher.png"), &png_192);

                // 3. iOS App Icons
                let _ = fs::write(mobile_dir.join("ios/AppIcon-60@2x.png"), &png_120);
                let _ = fs::write(mobile_dir.join("ios/AppIcon-60@3x.png"), &png_180);
                let _ = fs::write(mobile_dir.join("ios/AppIcon-76@2x.png"), &png_152);
                let _ = fs::write(mobile_dir.join("ios/AppIcon-83.5@2x.png"), &png_167);
                let _ = fs::write(mobile_dir.join("ios/AppIcon-1024.png"), &png_1024);

                // 4. Web Public Icons
                let favicon_ico = encode_ico(&[(16, &png_16), (32, &png_32), (48, &png_48)]);
                let _ = fs::write(web_public_dir.join("favicon.ico"), &favicon_ico);
                let _ = fs::write(web_public_dir.join("icon-192.png"), &png_192);
                let _ = fs::write(web_public_dir.join("icon-512.png"), &png_512);
                let _ = fs::write(web_public_dir.join("apple-touch-icon.png"), &png_180);

                // 5. Linux Desktop entry
                let desktop_entry = r#"[Desktop Entry]
Version=1.0
Type=Application
Name=Umuryango Ledger
GenericName=Family & Personal Finance Accountability
Comment=Multi-tenant family and personal finance accountability platform
Exec=app-gpui
Icon=umuryango-ledger
Terminal=false
Categories=Office;Finance;
StartupNotify=true
StartupWMClass=app-gpui
"#;
                let _ = fs::write(branding_dir.join("umuryango-ledger.desktop"), desktop_entry);
            }
        }
    }

    // Compile native Windows resource (taskbar and exe icon)
    #[cfg(target_os = "windows")]
    {
        let ico_path = manifest_dir.join("assets/icons/app.ico");
        if ico_path.exists() {
            let mut res = winres::WindowsResource::new();
            res.set_icon(&ico_path.to_string_lossy());
            let _ = res.compile();
        }
    }
}

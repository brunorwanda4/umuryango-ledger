//! Standalone multi-platform icon generator for Umuryango Ledger.
//! Generates Windows .ico, macOS .icns, Linux desktop icons, Android launcher mipmaps,
//! iOS app icons, and Next.js / PWA web icons from the master 1024x1024 app icon emblem.

use std::fs;
use std::io::Cursor;
use std::path::PathBuf;

fn encode_ico(images: &[(u32, &[u8])]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&0u16.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes());
    out.extend_from_slice(&(images.len() as u16).to_le_bytes());

    let mut offset = 6 + (images.len() * 16) as u32;

    for (size, bytes) in images {
        let w = if *size >= 256 { 0u8 } else { *size as u8 };
        let h = if *size >= 256 { 0u8 } else { *size as u8 };
        out.push(w);
        out.push(h);
        out.push(0);
        out.push(0);
        out.extend_from_slice(&1u16.to_le_bytes());
        out.extend_from_slice(&32u16.to_le_bytes());
        out.extend_from_slice(&(bytes.len() as u32).to_le_bytes());
        out.extend_from_slice(&offset.to_le_bytes());
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let root = if manifest_dir.join("assets/branding/app-icon.png").exists() {
        manifest_dir
    } else {
        manifest_dir.parent().unwrap().to_path_buf()
    };
    let icon_path = root.join("assets/branding/app-icon.png");

    println!("🎨 Reading master emblem from: {}", icon_path.display());
    let src_bytes = fs::read(&icon_path)?;
    let img = image::load_from_memory(&src_bytes)?;

    let gpui_icons_dir = root.join("crates/app-gpui/assets/icons");
    let mobile_dir = root.join("crates/app-gpui/assets/mobile");
    let web_public_dir = root.join("web/public");
    let branding_dir = root.join("assets/branding");

    fs::create_dir_all(&gpui_icons_dir)?;
    fs::create_dir_all(mobile_dir.join("mipmap-mdpi"))?;
    fs::create_dir_all(mobile_dir.join("mipmap-hdpi"))?;
    fs::create_dir_all(mobile_dir.join("mipmap-xhdpi"))?;
    fs::create_dir_all(mobile_dir.join("mipmap-xxhdpi"))?;
    fs::create_dir_all(mobile_dir.join("mipmap-xxxhdpi"))?;
    fs::create_dir_all(mobile_dir.join("ios"))?;
    fs::create_dir_all(&web_public_dir)?;
    fs::create_dir_all(&branding_dir)?;

    let resize_png = |size: u32| -> Vec<u8> {
        let resized = img.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
        let mut buf = Vec::new();
        let mut cursor = Cursor::new(&mut buf);
        let _ = resized.write_to(&mut cursor, image::ImageFormat::Png);
        buf
    };

    println!("⚙️ Generating multi-resolution PNGs...");
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

    // GPUI Desktop Icons
    println!("🖥️ Writing GPUI Desktop icons...");
    fs::write(gpui_icons_dir.join("icon-16.png"), &png_16)?;
    fs::write(gpui_icons_dir.join("icon-32.png"), &png_32)?;
    fs::write(gpui_icons_dir.join("icon-48.png"), &png_48)?;
    fs::write(gpui_icons_dir.join("icon-64.png"), &png_64)?;
    fs::write(gpui_icons_dir.join("icon-128.png"), &png_128)?;
    fs::write(gpui_icons_dir.join("icon-256.png"), &png_256)?;
    fs::write(gpui_icons_dir.join("icon-512.png"), &png_512)?;
    fs::write(gpui_icons_dir.join("icon-1024.png"), &png_1024)?;

    // Multi-size Windows app.ico
    let app_ico = encode_ico(&[
        (16, &png_16),
        (32, &png_32),
        (48, &png_48),
        (256, &png_256),
    ]);
    fs::write(gpui_icons_dir.join("app.ico"), &app_ico)?;

    // macOS AppIcon.icns
    let app_icns = encode_icns(&[
        (b"ic07", &png_128),
        (b"ic08", &png_256),
        (b"ic09", &png_512),
        (b"ic10", &png_1024),
    ]);
    fs::write(gpui_icons_dir.join("AppIcon.icns"), &app_icns)?;

    // Android launcher mipmaps
    println!("📱 Writing Android launcher mipmaps...");
    fs::write(mobile_dir.join("mipmap-mdpi/ic_launcher.png"), &png_48)?;
    fs::write(mobile_dir.join("mipmap-hdpi/ic_launcher.png"), &png_72)?;
    fs::write(mobile_dir.join("mipmap-xhdpi/ic_launcher.png"), &png_96)?;
    fs::write(mobile_dir.join("mipmap-xxhdpi/ic_launcher.png"), &png_144)?;
    fs::write(mobile_dir.join("mipmap-xxxhdpi/ic_launcher.png"), &png_192)?;

    // iOS app icons
    println!("🍏 Writing iOS launcher icons...");
    fs::write(mobile_dir.join("ios/AppIcon-60@2x.png"), &png_120)?;
    fs::write(mobile_dir.join("ios/AppIcon-60@3x.png"), &png_180)?;
    fs::write(mobile_dir.join("ios/AppIcon-76@2x.png"), &png_152)?;
    fs::write(mobile_dir.join("ios/AppIcon-83.5@2x.png"), &png_167)?;
    fs::write(mobile_dir.join("ios/AppIcon-1024.png"), &png_1024)?;

    // Next.js Web icons
    println!("🌐 Writing Web icons & favicon.ico...");
    let favicon_ico = encode_ico(&[
        (16, &png_16),
        (32, &png_32),
        (48, &png_48),
    ]);
    fs::write(web_public_dir.join("favicon.ico"), &favicon_ico)?;
    fs::write(web_public_dir.join("icon-192.png"), &png_192)?;
    fs::write(web_public_dir.join("icon-512.png"), &png_512)?;
    fs::write(web_public_dir.join("apple-touch-icon.png"), &png_180)?;

    // Linux Desktop entry
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
    fs::write(branding_dir.join("umuryango-ledger.desktop"), desktop_entry)?;

    println!("✅ All icons successfully generated across targets!");
    Ok(())
}

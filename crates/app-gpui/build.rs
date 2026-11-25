use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use image::{DynamicImage, GenericImageView, RgbaImage};

fn write_if_changed<P: AsRef<Path>>(path: P, content: &[u8]) {
    let p = path.as_ref();
    if let Ok(existing) = fs::read(p)
        && existing == content
    {
        return;
    }
    let _ = fs::write(p, content);
}

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

/// Crops a transparent logo to its non-transparent bounding box, centers it in a
/// square canvas with balanced padding (preserving exact aspect ratio), and
/// encodes it as a crisp PNG at `target_size x target_size`.
fn trim_and_square_png(img: &DynamicImage, target_size: u32) -> Vec<u8> {
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();

    let mut min_x = w;
    let mut min_y = h;
    let mut max_x = 0u32;
    let mut max_y = 0u32;
    let mut found = false;

    for (x, y, px) in rgba.enumerate_pixels() {
        if px.0[3] > 0 {
            found = true;
            if x < min_x {
                min_x = x;
            }
            if y < min_y {
                min_y = y;
            }
            if x > max_x {
                max_x = x;
            }
            if y > max_y {
                max_y = y;
            }
        }
    }

    let square_img = if found {
        let crop_w = max_x - min_x + 1;
        let crop_h = max_y - min_y + 1;
        let cropped = img.view(min_x, min_y, crop_w, crop_h).to_image();

        let longest = crop_w.max(crop_h);
        let pad = ((longest as f32) * 0.06).round() as u32;
        let side = longest + pad * 2;
        let offset_x = (side - crop_w) / 2;
        let offset_y = (side - crop_h) / 2;

        let mut canvas = RgbaImage::new(side, side);
        image::imageops::overlay(&mut canvas, &cropped, offset_x as i64, offset_y as i64);
        DynamicImage::ImageRgba8(canvas)
    } else {
        img.clone()
    };

    let resized = square_img.resize_exact(
        target_size,
        target_size,
        image::imageops::FilterType::Lanczos3,
    );
    let mut buf = Vec::new();
    let mut cursor = Cursor::new(&mut buf);
    let _ = resized.write_to(&mut cursor, image::ImageFormat::Png);
    buf
}

fn generate_logo_variant(src_path: &Path, gpui_out: &Path, web_out: &Path) {
    if !src_path.exists() {
        return;
    }
    if let Ok(src_bytes) = fs::read(src_path)
        && let Ok(img) = image::load_from_memory(&src_bytes)
    {
        let png_256 = trim_and_square_png(&img, 256);
        let png_512 = trim_and_square_png(&img, 512);
        write_if_changed(gpui_out, &png_256);
        write_if_changed(web_out, &png_512);
    }
}

fn main() {
    for file in ["icon.png", "logo.png", "logo-black.png", "logo-white.png"] {
        println!("cargo:rerun-if-changed=../../assets/branding/{file}");
        println!("cargo:rerun-if-changed=assets/branding/{file}");
    }

    // Locate workspace root
    let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = if manifest_dir.join("../../assets/branding/icon.png").exists() {
        manifest_dir.join("../..")
    } else if manifest_dir.join("assets/branding/icon.png").exists() {
        manifest_dir.clone()
    } else {
        manifest_dir
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    };

    let gpui_icons_dir = manifest_dir.join("assets/icons");
    let mobile_dir = manifest_dir.join("assets/mobile");
    let web_public_dir = workspace_root.join("web/public");
    let branding_dir = workspace_root.join("assets/branding");

    let _ = fs::create_dir_all(&gpui_icons_dir);
    let _ = fs::create_dir_all(&web_public_dir);
    let _ = fs::create_dir_all(&branding_dir);

    let icon_source = branding_dir.join("icon.png");

    if icon_source.exists()
        && let Ok(src_bytes) = fs::read(&icon_source)
        && let Ok(img) = image::load_from_memory(&src_bytes)
    {
        let _ = fs::create_dir_all(mobile_dir.join("mipmap-mdpi"));
        let _ = fs::create_dir_all(mobile_dir.join("mipmap-hdpi"));
        let _ = fs::create_dir_all(mobile_dir.join("mipmap-xhdpi"));
        let _ = fs::create_dir_all(mobile_dir.join("mipmap-xxhdpi"));
        let _ = fs::create_dir_all(mobile_dir.join("mipmap-xxxhdpi"));
        let _ = fs::create_dir_all(mobile_dir.join("ios"));

        // Helper to resize and get PNG bytes
        let resize_png = |size: u32| -> Vec<u8> {
            let resized = img.resize_exact(size, size, image::imageops::FilterType::Lanczos3);
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
        write_if_changed(gpui_icons_dir.join("icon-16.png"), &png_16);
        write_if_changed(gpui_icons_dir.join("icon-32.png"), &png_32);
        write_if_changed(gpui_icons_dir.join("icon-48.png"), &png_48);
        write_if_changed(gpui_icons_dir.join("icon-64.png"), &png_64);
        write_if_changed(gpui_icons_dir.join("icon-128.png"), &png_128);
        write_if_changed(gpui_icons_dir.join("icon-256.png"), &png_256);
        write_if_changed(gpui_icons_dir.join("icon-512.png"), &png_512);
        write_if_changed(gpui_icons_dir.join("icon-1024.png"), &png_1024);

        // Multi-size Windows app.ico (16, 32, 48, 256)
        let app_ico = encode_ico(&[(16, &png_16), (32, &png_32), (48, &png_48), (256, &png_256)]);
        let app_ico_path = gpui_icons_dir.join("app.ico");
        write_if_changed(&app_ico_path, &app_ico);

        // macOS AppIcon.icns (128, 256, 512, 1024)
        let app_icns = encode_icns(&[
            (b"ic07", &png_128),
            (b"ic08", &png_256),
            (b"ic09", &png_512),
            (b"ic10", &png_1024),
        ]);
        write_if_changed(gpui_icons_dir.join("AppIcon.icns"), &app_icns);

        // 2. Android Mipmap Icons
        write_if_changed(mobile_dir.join("mipmap-mdpi/ic_launcher.png"), &png_48);
        write_if_changed(mobile_dir.join("mipmap-hdpi/ic_launcher.png"), &png_72);
        write_if_changed(mobile_dir.join("mipmap-xhdpi/ic_launcher.png"), &png_96);
        write_if_changed(mobile_dir.join("mipmap-xxhdpi/ic_launcher.png"), &png_144);
        write_if_changed(mobile_dir.join("mipmap-xxxhdpi/ic_launcher.png"), &png_192);

        // 3. iOS App Icons
        write_if_changed(mobile_dir.join("ios/AppIcon-60@2x.png"), &png_120);
        write_if_changed(mobile_dir.join("ios/AppIcon-60@3x.png"), &png_180);
        write_if_changed(mobile_dir.join("ios/AppIcon-76@2x.png"), &png_152);
        write_if_changed(mobile_dir.join("ios/AppIcon-83.5@2x.png"), &png_167);
        write_if_changed(mobile_dir.join("ios/AppIcon-1024.png"), &png_1024);

        // 4. Web Public Icons
        let favicon_ico = encode_ico(&[(16, &png_16), (32, &png_32), (48, &png_48)]);
        write_if_changed(web_public_dir.join("favicon.ico"), &favicon_ico);
        write_if_changed(web_public_dir.join("icon-192.png"), &png_192);
        write_if_changed(web_public_dir.join("icon-512.png"), &png_512);
        write_if_changed(web_public_dir.join("apple-touch-icon.png"), &png_180);

        // 5. Linux Desktop entry
        let desktop_entry = r#"[Desktop Entry]
Version=1.0
Type=Application
Name=Umuryango Ledger
GenericName=Family & Personal Finance Accountability
Comment=Multi-tenant family and personal finance accountability platform
Exec=umuryango-app
Icon=umuryango-ledger
Terminal=false
Categories=Office;Finance;
StartupNotify=true
StartupWMClass=umuryango-app
"#;
        write_if_changed(
            branding_dir.join("umuryango-ledger.desktop"),
            desktop_entry.as_bytes(),
        );
    }

    // 6. Trimmed & square-centered Brand Logos (Primary Red, Black, White) for GPUI & Web
    generate_logo_variant(
        &branding_dir.join("logo.png"),
        &gpui_icons_dir.join("logo-256.png"),
        &web_public_dir.join("logo.png"),
    );
    generate_logo_variant(
        &branding_dir.join("logo-black.png"),
        &gpui_icons_dir.join("logo-black-256.png"),
        &web_public_dir.join("logo-black.png"),
    );
    generate_logo_variant(
        &branding_dir.join("logo-white.png"),
        &gpui_icons_dir.join("logo-white-256.png"),
        &web_public_dir.join("logo-white.png"),
    );

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

#!/usr/bin/env bun
import { createHash } from "node:crypto";
import {
  copyFileSync,
  existsSync,
  mkdirSync,
  readdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { spawnSync } from "node:child_process";
import os from "node:os";
import path from "node:path";

const args = process.argv.slice(2);
const targetArg = (args.find((a) => !a.startsWith("-")) || "all").toLowerCase();
const shouldInstall = args.includes("--install");
const buildAab = args.includes("--aab");

if (args.includes("-h") || args.includes("--help")) {
  console.log(`
Umuryango Cross-Platform Release Builder (Windows & Android)

Usage:
  bun run build:all                  Build both Windows (.exe) and Android (.apk) into dist/
  bun run build:windows              Build Windows desktop executable into dist/
  bun run build:android              Build Android arm64 APK into dist/
  bun run build:android -- --install Build Android APK and install/launch on connected USB device
  bun run build:android -- --aab     Build Android App Bundle (.aab) for Google Play
`);
  process.exit(0);
}

const rootDir = process.cwd();
const distDir = path.join(rootDir, "dist");
mkdirSync(distDir, { recursive: true });

function writeSha256(filePath) {
  const bytes = readFileSync(filePath);
  const hash = createHash("sha256").update(bytes).digest("hex");
  const fileName = path.basename(filePath);
  const shaPath = `${filePath}.sha256`;
  writeFileSync(shaPath, `${hash}  ${fileName}\n`, "utf8");
  return { hash, shaPath };
}

function runCommand(cmd, cmdArgs, options = {}) {
  console.log(`\n==> Running: ${cmd} ${cmdArgs.join(" ")}`);
  const res = spawnSync(cmd, cmdArgs, {
    stdio: "inherit",
    cwd: options.cwd || rootDir,
    env: options.env || process.env,
    shell: process.platform === "win32" && cmd.endsWith(".bat"),
  });
  if (res.status !== 0) {
    console.error(`Command failed with exit code ${res.status ?? 1}: ${cmd} ${cmdArgs.join(" ")}`);
    process.exit(res.status ?? 1);
  }
}

function buildWindows() {
  console.log("\n====================================================");
  console.log("  Building Umuryango for Windows (x86_64)");
  console.log("====================================================");

  runCommand("cargo", ["build", "--release", "-p", "umuryango-app", "--bin", "umuryango-app"]);

  const exeExt = process.platform === "win32" ? ".exe" : "";
  const sourceBin = path.join(rootDir, "target", "release", `umuryango-app${exeExt}`);
  if (!existsSync(sourceBin)) {
    console.error(`Expected compiled binary not found at: ${sourceBin}`);
    process.exit(1);
  }

  const outBin = path.join(
    distDir,
    process.platform === "win32"
      ? "umuryango-app-windows-x86_64.exe"
      : `umuryango-app-${process.platform}-${process.arch}`,
  );
  copyFileSync(sourceBin, outBin);
  const { hash, shaPath } = writeSha256(outBin);

  console.log(`\n[Windows Build Complete]`);
  console.log(`  Binary: ${outBin}`);
  console.log(`  SHA256: ${hash} (${shaPath})`);
}

function detectAndroidEnv() {
  const env = { ...process.env };
  const home = os.homedir();

  // 1. Detect Android SDK
  const sdkCandidates = [
    env.ANDROID_HOME,
    env.ANDROID_SDK_ROOT,
    process.platform === "win32"
      ? path.join(env.LOCALAPPDATA || path.join(home, "AppData", "Local"), "Android", "Sdk")
      : process.platform === "darwin"
        ? path.join(home, "Library", "Android", "sdk")
        : path.join(home, "Android", "Sdk"),
  ].filter(Boolean);

  const sdkPath = sdkCandidates.find((p) => existsSync(p));
  if (sdkPath) {
    env.ANDROID_HOME = sdkPath;
    env.ANDROID_SDK_ROOT = sdkPath;
  }

  // 2. Detect Android NDK
  let ndkPath = env.ANDROID_NDK_HOME || env.NDK_HOME;
  if ((!ndkPath || !existsSync(ndkPath)) && sdkPath) {
    const ndkRoot = path.join(sdkPath, "ndk");
    if (existsSync(ndkRoot)) {
      const versions = readdirSync(ndkRoot).sort();
      if (versions.length > 0) {
        ndkPath = path.join(ndkRoot, versions[versions.length - 1]);
      }
    }
  }
  if (ndkPath && existsSync(ndkPath)) {
    env.ANDROID_NDK_HOME = ndkPath;
    env.NDK_HOME = ndkPath;
  }

  // 3. Detect JAVA_HOME on Windows if not configured
  if ((!env.JAVA_HOME || !existsSync(env.JAVA_HOME)) && process.platform === "win32") {
    const jdkRoots = [
      "C:\\Program Files\\Microsoft",
      "C:\\Program Files\\Eclipse Adoptium",
      "C:\\Program Files\\Java",
      "C:\\Program Files\\Android\\Android Studio\\jbr",
    ];
    for (const root of jdkRoots) {
      if (!existsSync(root)) continue;
      if (root.endsWith("jbr")) {
        env.JAVA_HOME = root;
        break;
      }
      const dirs = readdirSync(root)
        .filter((d) => d.toLowerCase().startsWith("jdk"))
        .sort()
        .reverse();
      if (dirs.length > 0) {
        env.JAVA_HOME = path.join(root, dirs[0]);
        break;
      }
    }
  }

  return { env, sdkPath, ndkPath };
}

function buildAndroid() {
  console.log("\n====================================================");
  console.log("  Building Umuryango for Android (arm64-v8a)");
  console.log("====================================================");

  const { env, sdkPath, ndkPath } = detectAndroidEnv();
  if (!ndkPath) {
    console.error(
      "Android NDK not found. Install Android NDK via Android Studio SDK Manager or set ANDROID_NDK_HOME.",
    );
    process.exit(1);
  }

  console.log(`  ANDROID_HOME:     ${sdkPath || "(not set)"}`);
  console.log(`  ANDROID_NDK_HOME: ${ndkPath}`);
  console.log(`  JAVA_HOME:        ${env.JAVA_HOME || "(system default)"}`);

  const jniLibsDir = path.join(
    rootDir,
    "crates",
    "mobile",
    "android",
    "app",
    "src",
    "main",
    "jniLibs",
  );

  runCommand(
    "cargo",
    [
      "ndk",
      "-t",
      "arm64-v8a",
      "-o",
      jniLibsDir,
      "--platform",
      "31",
      "build",
      "--release",
      "-p",
      "umuryango-mobile",
    ],
    { env },
  );

  // Clean any stray intermediate .so files
  const arm64Dir = path.join(jniLibsDir, "arm64-v8a");
  if (existsSync(arm64Dir)) {
    for (const file of readdirSync(arm64Dir)) {
      if (file !== "libumuryango_mobile.so" && file.endsWith(".so")) {
        rmSync(path.join(arm64Dir, file), { force: true });
      }
    }
  }

  const androidDir = path.join(rootDir, "crates", "mobile", "android");
  const gradlew = process.platform === "win32" ? path.join(androidDir, "gradlew.bat") : "./gradlew";

  if (buildAab) {
    runCommand(gradlew, ["bundlePlayRelease", "-PallowDebugSigning=true"], {
      cwd: androidDir,
      env,
    });
    const aabSource = path.join(
      androidDir,
      "app",
      "build",
      "outputs",
      "bundle",
      "playRelease",
      "app-play-release.aab",
    );
    const outAab = path.join(distDir, "umuryango-app-android-arm64.aab");
    copyFileSync(aabSource, outAab);
    const { hash, shaPath } = writeSha256(outAab);
    console.log(`\n[Android AAB Build Complete]`);
    console.log(`  Bundle: ${outAab}`);
    console.log(`  SHA256: ${hash} (${shaPath})`);
    return;
  }

  runCommand(gradlew, ["assembleSideloadRelease", "-PallowDebugSigning=true"], {
    cwd: androidDir,
    env,
  });

  const apkCandidates = [
    path.join(
      androidDir,
      "app",
      "build",
      "outputs",
      "apk",
      "sideload",
      "release",
      "app-sideload-release.apk",
    ),
    path.join(
      androidDir,
      "app",
      "build",
      "outputs",
      "apk",
      "sideload",
      "release",
      "app-sideload-release-unsigned.apk",
    ),
  ];

  const apkSource = apkCandidates.find((p) => existsSync(p));
  if (!apkSource) {
    console.error(`Expected Android APK not found. Checked:\n  ${apkCandidates.join("\n  ")}`);
    process.exit(1);
  }

  const outApk = path.join(distDir, "umuryango-app-android-arm64.apk");
  copyFileSync(apkSource, outApk);
  const { hash, shaPath } = writeSha256(outApk);

  console.log(`\n[Android APK Build Complete]`);
  console.log(`  APK:    ${outApk}`);
  console.log(`  SHA256: ${hash} (${shaPath})`);

  if (shouldInstall) {
    const adbBin =
      sdkPath &&
      existsSync(
        path.join(sdkPath, "platform-tools", process.platform === "win32" ? "adb.exe" : "adb"),
      )
        ? path.join(sdkPath, "platform-tools", process.platform === "win32" ? "adb.exe" : "adb")
        : "adb";

    runCommand(adbBin, ["install", "-r", outApk], { env });
    runCommand(
      adbBin,
      [
        "shell",
        "am",
        "start",
        "-n",
        "rw.umuryango.app/dev.gpui.mobile.MainActivity",
        "-a",
        "android.intent.action.MAIN",
        "-c",
        "android.intent.category.LAUNCHER",
      ],
      { env },
    );
  }
}

if (targetArg === "windows" || targetArg === "win") {
  buildWindows();
} else if (targetArg === "android" || targetArg === "apk") {
  buildAndroid();
} else if (targetArg === "all" || targetArg === "both") {
  buildWindows();
  buildAndroid();
} else {
  console.error(`Unknown build target '${targetArg}'. Expected 'windows', 'android', or 'all'.`);
  process.exit(1);
}

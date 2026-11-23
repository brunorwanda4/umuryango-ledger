#!/usr/bin/env bun
import { readFileSync, writeFileSync } from "node:fs";
import { spawnSync } from "node:child_process";
import path from "node:path";

const rawArgs = process.argv.slice(2);
const flags = new Set(rawArgs.filter((a) => a.startsWith("-")));
const positional = rawArgs.filter((a) => !a.startsWith("-"));
const arg = (positional[0] || "patch").trim().toLowerCase();

if (flags.has("-h") || flags.has("--help") || arg === "help") {
  console.log(`
Umuryango Self-Update Release Publisher (Windows + Android Mobile)

Usage:
  bun run release                      Bump patch version (e.g. 0.1.1 -> 0.1.2), commit, tag, and push
  bun run release patch                Bump patch version (triggers GitHub Actions for Windows .exe + Android .apk)
  bun run release minor                Bump minor version (e.g. 0.1.1 -> 0.2.0)
  bun run release major                Bump major version (e.g. 0.1.1 -> 1.0.0)
  bun run release <x.y.z>              Set explicit version (e.g. 0.2.5)

Optional Local Build Flags:
  --build                              Also build both Windows (.exe) and Android (.apk) locally into dist/
  --build-mobile / --mobile / --android Also build Android (.apk) locally into dist/
  --build-windows / --windows          Also build Windows (.exe) locally into dist/
  --install                            Install and launch the built Android .apk on a connected phone via adb
`);
  process.exit(0);
}

const rootDir = process.cwd();
const cargoPath = path.join(rootDir, "Cargo.toml");
const pkgPath = path.join(rootDir, "package.json");

const cargoToml = readFileSync(cargoPath, "utf8");
const versionMatch = cargoToml.match(/^version\s*=\s*"(\d+)\.(\d+)\.(\d+)"/m);

if (!versionMatch) {
  console.error("Failed to locate workspace version in Cargo.toml");
  process.exit(1);
}

const major = Number(versionMatch[1]);
const minor = Number(versionMatch[2]);
const patch = Number(versionMatch[3]);
const currentVersion = `${major}.${minor}.${patch}`;

let nextVersion = "";
if (arg === "patch") {
  nextVersion = `${major}.${minor}.${patch + 1}`;
} else if (arg === "minor") {
  nextVersion = `${major}.${minor + 1}.0`;
} else if (arg === "major") {
  nextVersion = `${major + 1}.0.0`;
} else if (/^v?\d+\.\d+\.\d+$/.test(arg)) {
  nextVersion = arg.replace(/^v/, "");
} else {
  console.error(`Invalid release argument '${arg}'. Expected patch, minor, major, or x.y.z.`);
  process.exit(1);
}

const tag = `v${nextVersion}`;
console.log(`Bumping Umuryango version: v${currentVersion} -> ${tag}`);

const updatedCargo = cargoToml.replace(
  /^version\s*=\s*"\d+\.\d+\.\d+"/m,
  `version = "${nextVersion}"`,
);
writeFileSync(cargoPath, updatedCargo, "utf8");

const pkgJson = JSON.parse(readFileSync(pkgPath, "utf8"));
pkgJson.version = nextVersion;
writeFileSync(pkgPath, `${JSON.stringify(pkgJson, null, 2)}\n`, "utf8");

const wantAll = flags.has("--build") || flags.has("--all");
const wantMobile =
  wantAll ||
  flags.has("--build-mobile") ||
  flags.has("--mobile") ||
  flags.has("--android") ||
  flags.has("--install");
const wantWindows = wantAll || flags.has("--build-windows") || flags.has("--windows");

if (wantMobile || wantWindows) {
  const target = wantMobile && wantWindows ? "all" : wantMobile ? "android" : "windows";
  const buildArgs = ["run", "scripts/build-release.js", target];
  if (flags.has("--install")) {
    buildArgs.push("--install");
  }
  console.log(`\nRunning local ${target} release build for ${tag}...`);
  const buildRes = spawnSync("bun", buildArgs, {
    cwd: rootDir,
    stdio: "inherit",
  });
  if (buildRes.status !== 0) {
    console.error(`Local release build failed with exit code ${buildRes.status}`);
    process.exit(buildRes.status ?? 1);
  }
}

function runGit(args) {
  const res = spawnSync("git", args, { stdio: "inherit" });
  if (res.status !== 0) {
    console.error(`git ${args.join(" ")} failed with exit code ${res.status}`);
    process.exit(res.status ?? 1);
  }
}

runGit(["add", "Cargo.toml", "package.json"]);
runGit(["commit", "-m", `chore(release): ${tag}`]);
runGit(["tag", "-a", tag, "-m", `Release ${tag}`]);
runGit(["push", "origin", "HEAD", "--follow-tags"]);

console.log(
  `\nPublished ${tag}! GitHub Actions will build both Windows (umuryango-app-windows-x86_64.exe) and Android (umuryango-app-android-arm64.apk) and attach them to the GitHub Release so running desktop and mobile apps self-update.`,
);

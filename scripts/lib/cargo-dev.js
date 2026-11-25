// Shared Rust dev-service definitions and runners.
// Used by dev-all.js, dev-backend.js and dev-native.js so watch paths,
// cargo commands and process-tree handling live in exactly one place.
import { spawn } from "node:child_process";
import { existsSync, watch as fsWatch } from "node:fs";

// Paths are relative to the repo root. Shared crates appear in both lists,
// so a change there restarts both services.
const SHARED_WATCH = ["crates/core-domain", "crates/i18n", "locales", "Cargo.toml"];

export const RUST_SERVICES = {
  backend: {
    name: "backend",
    cargoArgs: ["run", "-p", "umuryango-api-server", "--bin", "umuryango-api-server"],
    watch: ["crates/api-server", ...SHARED_WATCH],
  },
  native: {
    name: "native",
    cargoArgs: ["run", "-p", "umuryango-app"],
    watch: [
      "crates/app-gpui",
      "crates/storage-local",
      "crates/api-client",
      "assets",
      ...SHARED_WATCH,
    ],
  },
};

export const DEBOUNCE_MS = 400;
export const KILL_TIMEOUT_MS = 5000;
export const CARGO_WATCH_INSTALL = "cargo install cargo-watch --locked";

// Files written by crates/app-gpui/build.rs plus editor temp files.
// Watching them would trigger rebuild loops.
const IGNORE_GLOBS = [
  "**/*.ico",
  "**/*.icns",
  "**/icon-*",
  "**/logo-*-256.png",
  "**/logo-256.png",
  "**/AppIcon*",
  "**/ic_launcher*",
  "**/*.desktop",
  "**/*~",
  "**/*.swp",
  "**/*.tmp",
];
const IGNORED_DIRS = new Set(["target", "node_modules", ".next", ".git", "dist", "out"]);

export function hasCargoWatch() {
  return Bun.which("cargo-watch") !== null;
}

export function existingPaths(paths) {
  return paths.filter((p) => existsSync(p));
}

export function cargoRunCommand(service) {
  return ["cargo", ...service.cargoArgs];
}

// cargo-watch keeps the process alive, rebuilds on change and kills the whole
// process group of the previous run (so the old binary never holds a port).
export function cargoWatchCommand(service) {
  const cmd = ["cargo", "watch", "--why", "--delay", String(DEBOUNCE_MS / 1000)];
  for (const p of existingPaths(service.watch)) cmd.push("--watch", p);
  for (const glob of IGNORE_GLOBS) cmd.push("--ignore", glob);
  cmd.push("--exec", service.cargoArgs.join(" "));
  return cmd;
}

export function isIgnoredPath(filename) {
  if (!filename) return true;
  const segments = filename.split(/[\\/]/);
  if (segments.some((seg) => IGNORED_DIRS.has(seg))) return true;
  const base = segments.at(-1) ?? "";
  return /\.(ico|icns|desktop|swp|tmp)$|~$|^icon-|^logo-.*256\.png$|^AppIcon|^ic_launcher/i.test(
    base,
  );
}

export function spawnTree(cmd, options = {}) {
  const [command, ...args] = cmd;
  return spawn(command, args, {
    stdio: "inherit",
    ...options,
    // Own process group on POSIX, so the whole tree (cargo -> binary) can be killed.
    detached: process.platform !== "win32",
  });
}

export function killTree(child, signal = "SIGTERM") {
  if (!child || child.pid == null) return;
  if (process.platform === "win32") {
    // taskkill /T kills the whole tree (cargo -> built binary)
    spawn("taskkill", ["/pid", String(child.pid), "/T", "/F"], { stdio: "ignore" });
    return;
  }
  try {
    process.kill(-child.pid, signal);
  } catch {
    try {
      child.kill(signal);
    } catch {}
  }
}

export async function stopTree(child) {
  if (!child) return;
  if (child.exitCode === null && child.signalCode === null) {
    const exited = new Promise((resolve) => child.once("exit", resolve));
    killTree(child, "SIGINT");
    await Promise.race([exited, new Promise((resolve) => setTimeout(resolve, KILL_TIMEOUT_MS))]);
  }
  // Kill leftovers (e.g. the compiled API server still holding port 4433)
  killTree(child, "SIGKILL");
}

function runOnce(cmd) {
  const child = spawnTree(cmd);
  const stop = () => killTree(child, "SIGINT");
  process.on("SIGINT", stop);
  process.on("SIGTERM", stop);
  child.on("exit", (code) => process.exit(code ?? 0));
}

// Fallback used only when cargo-watch is not installed.
function runWithFsWatcher(service) {
  const cmd = cargoRunCommand(service);
  let child = spawnTree(cmd);
  let chain = Promise.resolve();
  let timer;
  let stopping = false;

  const restart = (file) => {
    chain = chain.then(async () => {
      if (stopping) return;
      console.log(`[${service.name}] change in ${file}, rebuilding...`);
      await stopTree(child);
      if (!stopping) child = spawnTree(cmd);
    });
  };

  const watchers = existingPaths(service.watch).map((root) =>
    fsWatch(root, { recursive: true }, (_event, filename) => {
      const file = filename ?? root;
      if (isIgnoredPath(file)) return;
      clearTimeout(timer);
      timer = setTimeout(() => restart(file), DEBOUNCE_MS);
    }),
  );

  const stop = async () => {
    if (stopping) return;
    stopping = true;
    clearTimeout(timer);
    for (const w of watchers) w.close();
    await stopTree(child);
    process.exit(0);
  };
  process.on("SIGINT", stop);
  process.on("SIGTERM", stop);
}

export function runRustService(key, { watch }) {
  const service = RUST_SERVICES[key];
  if (!watch) return runOnce(cargoRunCommand(service));
  if (hasCargoWatch()) return runOnce(cargoWatchCommand(service));

  console.warn(
    `[${service.name}] cargo-watch not found, using the built-in watcher. ` +
      `For faster, cleaner restarts run: ${CARGO_WATCH_INSTALL}`,
  );
  return runWithFsWatcher(service);
}

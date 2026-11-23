import { watch } from "node:fs";

const watchTargets = [
  "crates/api-server",
  "crates/core-domain",
  "crates/i18n",
  "locales",
  "Cargo.toml",
  "Cargo.lock",
];
const IGNORED_DIRS = new Set(["target", "node_modules", ".git", ".next", "dist", "out"]);
const watchers = [];

let child;
let restartTimer;
let restarting = false;
let restartPending = false;
let stopping = false;

function isIgnored(filename) {
  if (!filename) return true;
  const segments = filename.split(/[\\/]/);
  if (segments.some((seg) => IGNORED_DIRS.has(seg))) return true;
  return /(~|\.swp|\.tmp)$/i.test(filename);
}

function startBackend() {
  console.log("Starting backend in development mode...");
  child = Bun.spawn(
    ["cargo", "run", "-p", "umuryango-api-server", "--bin", "umuryango-api-server"],
    {
      stdin: "inherit",
      stdout: "inherit",
      stderr: "inherit",
    },
  );
}

async function restartBackend() {
  if (restarting) {
    restartPending = true;
    return;
  }

  restarting = true;
  console.log("Changes detected; rebuilding backend...");

  if (child) {
    child.kill();
    await child.exited;
    child = undefined;
  }

  startBackend();
  restarting = false;

  if (restartPending) {
    restartPending = false;
    scheduleRestart();
  }
}

function scheduleRestart() {
  if (restartTimer) {
    clearTimeout(restartTimer);
  }

  restartTimer = setTimeout(() => {
    restartTimer = undefined;
    void restartBackend();
  }, 300);
}

function stop() {
  if (stopping) {
    return;
  }

  stopping = true;
  for (const watcher of watchers) {
    watcher.close();
  }
  if (restartTimer) {
    clearTimeout(restartTimer);
  }
  child?.kill();
}

const watchEnabled = !process.argv.includes("--no-watch");

if (watchEnabled) {
  for (const target of watchTargets) {
    const watcher = watch(target, { recursive: true }, (_eventType, filename) => {
      if (!stopping && !isIgnored(filename)) {
        scheduleRestart();
      }
    });
    watchers.push(watcher);
  }
}

process.on("SIGINT", stop);
process.on("SIGTERM", stop);

startBackend();

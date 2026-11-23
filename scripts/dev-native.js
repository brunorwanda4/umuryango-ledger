import { existsSync, watch } from "node:fs";

const watchTargets = ["crates", "assets", "locales", "Cargo.toml", "Cargo.lock"].filter(existsSync);
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
  if (
    filename.includes("app.ico") ||
    filename.includes("AppIcon.icns") ||
    filename.includes("icon-") ||
    filename.includes("ic_launcher") ||
    filename.includes(".desktop")
  ) {
    return true;
  }
  return /(~|\.swp|\.tmp)$/i.test(filename);
}

function startApplication() {
  console.log("Starting native application...");
  child = Bun.spawn(["cargo", "run", "-p", "umuryango-app"], {
    stdin: "inherit",
    stdout: "inherit",
    stderr: "inherit",
  });
}

async function restartApplication() {
  if (restarting) {
    restartPending = true;
    return;
  }

  restarting = true;
  console.log("Changes detected; rebuilding native application...");

  if (child) {
    child.kill();
    await child.exited;
    child = undefined;
  }

  startApplication();
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
    void restartApplication();
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

startApplication();

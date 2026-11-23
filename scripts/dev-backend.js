import { watch } from "node:fs";

const watchTargets = [
  "crates/api-server",
  "crates/core-domain",
  "crates/i18n",
  "locales",
  "Cargo.toml",
  "Cargo.lock",
];
const watchers = [];

let child;
let restartTimer;
let restarting = false;
let restartPending = false;
let stopping = false;

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

for (const target of watchTargets) {
  const watcher = watch(target, { recursive: true }, () => {
    if (!stopping) {
      scheduleRestart();
    }
  });
  watchers.push(watcher);
}

process.on("SIGINT", stop);
process.on("SIGTERM", stop);

startBackend();

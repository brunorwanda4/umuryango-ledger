import { watch } from "node:fs";

const watchTargets = ["app", "crates", "assets", "Cargo.toml", "Cargo.lock"];
const watchers = [];

let child;
let restartTimer;
let restarting = false;
let restartPending = false;
let stopping = false;

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

startApplication();

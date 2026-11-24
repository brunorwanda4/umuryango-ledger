#!/usr/bin/env bun
import { watch as fsWatch, existsSync } from "node:fs";
import path from "node:path";
import {
  CARGO_WATCH_INSTALL,
  DEBOUNCE_MS,
  RUST_SERVICES,
  cargoRunCommand,
  cargoWatchCommand,
  hasCargoWatch,
  isIgnoredPath,
  spawnTree,
  stopTree,
} from "./lib/cargo-dev.js";

const args = process.argv.slice(2);

const colors = {
  reset: "\x1b[0m",
  bold: "\x1b[1m",
  dim: "\x1b[2m",
  cyan: "\x1b[36m",
  green: "\x1b[32m",
  magenta: "\x1b[35m",
  yellow: "\x1b[33m",
  red: "\x1b[31m",
};

const bunExe = process.execPath;

function printHelp() {
  console.log(`
Umuryango Concurrent Development Runner

Usage:
  bun run dev:all                Run backend and web frontend concurrently (default)
  bun run dev:native:all         Run backend and native desktop app concurrently
  bun run dev:all --native       Run backend and native desktop app
  bun run dev:all not --web      Run backend and native desktop app (exclude web)
  bun run dev:all --no-web       Run backend and native desktop app (exclude web)
  bun run dev:all --all          Run backend, web frontend, and native app concurrently
  bun run dev:all --only <list>  Run only specified services (e.g. --only app,backend)
  bun run dev:all --raw          Run without prefixed log output
  bun run dev:all --no-watch     Start services without restarting them on file changes

Service names / aliases:
  - backend (or: api, server)
  - web     (or: frontend, portal)
  - app     (or: native, desktop)

Options:
  --native, --app     Run backend and native desktop app
  --web, --frontend   Run backend and web frontend
  --all               Run backend, web frontend, and native app concurrently
  not --web, --no-web Exclude web frontend (runs backend + native app)
  not --app, --no-app Exclude native app (runs backend + web frontend)
  --no-backend        Exclude backend API
  --only <services>   Run only specified comma-separated services
  --raw               Inherit stdout/stderr directly (no prefixing)
  --no-watch          Disable automatic restarts on file changes
  -h, --help          Show this help message

Watching:
  Rust services (backend, app) run under cargo-watch: a change in a service's
  crates rebuilds and restarts only that service. Without cargo-watch installed
  a built-in file watcher is used instead (${CARGO_WATCH_INSTALL}).
  The frontend is never restarted by the runner (Next.js hot-reloads).
`);
}

function resolveService(str) {
  if (!str) return null;
  const s = str.toLowerCase().replace(/^-+/, "").trim();
  if (["backend", "api", "server", "actix"].includes(s)) return "backend";
  if (["frontend", "web", "next", "portal"].includes(s)) return "frontend";
  if (["native", "app", "desktop", "gpui", "client"].includes(s)) return "native";
  return null;
}

const excluded = new Set();
const explicitlyIncluded = new Set();
let runAll = false;
let rawOutput = false;
let onlyMode = false;
let watchEnabled = true;

for (let i = 0; i < args.length; i++) {
  const arg = args[i].trim();
  const lower = arg.toLowerCase();

  if (arg === "-h" || arg === "--help") {
    printHelp();
    process.exit(0);
  }

  if (arg === "--raw") {
    rawOutput = true;
    continue;
  }

  if (arg === "--no-watch") {
    watchEnabled = false;
    continue;
  }

  if (arg === "--all") {
    runAll = true;
    continue;
  }

  // Exclusion patterns: "not <svc>", "without <svc>", "--not <svc>", "--without <svc>"
  if (
    ["not", "without", "--not", "--without", "exclude", "--exclude", "skip", "--skip"].includes(
      lower,
    )
  ) {
    const nextArg = args[i + 1];
    if (nextArg) {
      const svc = resolveService(nextArg);
      if (svc) {
        excluded.add(svc);
        i++;
        continue;
      }
    }
  }

  // Negation prefix patterns: --no-web, --not-web, --without-web, etc.
  if (
    lower.startsWith("--no-") ||
    lower.startsWith("--not-") ||
    lower.startsWith("--without-") ||
    lower.startsWith("-no-")
  ) {
    const rawName = lower.replace(/^--?(no|not|without)-/, "");
    const svc = resolveService(rawName);
    if (svc) {
      excluded.add(svc);
      continue;
    }
  }

  // --only patterns: --only=app,backend or --only app
  if (lower.startsWith("--only=") || lower.startsWith("--services=")) {
    onlyMode = true;
    const parts = lower.split("=")[1].split(/[,+]/);
    for (const part of parts) {
      const svc = resolveService(part);
      if (svc) explicitlyIncluded.add(svc);
    }
    continue;
  }

  if (lower === "--only" || lower === "only") {
    onlyMode = true;
    const nextArg = args[i + 1];
    if (nextArg && !nextArg.startsWith("-")) {
      const parts = nextArg.split(/[,+]/);
      for (const part of parts) {
        const svc = resolveService(part);
        if (svc) explicitlyIncluded.add(svc);
      }
      i++;
    }
    continue;
  }

  // Direct positive service flags: --native, --app, --web, --frontend, --backend, --api
  if (arg.startsWith("-")) {
    const directSvc = resolveService(arg);
    if (directSvc) {
      explicitlyIncluded.add(directSvc);
      continue;
    }
  }
}

// ---- Service definitions ----

const useCargoWatch = watchEnabled && hasCargoWatch();

function rustService(key, color, desc) {
  const def = RUST_SERVICES[key];
  return {
    name: def.name,
    color,
    desc,
    // cargo-watch restarts the service itself; the fs watcher is only the fallback.
    cmd: useCargoWatch ? cargoWatchCommand(def) : cargoRunCommand(def),
    watch: watchEnabled && !useCargoWatch ? def.watch : [],
  };
}

const SERVICE_DEFINITIONS = {
  backend: rustService("backend", colors.cyan, "Actix-web API (http://127.0.0.1:4433)"),
  frontend: {
    name: "frontend",
    color: colors.green,
    cmd: [bunExe, "run", "--cwd", "web", "dev"],
    desc: "Next.js Web Portal (http://localhost:3000)",
    watch: [], // Next.js hot-reloads on its own, so the runner never restarts it
  },
  native: rustService("native", colors.magenta, "GPUI Native Client"),
};

let activeServices = new Set();

if (onlyMode) {
  activeServices = new Set(explicitlyIncluded);
} else if (runAll) {
  activeServices = new Set(["backend", "frontend", "native"]);
} else if (explicitlyIncluded.size > 0) {
  if (explicitlyIncluded.has("native") && !explicitlyIncluded.has("frontend")) {
    activeServices.add("backend");
    activeServices.add("native");
  } else if (explicitlyIncluded.has("frontend") && !explicitlyIncluded.has("native")) {
    activeServices.add("backend");
    activeServices.add("frontend");
  } else {
    for (const s of explicitlyIncluded) {
      activeServices.add(s);
    }
    if (!excluded.has("backend")) {
      activeServices.add("backend");
    }
  }
} else if (excluded.size > 0) {
  // Excluding services from dev:all defaults to all other services
  activeServices = new Set(["backend", "frontend", "native"]);
} else {
  // Default: backend + frontend (web)
  activeServices = new Set(["backend", "frontend"]);
}

for (const exc of excluded) {
  activeServices.delete(exc);
}

const serviceOrder = ["backend", "frontend", "native"];
const servicesToStart = serviceOrder
  .filter((key) => activeServices.has(key))
  .map((key) => SERVICE_DEFINITIONS[key]);

if (servicesToStart.length === 0) {
  console.error(`${colors.red}Error: No services selected to run.${colors.reset}`);
  console.error(`Available services: backend, frontend (web), native (app). Use --help for usage.`);
  process.exit(1);
}

function watchSummary() {
  if (!watchEnabled) return "off (--no-watch)";
  if (useCargoWatch) return "cargo-watch (only the changed service rebuilds)";
  return `built-in watcher (faster restarts: ${CARGO_WATCH_INSTALL})`;
}

console.log(
  `${colors.bold}============================================================${colors.reset}`,
);
console.log(`${colors.bold}  Umuryango Concurrent Development Runner${colors.reset}`);
for (const svc of servicesToStart) {
  console.log(`  - ${svc.color}${svc.name.padEnd(10)}${colors.reset}: ${svc.desc}`);
}
console.log(`  Watch: ${watchSummary()}`);
console.log(`  Press ${colors.yellow}Ctrl+C${colors.reset} to stop all services.`);
console.log(
  `${colors.bold}============================================================\n${colors.reset}`,
);

// ---- Process management ----

const stateByService = new Map(); // name -> { child, chain }
let stopping = false;

async function stopChild(child) {
  if (!child) return;
  child.intentionalStop = true;
  await stopTree(child);
}

function prefixStream(readable, svc, isError) {
  if (!readable) return;
  let buffer = "";
  const emit = (line) => {
    const formatted = `${svc.color}[${svc.name}]${colors.reset} ${line}`;
    if (isError) console.error(formatted);
    else console.log(formatted);
  };
  readable.setEncoding("utf8");
  readable.on("data", (chunk) => {
    buffer += chunk;
    const lines = buffer.split("\n");
    buffer = lines.pop() ?? "";
    for (const raw of lines) {
      emit(raw.endsWith("\r") ? raw.slice(0, -1) : raw);
    }
  });
  readable.on("end", () => {
    if (buffer.length > 0) {
      emit(buffer.endsWith("\r") ? buffer.slice(0, -1) : buffer);
      buffer = "";
    }
  });
}

const childEnv = { ...process.env, FORCE_COLOR: "1" };
// Keep cargo's colored output when its stdout is piped through the prefixer.
if (!rawOutput) childEnv.CARGO_TERM_COLOR = "always";

function launch(svc) {
  const child = spawnTree(svc.cmd, {
    stdio: rawOutput ? "inherit" : ["inherit", "pipe", "pipe"],
    env: childEnv,
  });
  child.intentionalStop = false;

  if (!rawOutput) {
    prefixStream(child.stdout, svc, false);
    prefixStream(child.stderr, svc, true);
  }

  child.on("exit", (code, signal) => {
    if (stopping || child.intentionalStop) return;
    const reason = code !== null ? `code ${code}` : `signal ${signal}`;
    const hint = svc.watch.length > 0 ? " (save a file to restart it)" : "";
    console.error(
      `${svc.color}[${svc.name}]${colors.reset} ${colors.red}Process exited with ${reason}${hint}${colors.reset}`,
    );
  });

  stateByService.get(svc.name).child = child;
  return child;
}

// Restarts are queued so two quick changes never run two restarts at once.
function scheduleRestart(svc, changedFile) {
  const state = stateByService.get(svc.name);
  if (!state || stopping) return;

  state.chain = state.chain.then(async () => {
    if (stopping) return;
    console.log(
      `${svc.color}[${svc.name}]${colors.reset} ${colors.yellow}change in ${changedFile}, restarting...${colors.reset}`,
    );
    await stopChild(state.child);
    if (!stopping) launch(svc);
  });
}

// ---- Fallback file watching (only when cargo-watch is not installed) ----

function startWatchers(svc) {
  if (svc.watch.length === 0) return [];

  const roots = svc.watch
    .map((p) => path.resolve(process.cwd(), p))
    .filter((p) => {
      if (existsSync(p)) return true;
      console.warn(
        `${svc.color}[${svc.name}]${colors.reset} ${colors.dim}watch path not found, skipping: ${p}${colors.reset}`,
      );
      return false;
    });

  let timer = null;
  let lastFile = "";

  return roots.map((root) =>
    fsWatch(root, { recursive: true }, (_eventType, filename) => {
      const file = filename ?? path.basename(root);
      if (isIgnoredPath(file)) return;
      lastFile = file;
      clearTimeout(timer);
      timer = setTimeout(() => scheduleRestart(svc, lastFile), DEBOUNCE_MS);
    }),
  );
}

// ---- Startup ----

for (const svc of servicesToStart) {
  stateByService.set(svc.name, { child: null, chain: Promise.resolve() });
  launch(svc);
}

const watchers = servicesToStart.flatMap((svc) => startWatchers(svc));

// ---- Shutdown ----

async function shutdown() {
  if (stopping) return;
  stopping = true;
  console.log(`\n${colors.yellow}[system] Shutting down development services...${colors.reset}`);

  for (const w of watchers) {
    try {
      w.close();
    } catch {}
  }

  await Promise.all([...stateByService.values()].map((state) => stopChild(state.child)));
  process.exit(0);
}

process.on("SIGINT", shutdown);
process.on("SIGTERM", shutdown);

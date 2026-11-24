#!/usr/bin/env bun
// Actix-web API dev runner. Uses cargo-watch for rebuild-on-change when installed.
import { runRustService } from "./lib/cargo-dev.js";

runRustService("backend", { watch: !process.argv.includes("--no-watch") });

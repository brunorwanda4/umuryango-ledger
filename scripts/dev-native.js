#!/usr/bin/env bun
// GPUI native app dev runner. Uses cargo-watch for rebuild-on-change when installed.
import { runRustService } from "./lib/cargo-dev.js";

runRustService("native", { watch: !process.argv.includes("--no-watch") });

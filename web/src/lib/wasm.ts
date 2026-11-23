/**
 * WebAssembly Loader for Umuryango Ledger Client-Side Calculations
 */

export interface BalanceBreakdownResult {
  total_cash: string;
  total_momo: string;
  total_bank: string;
  total_assets: string;
}

export async function loadWasmBridge() {
  try {
    // In production, loads the compiled wasm-bindgen output pkg
    const wasm = await import("@/../wasm-pkg/umuryango_wasm_bridge.js" as any);
    return wasm;
  } catch (err) {
    console.warn("WASM bridge module not yet built. Using JavaScript fallback.", err);
    return null;
  }
}

export function calculateBreakdownFallback(
  wallets: Array<{ wallet_type: string; balance: string }>,
): BalanceBreakdownResult {
  let cash = 0;
  let momo = 0;
  let bank = 0;

  for (const w of wallets) {
    const val = parseFloat(w.balance) || 0;
    if (w.wallet_type === "cash") cash += val;
    else if (w.wallet_type === "momo") momo += val;
    else if (w.wallet_type === "bank") bank += val;
  }

  return {
    total_cash: cash.toFixed(2),
    total_momo: momo.toFixed(2),
    total_bank: bank.toFixed(2),
    total_assets: (cash + momo + bank).toFixed(2),
  };
}

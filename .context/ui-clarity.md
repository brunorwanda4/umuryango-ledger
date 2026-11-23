# UI Clarity & "Less Is More" — Umuryango Ledger

This document establishes the interaction design rules for radical simplicity, eliminating clutter, and maximizing clarity in financial reporting.

---

## 1. The Zero-Bloat Philosophy

Financial software often suffers from decorative clutter: unnecessary illustrations, long explanatory paragraphs, redundant status icons, and placeholder buttons that lead nowhere.

In `umuryango-ledger`, we follow the **Less Is More** discipline:
1. **No Dead UI:** Every button, input, tab, and menu option must perform a concrete, working action. If a feature is not yet implemented, do not render a disabled or fake button.
2. **Remove Redundant Text:** If a label says "Umutungo Wose (Total Balance)", do not add an explanatory paragraph underneath saying "Hano urasangamo amafaranga yose umuryango ufite". Let clear numbers and labels speak for themselves.
3. **No Gratuitous Graphics:** Do not use full-page illustrations or decorative banners that push critical financial data below the fold.

---

## 2. The 5-Second Scan Test

When a user opens any screen in `umuryango-ledger`, they must instantly discern the most important state:

| Screen | Core Question Answered in 5 Seconds | Primary Visual Anchor |
| :--- | :--- | :--- |
| **Dashboard** | *What is our net balance and who holds what?* | Total Balance KPI + Custody Distribution Bar |
| **Transactions** | *What was spent recently and by whom?* | Filterable table with colored amounts and custodians |
| **Projects** | *How much is needed to complete the roof/tuition?* | Progress bar with `Raised / Target (Percentage%)` |
| **Mother's Report** | *Where did the money go and what is left in hand?* | Big Spotlight Total + In-Hand Custody Cards |

---

## 3. High Information Density with Clean Whitespace

- **Data Density:** Use compact tabular rows (`py-3 px-4`) and legible monospace or tabular numbers (`tabular-nums`) for currency amounts.
- **Visual Breathing Room:** Group related fields in rounded cards (`rounded-2xl` or `rounded-3xl`) with 16px to 24px internal padding (`p-4` to `p-6`).
- **Immediate Action Placement:** Primary actions (`+ Log Transaction`, `Handover`) must always sit at the top-right of the screen header or as a persistent floating button on mobile.

---

## 4. Distinct Visual Vocabulary: Expenses vs. Handovers

A frequent point of family conflict is misinterpreting a cash handover as an expense.
- **Expense (Outflow):** Displayed in Crimson (`text-brand-crimson`) with a negative sign (`-45,000 RWF`). Indicates funds permanently left family custody (e.g., market food).
- **Handover (Custody Transfer):** Displayed in Steel Blue (`text-brand-steel-blue`) with an exchange arrow (`↔ 100,000 RWF`). Indicates money simply moved from MoMo to Papa's wallet without diminishing family wealth.

---

## 5. Zero-Clipping & Hardware Safe Areas

Never let UI elements get cut off or obstructed by device hardware:
- Never place interactive buttons or vital totals under the system status bar, camera notch, 3-button navigation bar, or gesture pill.
- Ensure every scrollable view provides total bottom padding that clears the navigation bar and software keyboard (IME).
- Buttons and touch targets must measure at least 44x44dp on mobile to eliminate accidental misclicks.

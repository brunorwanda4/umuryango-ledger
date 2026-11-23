# Feature Spec 02: Custody Wallets & Custodian Handovers

## 1. Objective
Model family funds as physically held custody buckets (Bank, Mobile Money, Cash in Hand) and provide atomic, non-destructive handovers between custodians.

---

## 2. Custody Model

In traditional bookkeeping, accounts are abstract. In extended families, every franc is physically in someone's hands or on their phone:
- **Bank of Kigali / Equity Bank Account:** Managed by a designated account holder (e.g., Papa).
- **MTN Mobile Money Account:** Held by a designated phone holder (e.g., Mama or elder daughter).
- **Cash in Hand Envelope:** Physical banknotes held in a drawer or wallet for daily food markets.

Each wallet has:
- `id`: UUID
- `name`: Human-readable label (e.g., "Cash in Hand (Mama)")
- `wallet_type`: `BANK` | `MOMO` | `CASH`
- `balance`: `rust_decimal::Decimal` / `NUMERIC(18, 2)`
- `custodian_id`: UUID of responsible family member
- `family_id`: UUID of the family

---

## 3. Mathematical Invariant of Handovers

A custody handover moves money from one wallet to another without altering the total family net balance:

$$\Delta \text{Balance}(\text{Source}) = -A$$
$$\Delta \text{Balance}(\text{Target}) = +A$$
$$\Delta \text{Total Family Net Worth} = 0$$

### Validation Rules
1. **Source $\ne$ Target:** A wallet cannot hand over funds to itself.
2. **Same Family Tenancy:** Both wallets must belong to the exact same `family_id`.
3. **Sufficient Custody:** $\text{Balance}(\text{Source}) \ge A$. Overdraft is forbidden.
4. **Amount $> 0$:** Handover amount must be strictly positive.

---

## 4. Handover Workflow & Mutual Confirmation

1. **Initiation:** Custodian A logs a transfer of 100,000 RWF from MTN MoMo to Cash in Hand (Custodian B).
2. **Pending Acknowledgment:** The status is set to `PENDING_CONFIRMATION` or instantly applied depending on family trust settings.
3. **Acknowledgment:** Custodian B taps *"Emeza ko Wakiriye" (Acknowledge Receipt)* upon receiving the physical cash or MoMo SMS.
4. **Realtime Broadcast:** Connected family devices receive the `HandoverCompleted` event and update their local custody breakdown immediately.

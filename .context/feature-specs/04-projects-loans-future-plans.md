# Feature Spec 04: Projects, Loans & Future Plans

## 1. Objective
Support long-term family financial planning, pooling funds for major collective milestones, and maintaining a transparent record of internal and external debts.

---

## 2. Family Projects (`projects`)

Family projects allow pooling funds toward specific family goals (e.g., *House Renovation*, *School Fees*, *Land Purchase*).

### Attributes
- `id`: UUID
- `family_id`: UUID
- `title`: String (e.g., "Ubwubatsi bw'Inzu y'Ababyeyi")
- `description`: String
- `target_amount`: `Decimal` / `BIGINT`
- `collected_amount`: `Decimal` / `BIGINT`
- `status`: `ACTIVE` | `COMPLETED` | `ON_HOLD`
- `deadline`: Optional Date

### Allocation Rules
- When a family member logs income with `category = "Project: House Renovation"`, the allocated amount increments `collected_amount`.
- Progress percentage:
  $$\text{Progress} = \min\left(100, \left\lfloor \frac{\text{collected}}{\text{target}} \times 100 \right\rfloor \right)$$
- Visual progress bar displays in brand steel blue (`#669bbc`) or emerald when 100% achieved.

---

## 3. Loans & Liabilities (`loans`)

Tracks borrowed funds, mutual aid agreements, or obligations requiring repayment:

### Attributes
- `id`: UUID
- `family_id`: UUID
- `loan_type`: `EXTERNAL_DEBT` (Family owes bank or neighbor) | `INTERNAL_LOAN` (Family loaned funds to a member)
- `principal_amount`: `Decimal`
- `remaining_balance`: `Decimal`
- `counterparty_name`: String (e.g., "Umurenge SACCO" or "Aunt Jane")
- `due_date`: Date
- `is_settled`: Boolean

### Repayment Tracking
- Repayments are logged as standard transactions linking to the `loan_id`.
- Each repayment atomically reduces `remaining_balance` until zero, which marks `is_settled = true`.

---

## 4. Future Plans & Allocations (`future_plans`)

Planned expenses for upcoming months (e.g., *Fertilizer in September*, *Holiday Gathering in December*):
- Helps families budget before commitments fall due.
- Displayed as an upcoming commitments checklist in the dashboard and Mother's Report.

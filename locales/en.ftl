# Umuryango Ledger - English Localization (en)

# General & Navigation
app-name = Umuryango Ledger
tagline = Family & Personal Financial Accountability
nav-dashboard = Dashboard
nav-transactions = Transactions
nav-wallets = Wallets & Custody
nav-projects = Projects
nav-loans = Loans & Debts
nav-plans = Future Plans
nav-reports = Reports
nav-members = Family Members

# Scopes & Roles
personal-scope = Personal Finances
family-scope = Family Ledger
role-head = Family Head
role-custodian = Custodian
role-member = Contributing Member

# Ledger Terms
income = Income
expense = Expense
handover = Custodian Handover
custodian = Custodian
opening-balance = Opening Balance
closing-balance = Closing Balance
held-by = Held by
authorized-by = Authorized by
paid-to = Paid to

# Channels & Wallets
channel-bank = Bank Account
channel-momo = Mobile Money (MoMo)
channel-cash = Cash in Hand

# Report Headings & Summaries
mothers-report-title = Family Financial Summary (Mother's Report)
total-income = Total Income
total-expenses = Total Expenses
cash-custody-breakdown = Cash & Custody Breakdown
active-projects = Active Family Projects
loans-summary = Loans & Outstanding Debts
future-plans = Future Plans & Goals
report-generated-at = Generated on {$date}
report-period = Period: {$start} to {$end}

# Real-time Notifications
notif-income-added = {$actor} added {$amount} RWF to {$wallet} ({$source}).
notif-expense-logged = {$actor} spent {$amount} RWF from {$wallet} on {$category}.
notif-handover-completed = {$actor} transferred {$amount} RWF from {$from_wallet} to {$to_wallet}.
notif-project-loan-updated = {$entity_type} '{$title}' was updated.

# Domain & Localized API Errors
error-insufficient-custody = Wallet '{$wallet}' only holds {$available} RWF, which is not enough for {$requested} RWF.
error-unauthorized-role = Your role in this family does not allow performing this action.
error-invalid-handover = Handover must be between two different custody wallets within the same family.
error-scope-mismatch = Cannot mix personal wallet funds with family ledger transactions.
error-invalid-amount = Transaction amount must be greater than zero.
error-not-found = The requested record was not found.
error-validation = Validation error: {$message}

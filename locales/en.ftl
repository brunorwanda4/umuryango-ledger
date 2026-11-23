# Umuryango Ledger - English Localization (en)

# Common & Navigation
app-name = Umuryango Ledger
tagline = Family & Personal Financial Trust Ledger
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
role-custodian = Asset Custodian
role-member = Contributing Member

# Ledger & Custody Terms
income = Income
expense = Expense
handover = Custody Handover
custodian = Custodian
opening-balance = Opening Balance
closing-balance = Closing Balance
held-by = Held by
authorized-by = Authorized by
paid-to = Paid to

# Channels
channel-bank = Bank Account
channel-momo = Mobile Money (MoMo)
channel-cash = Cash in Hand

# Reports
mothers-report-title = Family Financial Summary (Mother's Report)
total-income = Total Income
total-expenses = Total Expenses
cash-custody-breakdown = Cash in Hand & Bank Breakdown
active-projects = Active Family Projects
loans-summary = Loans & Borrowings Summary
future-plans = Future Plans & Goals
report-generated-at = Generated on {$date}
report-period = Period: {$start} to {$end}

# Real-time Notifications
notif-income-added = {$actor} added {$amount} RWF to {$wallet} ({$source}).
notif-expense-logged = {$actor} spent {$amount} RWF from {$wallet} on {$category}.
notif-handover-completed = {$actor} transferred {$amount} RWF from {$from_wallet} to {$to_wallet}.
notif-project-loan-updated = {$entity_type} '{$title}' was updated.
notif-app-update-available = A new version ({$version}) is available. Click to update.
notif-app-update-mandatory = A required update ({$version}) is available. Please update now to continue.

# App Updates & OTA
update-ready-banner = New version ({$version}) is ready. Click to restart and update.
update-downloading = Downloading update ({$progress}%)...
update-mandatory-title = Required Update Available
update-restart-now = Restart & Update Now

# Authentication & Onboarding
login-title = Log In
register-title = Create Account
full-name = Full Name
phone-number = Phone Number
gender = Gender
gender-male = Male
gender-female = Female
gender-other = Other
password = Password
confirm-password = Password Again
create-account = Create Account
already-have-account = Already have an account? Log in
onboarding-title = Welcome to Umuryango Ledger
create-family = Create Family
family-name = Family Name
add-member-phone = Add Member by Phone Number
add-phone-btn = + Add Phone
join-family = Join Family
pending-family-invites = Pending Family Invites
enter-4-char-code = Enter 4-character code (e.g. HK0N)
skip-for-now = Skip for now
continue-to-dashboard = Continue to Dashboard
you-were-added-to = You were added to {$family_name}
join-btn = Join

# Domain & Localized API Errors
error-insufficient-custody = Wallet '{$wallet}' only holds {$available} RWF, which is not enough for {$requested} RWF.
error-unauthorized-role = Your role in this family does not allow performing this action.
error-invalid-handover = Handover must be between two different custody wallets within the same family.
error-scope-mismatch = Cannot mix personal wallet funds with family ledger transactions.
error-invalid-amount = Transaction amount must be greater than zero.
error-not-found = The requested record was not found.
error-validation = Validation error: {$message}
error-password-mismatch = Passwords do not match. Please enter the same password twice.
error-phone-already-exists = An account with this phone number already exists.
error-invalid-credentials = Invalid phone number or password.
error-invalid-invite-code = Invalid 4-character family code. Please check and try again.

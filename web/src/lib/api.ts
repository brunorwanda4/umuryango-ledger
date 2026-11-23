const API_BASE = process.env.NEXT_PUBLIC_API_URL || "http://localhost:4433/api/v1";

export interface LocalizedReportData {
  title: string;
  generated_date: string;
  total_assets_label: string;
  total_assets_value: string;
  cash_label: string;
  cash_value: string;
  momo_label: string;
  momo_value: string;
  bank_label: string;
  bank_value: string;
  liabilities_label: string;
  liabilities_value: string;
  mothers_headline: string;
  mothers_description: string;
}

export interface User {
  id: string;
  full_name: string;
  phone: string;
  gender: string;
  preferred_locale: string;
}

export interface Family {
  id: string;
  name: string;
  description?: string;
  currency: string;
  invite_code?: string;
}

export interface PendingFamilyInvite {
  id: string;
  family_id: string;
  family_name: string;
  invited_phone: string;
  invited_by_name: string;
  role: string;
  status: string;
  created_at: string;
}

export interface AuthResponse {
  token: string;
  user: User;
  families: Family[];
  pending_invites: PendingFamilyInvite[];
}

export interface RegisterPayload {
  full_name: string;
  phone: string;
  gender: string;
  password: string;
  confirm_password?: string;
  preferred_locale?: string;
}

export interface LoginPayload {
  phone: string;
  password: string;
}

export interface CreateFamilyPayload {
  name: string;
  description?: string;
  currency?: string;
  member_phones?: string[];
}

export interface JoinFamilyPayload {
  invite_code?: string;
  invite_id?: string;
}

export async function loginUser(
  payload: LoginPayload,
  locale: "rw" | "en" = "rw",
): Promise<AuthResponse> {
  const res = await fetch(`${API_BASE}/auth/login`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Accept-Language": locale,
    },
    body: JSON.stringify(payload),
  });

  if (!res.ok) {
    const errorData = await res.json().catch(() => ({}));
    throw new Error(errorData.message || "Failed to log in");
  }

  return res.json();
}

export async function registerUser(
  payload: RegisterPayload,
  locale: "rw" | "en" = "rw",
): Promise<AuthResponse> {
  const res = await fetch(`${API_BASE}/auth/register`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Accept-Language": locale,
    },
    body: JSON.stringify(payload),
  });

  if (!res.ok) {
    const errorData = await res.json().catch(() => ({}));
    throw new Error(errorData.message || "Failed to register account");
  }

  return res.json();
}

export async function createFamily(
  payload: CreateFamilyPayload,
  token: string,
  locale: "rw" | "en" = "rw",
): Promise<{ family: Family; invite_code: string; added_invites_count: number }> {
  const res = await fetch(`${API_BASE}/families`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
      "Accept-Language": locale,
    },
    body: JSON.stringify(payload),
  });

  if (!res.ok) {
    const errorData = await res.json().catch(() => ({}));
    throw new Error(errorData.message || "Failed to create family");
  }

  return res.json();
}

export async function joinFamily(
  payload: JoinFamilyPayload,
  token: string,
  locale: "rw" | "en" = "rw",
): Promise<{ family: Family }> {
  const res = await fetch(`${API_BASE}/families/join`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
      "Accept-Language": locale,
    },
    body: JSON.stringify(payload),
  });

  if (!res.ok) {
    const errorData = await res.json().catch(() => ({}));
    throw new Error(errorData.message || "Failed to join family");
  }

  return res.json();
}

export async function getPendingInvites(
  token: string,
  locale: "rw" | "en" = "rw",
): Promise<PendingFamilyInvite[]> {
  const res = await fetch(`${API_BASE}/families/pending-invites`, {
    headers: {
      Authorization: `Bearer ${token}`,
      "Accept-Language": locale,
    },
  });

  if (!res.ok) {
    return [];
  }

  return res.json();
}

export async function fetchZeroLoginReport(
  token: string,
  locale: "rw" | "en" = "rw",
): Promise<LocalizedReportData> {
  const res = await fetch(`${API_BASE}/reports/zero-login/${token}`, {
    headers: {
      "Accept-Language": locale,
    },
    cache: "no-store",
  });

  if (!res.ok) {
    throw new Error(`Failed to load report: ${res.statusText}`);
  }

  return res.json();
}

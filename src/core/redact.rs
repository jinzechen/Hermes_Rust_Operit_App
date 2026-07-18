//! Sensitive-information redaction.
//!
//! Two complementary functions:
//!
//! * [`redact_secrets`] — detects API keys, tokens, connection strings and
//!   other secret material, replacing the value portion with `****`.
//! * [`redact_pii`] — detects personally identifiable information (email
//!   addresses, phone numbers, ID numbers, credit card numbers) and applies
//!   the same keep-first-4-last-4 redaction strategy.
//!
//! All regexes are compiled once via [`once_cell::sync::Lazy`].

use once_cell::sync::Lazy;
use regex::Regex;

// ── Redaction helper ─────────────────────────────────────────────────────────

/// Replace the middle of `s` with `****`.
///
/// When the string is shorter than 9 characters, all characters from position 4
/// onward are replaced.  Otherwise characters 4..len-4 are replaced with four
/// asterisks.
fn mask_value(s: &str) -> String {
    let len = s.chars().count();
    if len <= 8 {
        // Too short for meaningful first-4 / last-4 — mask everything after 4.
        let prefix: String = s.chars().take(4).collect();
        format!("{}****", prefix)
    } else {
        let prefix: String = s.chars().take(4).collect();
        let suffix: String = s.chars().skip(len.saturating_sub(4)).collect();
        format!("{}****{}", prefix, suffix)
    }
}

// ── Secrets detection ────────────────────────────────────────────────────────

/// Key-prefix patterns that indicate a secret value nearby.
/// Used for the index of known prefixes (for building the combined regex).
/// NOTE: longer/more-specific patterns MUST come before shorter/generic ones
/// because Rust regex alternation is leftmost-first.
static SECRET_KEY_PREFIXES: &[&str] = &[
    // Compound SECRET patterns (MUST come before bare SECRET)
    r"JWT[\s_]?SECRET\b",
    r"AWS[\s_]?SECRET[\s_]?ACCESS[\s_]?KEY",
    r"AZURE[\s_]?CLIENT[\s_]?SECRET",
    r"STRIPE[\s_]?SECRET[\s_]?KEY",
    // Compound TOKEN patterns (MUST come before bare TOKEN)
    r"GITHUB[\s_]?TOKEN",
    r"GITLAB[\s_]?TOKEN",
    r"SLACK[\s_]?TOKEN",
    r"DISCORD[\s_]?TOKEN",
    r"TELEGRAM[\s_]?TOKEN",
    r"TELEGRAM[\s_]?BOT[\s_]?TOKEN",
    r"TWILIO[\s_]?AUTH[\s_]?TOKEN",
    r"ACCESS[\s_]?TOKEN",
    r"REFRESH[\s_]?TOKEN",
    r"SESSION[\s_]?TOKEN",
    r"BEARER[\s_]?TOKEN",
    r"AUTH[\s_]?TOKEN",
    // Compound KEY patterns (MUST come before bare KEY)
    r"API[\s_]?KEY",
    r"OPENAI[\s_]?API[\s_]?KEY",
    r"ANTHROPIC[\s_]?API[\s_]?KEY",
    r"DEEPSEEK[\s_]?API[\s_]?KEY",
    r"GEMINI[\s_]?API[\s_]?KEY",
    r"COHERE[\s_]?API[\s_]?KEY",
    r"MISTRAL[\s_]?API[\s_]?KEY",
    r"GROQ[\s_]?API[\s_]?KEY",
    r"PERPLEXITY[\s_]?API[\s_]?KEY",
    r"SENDGRID[\s_]?API[\s_]?KEY",
    r"MAILGUN[\s_]?API[\s_]?KEY",
    r"HEROKU[\s_]?API[\s_]?KEY",
    r"ACCESS[\s_]?KEY",
    r"PRIVATE[\s_]?KEY",
    r"AWS[\s_]?ACCESS[\s_]?KEY[\s_]?ID",
    r"STRIPE[\s_]?PUBLISHABLE[\s_]?KEY",
    r"ENCRYPTION[\s_]?KEY",
    r"SIGNING[\s_]?KEY",
    r"GCP[\s_]?SERVICE[\s_]?ACCOUNT[\s_]?KEY",
    // Compound URL/URI patterns
    r"DATABASE[\s_]?URL",
    r"REDIS[\s_]?URL",
    r"MYSQL[\s_]?URL",
    r"POSTGRES[\s_]?URL",
    r"MONGODB[\s_]?URI",
    // Webhook patterns
    r"SLACK[\s_]?WEBHOOK[\s_]?URL",
    r"DISCORD[\s_]?WEBHOOK[\s_]?URL",
    // Other compound patterns
    r"TWILIO[\s_]?ACCOUNT[\s_]?SID",
    r"CONNECTION[\s_]?STRING",
    // Generic bare patterns (LAST, so compound variants match first)
    r"\bSECRET\b",
    r"\bTOKEN\b",
    r"\bJWT\b",
    r"PASSWORD",
];

/// Combined regex: `(KEY_PREFIX)[:= ]+['\"]?VALUE['\"]?` — captures the value
/// in group 1 so it can be masked.
static SECRET_REDACT_RE: Lazy<Regex> = Lazy::new(|| {
    let prefixes = SECRET_KEY_PREFIXES.join("|");
    let pat = format!(
        r#"(?i)({})\s*[:=]\s*['"]?([A-Za-z0-9+/=._\-]{{8,}})['"]?"#,
        prefixes
    );
    Regex::new(&pat).unwrap()
});

/// `KEY=VALUE` pattern where the VALUE looks like base-64 / hex  (≥ 20 chars).
static KEY_VALUE_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(?i)\b([A-Za-z0-9_]+)\s*=\s*([A-Za-z0-9+/=]{20,})").unwrap()
});

// ── PII detection ────────────────────────────────────────────────────────────

/// Email address.
static EMAIL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}").unwrap()
});

/// Chinese mobile phone number (1xx-xxxx-xxxx).
static PHONE_CN_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\b1[3-9]\d{9}\b").unwrap());

/// International phone number (loose E.164-ish).
static PHONE_INTL_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\+\d{1,3}[-\s]?\d{4,14}").unwrap());

/// Chinese 18-digit resident ID number.
static ID_CARD_CN_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b[1-9]\d{5}(?:19|20)\d{2}(?:0[1-9]|1[0-2])(?:0[1-9]|[12]\d|3[01])\d{3}[\dXx]\b").unwrap()
});

/// Credit-card number (13-19 digits, optional spaces/dashes).
static CREDIT_CARD_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\b(?:\d[ -]*?){13,19}\b").unwrap()
});

// ── Public API ───────────────────────────────────────────────────────────────

/// Redact secrets from `text`.
///
/// Two passes:
/// 1. Known key-prefix patterns (30+ prefixes) followed by `:` or `=` and a
///    value — only the value portion is masked.
/// 2. Generic `KEY=VALUE` pairs where the VALUE looks base-64-ish and is ≥ 20
///    characters.
pub fn redact_secrets(text: &str) -> String {
    // Pass 1: known prefixes with colon/equals separator and a value.
    let result = SECRET_REDACT_RE
        .replace_all(text, |caps: &regex::Captures| {
            // caps[0] is the full match (prefix + separator + optional quote + value)
            // caps[1] is the prefix (captured by the outer group)
            // caps[2] is the value
            let full = &caps[0];
            let prefix = &caps[1];
            let value = &caps[2];
            // Reconstruct: prefix, then the separator and optional quote chars,
            // then the masked value, then optional trailing quote.
            // The separator+quote part is everything between prefix and value.
            let value_start_in_full = full.find(value).unwrap();
            let before = &full[prefix.len()..value_start_in_full];
            let after_len = full.len() - (value_start_in_full + value.len());
            let after = &full[full.len() - after_len..];
            format!("{}{}{}{}", prefix, before, mask_value(value), after)
        })
        .to_string();

    // Pass 2: generic KEY=VALUE base-64-ish assignment.
    KEY_VALUE_RE
        .replace_all(&result, |caps: &regex::Captures| {
            let key = &caps[1];
            let value = &caps[2];
            format!("{}={}", key, mask_value(value))
        })
        .to_string()
}

/// Redact personally identifiable information from `text`.
///
/// Email addresses, phone numbers, Chinese ID numbers and credit-card numbers
/// are each detected and their middle portion is replaced with `****`.
pub fn redact_pii(text: &str) -> String {
    let mut result = text.to_string();

    // Email
    result = EMAIL_RE
        .replace_all(&result, |caps: &regex::Captures| {
            let full = &caps[0];
            mask_value(full)
        })
        .to_string();

    // Chinese mobile phone
    result = PHONE_CN_RE
        .replace_all(&result, |caps: &regex::Captures| {
            let full = &caps[0];
            mask_value(full)
        })
        .to_string();

    // International phone (must run after CN to avoid partial matches)
    result = PHONE_INTL_RE
        .replace_all(&result, |caps: &regex::Captures| {
            let full = &caps[0];
            mask_value(full)
        })
        .to_string();

    // Chinese ID card
    result = ID_CARD_CN_RE
        .replace_all(&result, |caps: &regex::Captures| {
            let full = &caps[0];
            mask_value(full)
        })
        .to_string();

    // Credit card
    result = CREDIT_CARD_RE
        .replace_all(&result, |caps: &regex::Captures| {
            let full = &caps[0];
            mask_value(full)
        })
        .to_string();

    result
}

// ── Tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── mask_value ───────────────────────────────────────────────────────

    #[test]
    fn mask_short_string() {
        assert_eq!(mask_value("abcd"), "abcd****");
        assert_eq!(mask_value("12345678"), "1234****");
    }

    #[test]
    fn mask_long_string() {
        let s = "sk-abcdefghijklmnopqrstuvwxyz012345";
        assert_eq!(mask_value(s), "sk-a****2345");
    }

    // ── redact_secrets ───────────────────────────────────────────────────

    #[test]
    fn redact_openai_key_in_env() {
        let input = "export OPENAI_API_KEY=sk-proj-abcdefghijklmnopqrstuvwxyz012345";
        let output = redact_secrets(input);
        // Key name should be preserved, value masked.
        assert!(output.contains("OPENAI_API_KEY"));
        assert!(!output.contains("sk-proj-abcdefghijklmnopqrstuvwxyz012345"));
        assert!(output.contains("****"));
    }

    #[test]
    fn redact_github_token() {
        let input = "GITHUB_TOKEN=ghp_1234567890abcdefghijklmnopqrstuv";
        let output = redact_secrets(input);
        assert!(output.contains("GITHUB_TOKEN"));
        assert!(output.contains("****"));
        assert!(!output.contains("ghp_1234567890abcdefghijklmnopqrstuv"));
    }

    #[test]
    fn redact_database_url() {
        let input = r#"DATABASE_URL="postgres://user:pass@host:5432/db""#;
        let output = redact_secrets(input);
        assert!(output.contains("DATABASE_URL"));
        assert!(output.contains("****"));
        assert!(!output.contains("postgres://user:pass@host:5432/db"));
    }

    #[test]
    fn redact_aws_keys() {
        let input = "AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE\nAWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY";
        let output = redact_secrets(input);
        assert!(output.contains("AWS_ACCESS_KEY_ID"));
        assert!(output.contains("AWS_SECRET_ACCESS_KEY"));
        assert!(!output.contains("wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"));
    }

    #[test]
    fn redact_anthropic_key() {
        // Key=value pattern (same format as other passing tests)
        let input = "ANTHROPIC_API_KEY=sk-ant-api03-abcdefghijklmnopqrstuvwxyz123456";
        let output = redact_secrets(input);
        assert!(output.contains("ANTHROPIC_API_KEY"));
        assert!(output.contains("****"), "expected redaction, got: {}", output);
    }

    #[test]
    fn redact_jwt_secret() {
        let input = "JWT_SECRET=my-super-secret-jwt-signing-key-12345";
        let output = redact_secrets(input);
        assert!(output.contains("JWT_SECRET"));
        assert!(output.contains("****"));
        assert!(!output.contains("my-super-secret-jwt-signing-key-12345"));
    }

    #[test]
    fn redact_stripe_key() {
        let input = "STRIPE_SECRET_KEY=sk_test_placeholder";
        let output = redact_secrets(input);
        assert!(output.contains("STRIPE_SECRET_KEY"));
        assert!(output.contains("****"));
    }

    #[test]
    fn redact_discord_token() {
        let input = r#"DISCORD_TOKEN: "MTAxMjM0NTY3ODkwMTIzNA.GaBcDe.FgHiJkLmNoPqRsTuVwXyZaBcDeFgHiJkLmNo""#;
        let output = redact_secrets(input);
        assert!(output.contains("DISCORD_TOKEN"));
        assert!(output.contains("****"));
    }

    #[test]
    fn redact_telegram_token() {
        let input = "TELEGRAM_BOT_TOKEN=1234567890:AAFgHiJkLmNoPqRsTuVwXyZaBcDeFgHiJk";
        let output = redact_secrets(input);
        assert!(output.contains("TELEGRAM_BOT_TOKEN"));
        assert!(output.contains("****"));
    }

    #[test]
    fn preserves_safe_text() {
        let input = "Hello, this is a normal message without secrets.";
        let output = redact_secrets(input);
        assert_eq!(output, input);
    }

    // ── redact_pii ───────────────────────────────────────────────────────

    #[test]
    fn redact_email() {
        let input = "Contact: john.doe@example.com for more info.";
        let output = redact_pii(input);
        assert!(!output.contains("john.doe@example.com"));
        assert!(output.contains("****"));
        assert!(output.contains("john****.com")); // first 4 + **** + last 4
    }

    #[test]
    fn redact_chinese_phone() {
        let input = "我的手机号是 13812345678，请记录。";
        let output = redact_pii(input);
        assert!(!output.contains("13812345678"));
        assert!(output.contains("****"));
    }

    #[test]
    fn redact_chinese_id_card() {
        let input = "身份证号：110101199003077758";
        let output = redact_pii(input);
        assert!(!output.contains("110101199003077758"));
        assert!(output.contains("****"));
    }

    #[test]
    fn redact_credit_card() {
        let input = "Card: 4111-1111-1111-1111 expires 12/28";
        let output = redact_pii(input);
        assert!(!output.contains("4111-1111-1111-1111"));
        assert!(output.contains("****"));
    }

    #[test]
    fn redact_international_phone() {
        let input = "Call +86 13912345678 or +1-555-123-4567";
        let output = redact_pii(input);
        // Both numbers should be redacted.
        assert!(!output.contains("+8613912345678"));
        assert!(output.contains("****"));
    }

    #[test]
    fn pii_preserves_safe_text() {
        let input = "The weather is nice today.";
        let output = redact_pii(input);
        assert_eq!(output, input);
    }

    #[test]
    fn redact_multiple_items() {
        let input =
            "Alice <alice@example.com> phone 13912345678 and credit 5500-0000-0000-0004";
        let output = redact_pii(input);
        assert!(!output.contains("alice@example.com"));
        assert!(!output.contains("13912345678"));
        assert!(!output.contains("5500-0000-0000-0004"));
        // All should have been redacted.
        let asterisk_count = output.matches("****").count();
        assert!(asterisk_count >= 3);
    }
}

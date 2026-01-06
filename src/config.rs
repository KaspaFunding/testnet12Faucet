use serde::{Deserialize, Serialize};
use std::fs;

fn parse_kas_to_sompi(s: &str) -> Result<u64, String> {
    const SOMPI_PER_KAS: u64 = 100_000_000;
    let raw = s.trim();
    if raw.is_empty() {
        return Err("amount is empty".to_string());
    }

    let mut parts = raw.split('.');
    let whole = parts.next().unwrap_or("0").trim();
    let frac = parts.next();
    if parts.next().is_some() {
        return Err("invalid amount: too many decimal points".to_string());
    }

    let whole: u64 = whole
        .parse::<u64>()
        .map_err(|_| "invalid amount: whole part is not a number".to_string())?;

    let frac_str = frac.unwrap_or("").trim();
    if frac_str.len() > 8 {
        return Err("invalid amount: max 8 decimal places".to_string());
    }

    let mut frac_padded = frac_str.to_string();
    while frac_padded.len() < 8 {
        frac_padded.push('0');
    }

    let frac_value = if frac_padded.is_empty() {
        0u64
    } else {
        frac_padded
            .parse::<u64>()
            .map_err(|_| "invalid amount: fractional part is not a number".to_string())?
    };

    whole
        .checked_mul(SOMPI_PER_KAS)
        .and_then(|v| v.checked_add(frac_value))
        .ok_or_else(|| "amount overflows u64".to_string())
}

fn deserialize_amount_per_claim<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum AmountField {
        Sompi(u64),
        KasFloat(f64),
        KasString(String),
    }

    let v = AmountField::deserialize(deserializer)?;
    match v {
        AmountField::Sompi(s) => Ok(s),
        AmountField::KasFloat(f) => {
            if !f.is_finite() || f < 0.0 {
                return Err(serde::de::Error::custom(
                    "amount_per_claim must be a finite number >= 0",
                ));
            }
            let s = format!("{:.8}", f);
            parse_kas_to_sompi(&s).map_err(serde::de::Error::custom)
        }
        AmountField::KasString(s) => {
            let raw = s.trim();
            if raw.is_empty() {
                return Err(serde::de::Error::custom("amount_per_claim is empty"));
            }
            if raw.chars().any(|c| c == '.') {
                parse_kas_to_sompi(raw).map_err(serde::de::Error::custom)
            } else {
                raw.parse::<u64>().map_err(|_| {
                    serde::de::Error::custom(
                        "amount_per_claim must be a u64 sompi integer or a KAS decimal string",
                    )
                })
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub kaspad_url: String,
    pub port: u16,
    pub faucet_private_key: String,
    #[serde(deserialize_with = "deserialize_amount_per_claim")]
    pub amount_per_claim: u64,
    pub claim_interval_seconds: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            kaspad_url: "127.0.0.1:16210".to_string(),
            port: 3010,
            faucet_private_key: String::new(),
            amount_per_claim: 100_000_000, // 0.001 KAS in sompis
            claim_interval_seconds: 3600, // 1 hour
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config_path = "faucet-config.toml";
        if !std::path::Path::new(config_path).exists() {
            let default = Config::default();
            let toml = toml::to_string_pretty(&default)?;
            fs::write(config_path, toml)?;
            anyhow::bail!("Created default config at {}. Please edit and restart.", config_path);
        }

        let contents = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}

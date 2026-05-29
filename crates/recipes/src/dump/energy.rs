use crate::error::{Error, Result};

pub(super) fn parse_energy(spec: &str) -> Result<f64> {
    let trimmed = spec.trim();
    let split_at = trimmed
        .find(|c: char| c.is_ascii_alphabetic())
        .ok_or_else(|| Error::InvalidEnergy(spec.to_owned()))?;
    let (num, unit) = trimmed.split_at(split_at);
    let value: f64 = num
        .trim()
        .parse()
        .map_err(|_| Error::InvalidEnergy(spec.to_owned()))?;
    Ok(value * multiplier(unit.trim()).ok_or_else(|| Error::InvalidEnergy(spec.to_owned()))?)
}

fn multiplier(unit: &str) -> Option<f64> {
    Some(match unit {
        "W" | "J" => 1.0,
        "kW" | "kJ" => 1e3,
        "MW" | "MJ" => 1e6,
        "GW" | "GJ" => 1e9,
        "TW" | "TJ" => 1e12,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_known_units() {
        assert_eq!(parse_energy("75kW").unwrap(), 75_000.0);
        assert_eq!(parse_energy(" 1.5 MW ").unwrap(), 1_500_000.0);
        assert_eq!(parse_energy("100W").unwrap(), 100.0);
        assert_eq!(parse_energy("2GJ").unwrap(), 2e9);
    }

    #[test]
    fn rejects_unknown() {
        assert!(parse_energy("12xW").is_err());
        assert!(parse_energy("kW").is_err());
        assert!(parse_energy("12").is_err());
    }
}

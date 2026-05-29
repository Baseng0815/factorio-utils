use crate::error::{Error, Result};

pub const CURRENT_VERSION: char = '0';

pub fn split_version(s: &str) -> Result<(char, &str)> {
    let mut chars = s.chars();
    let v = chars.next().ok_or(Error::MissingVersionByte)?;
    Ok((v, chars.as_str()))
}

pub fn check_supported(v: char) -> Result<()> {
    if v == CURRENT_VERSION {
        Ok(())
    } else {
        Err(Error::UnsupportedVersion(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splits_leading_byte() {
        let (v, rest) = split_version("0abc").unwrap();
        assert_eq!(v, '0');
        assert_eq!(rest, "abc");
    }

    #[test]
    fn empty_errors() {
        assert!(matches!(split_version(""), Err(Error::MissingVersionByte)));
    }

    #[test]
    fn unsupported_version_rejected() {
        assert!(matches!(check_supported('1'), Err(Error::UnsupportedVersion('1'))));
    }
}

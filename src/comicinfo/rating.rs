//! [Rating value](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd#L63-L69)
//!

use std::{cmp::Ordering, fmt::Display, str::FromStr};

use derive_more::{Deref, Into};
use serde::{Deserialize, Serialize};

/// [Ref](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd#L63-L69)
///
/// The [`Default`] value is `0.0`.
/// ```
/// use cbz::comicinfo::rating::Rating;
///
/// let rating = Rating::default();
/// // Yes! We are using epsilon here.
/// // and we are also `dereference`-ing the rating value here to make your life "easier".
/// assert!((*rating - 0.0).abs() < f32::EPSILON);
///
/// let rating = Rating::new(1.2).unwrap();
/// assert!((*rating - 1.2).abs() < f32::EPSILON);
/// ```
#[derive(Debug, Clone, Copy, Default, PartialEq, PartialOrd, Deref, Serialize, Into)]
pub struct Rating(f32);

/// [REF](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd#L67)
pub const PRECISION: usize = 2;
/// [REF](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd#L65)
pub const MIN_VALUE: f32 = 0f32;
/// [REF](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd#L66)
pub const MAX_VALUE: f32 = 5f32;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum RatingError {
    #[error("Cannot parse float")]
    ParseFloat(#[from] std::num::ParseFloatError),
    #[error("Rating value should be between [{}, {}].", MIN_VALUE, MAX_VALUE)]
    InvalidValue,
    #[error("Comparaison impossible")]
    Cmp { a: f32, b: f32 },
}

impl Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{number:.prec$}", prec = PRECISION, number = self.0)
    }
}

impl TryFrom<f32> for Rating {
    type Error = RatingError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        let min_cmp = value.partial_cmp(&MIN_VALUE).ok_or(RatingError::Cmp {
            a: value,
            b: MIN_VALUE,
        })?;
        // dbg!(&min_cmp);
        let max_cmp = value.partial_cmp(&MAX_VALUE).ok_or(RatingError::Cmp {
            a: value,
            b: MAX_VALUE,
        })?;
        // dbg!(&max_cmp);
        if matches!(min_cmp, Ordering::Equal | Ordering::Greater)
            && matches!(max_cmp, Ordering::Equal | Ordering::Less)
        {
            Ok(Self(value))
        } else {
            Err(RatingError::InvalidValue)
        }
    }
}

impl FromStr for Rating {
    type Err = RatingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<f32>()?.try_into()
    }
}

impl<'de> Deserialize<'de> for Rating {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let maybe_val = f32::deserialize(deserializer)?;
        maybe_val.try_into().map_err(D::Error::custom)
    }
}

impl Rating {
    /// Create a new rating value.
    ///
    /// The min value is `0.0`.
    ///
    /// The max value is `5.0`.
    ///
    /// Values out of this interval return [`None`].
    ///
    pub fn new(value: f32) -> Option<Self> {
        value.try_into().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rating_eps_cmp_default() {
        let rating = Rating::default();
        assert!((*rating - 0.0).abs() < f32::EPSILON);
    }
    #[test]
    fn rating_eps_cmp() {
        let rating = Rating::new(1.2).unwrap();
        assert!((*rating - 1.2).abs() < f32::EPSILON);
    }
    #[test]
    fn rating_eps_cmp_invalid() {
        assert!(Rating::new(-1.2).is_none());
        assert!(Rating::new(6.2).is_none());
    }
    #[test]
    fn ser() -> anyhow::Result<()> {
        assert_eq!(
            serde_json::to_string(&Rating::new(2.3).unwrap())?.as_str(),
            "2.3"
        );
        Ok(())
    }
    #[test]
    fn deser() -> anyhow::Result<()> {
        let rating = serde_json::from_str::<Rating>("2.3")?;
        assert!((*rating - 2.3).abs() < f32::EPSILON);
        Ok(())
    }
    #[test]
    fn display() {
        assert_eq!(Rating::new(4.9).unwrap().to_string().as_str(), "4.90");
    }
}

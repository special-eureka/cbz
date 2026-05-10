//! [`Age Rating` enum](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd#L70-L88)
//!

use derive_more::Display;
use serde::{Deserialize, Serialize};

macro_rules! age_rating_enum {
    ($($key:ident => $value:literal,)*) => {
        #[derive(Debug, Clone, Copy, Default, Display, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
        /// [`Age Rating` enum](https://github.com/anansi-project/comicinfo/blob/db8e1d84132f97403b226f2e12aaec1342c2a223/schema/v2.0/ComicInfo.xsd#L70-L88)
        ///
        pub enum AgeRating {
            $(
                #[serde(rename = $value)]
                #[display($value)]
                $key,
            )*
            #[serde(other)]
            #[default]
            Unknown,
        }
        impl From<&str> for AgeRating {
            fn from(value: &str) -> Self {
                match value {
                    $($value => Self::$key,)*
                    _ => Self::Unknown
                }
            }
        }
        #[cfg(test)]
        mod val_tests {
            use super::AgeRating;
            use serde_json::{to_string, from_str};
            #[test]
            fn test_display() {
                $(
                    assert_eq!(AgeRating::$key.to_string().as_str(), $value);
                )*
            }
            #[test]
            fn test_from_str() {
                $(
                    assert_eq!(AgeRating::$key, $value.into());
                )*
            }
            #[test]
            fn test_ser() -> anyhow::Result<()> {
                $(
                    assert_eq!(to_string(&AgeRating::$key)?.as_str(), format!("\"{}\"", $value).as_str());
                )*
                Ok(())
            }
            #[test]
            fn test_deser() -> anyhow::Result<()> {
                $(
                    assert_eq!(AgeRating::$key, from_str(&format!("\"{}\"", $value))?);
                )*
                Ok(())
            }
        }
    };
}

age_rating_enum! {
    AdultsOnly18Plus => "Adults Only 18+",
    EarlyChildhood => "Early Childhood",
    Everyone => "Everyone",
    Everyone18Plus => "Everyone 10+",
    G => "G",
    KidsToAdults => "Kids to Adults",
    M => "M",
    MA15Plus => "MA15+",
    Mature17Plus => "Mature 17+",
    PG => "PG",
    R18Plus => "R18+",
    RatingPending => "Rating Pending",
    Teen => "Teen",
    X18Plus => "X18+",
}

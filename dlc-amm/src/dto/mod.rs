use serde;

/// Deserialize an array of float.
pub fn deserialize_float_array<'de, D>(deserializer: D) -> Result<[u8; 32], D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    if deserializer.is_human_readable() {
        let string: String = serde::de::Deserialize::deserialize(deserializer)?;
        let mut hex = [0u8; 32];
        from_hex(&string, &mut hex).map_err(serde::de::Error::custom)?;
        Ok(hex)
    } else {
        serde::de::Deserialize::deserialize(deserializer)
    }
}

fn deeserialize

#[derive(Clone, PartialEq, Debug, Eq)]
#[cfg_attr(
    any(test, feature = "serde"),
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct MarketId(u64);


#[cfg_attr(
    any(test, feature = "serde"),
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct GetPriceForPurchase {

  #[serde(deserialize_with="string_or_seq_string")]
  purchase_vector: Vec<f64>,
}


pub struct GetPriceAtThePoint {
}

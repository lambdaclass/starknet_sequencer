use cairo_felt::Felt252;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_with::{serde_as, DeserializeAs, SerializeAs};
use starknet_core::types::FieldElement;

// We need the newtype in order to be able to use it the RPC function signatures since
// jsonrpsee uses serde's deserialize implementations to deserialize params and
// Felt252's from cairo-felt ends up deserializing from the limbs instead of a hex number
#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct FeltParam(#[serde_as(as = "FeltHex")] pub Felt252);

pub struct FeltHex;
pub struct FeltHexOption;
pub struct FeltPendingBlockHash;
pub(crate) struct NumAsHex;

pub fn to_felt252(field_element: FieldElement) -> Felt252 {
    return Felt252::from_bytes_be(&field_element.to_bytes_be());
}

pub fn to_field_element(felt252: Felt252) -> FieldElement {
    return FieldElement::from_bytes_be(&felt252.to_be_bytes()).expect("Could not convert Felt252 to FieldElement");
}


impl SerializeAs<Felt252> for FeltHex {
    fn serialize_as<S>(value: &Felt252, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = value.to_str_radix(16);
        serializer.serialize_str(&format!("0x{}", &value))
    }
}

impl<'de> DeserializeAs<'de, Felt252> for FeltHex {
    fn deserialize_as<D>(deserializer: D) -> Result<Felt252, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.starts_with("0x") {
            true => Ok(Felt252::parse_bytes(value[2..].as_bytes(), 16).unwrap()),
            false => Ok(Felt252::parse_bytes(value.as_bytes(), 16).unwrap()),
        }
    }
}

impl SerializeAs<Option<Felt252>> for FeltHexOption {
    fn serialize_as<S>(value: &Option<Felt252>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => serializer.serialize_str(&value.to_str_radix(16)),
            None => serializer.serialize_none(),
        }
    }
}

impl<'de> DeserializeAs<'de, Option<Felt252>> for FeltHexOption {
    fn deserialize_as<D>(deserializer: D) -> Result<Option<Felt252>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "" => Ok(None),
            _ => Ok(Some(Felt252::from_bytes_be(value.as_bytes()))),
        }
    }
}

impl SerializeAs<Option<Felt252>> for FeltPendingBlockHash {
    fn serialize_as<S>(value: &Option<Felt252>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => serializer.serialize_str(&value.to_str_radix(16)),
            // We don't know if it's `null` or `"pending"`
            None => serializer.serialize_none(),
        }
    }
}

impl<'de> DeserializeAs<'de, Option<Felt252>> for FeltPendingBlockHash {
    fn deserialize_as<D>(deserializer: D) -> Result<Option<Felt252>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        if value.is_empty() || value == "pending" || value == "None" {
            Ok(None)
        } else {
            Ok(Some(Felt252::from_bytes_be(value.as_bytes())))
        }
    }
}

impl SerializeAs<u64> for NumAsHex {
    fn serialize_as<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{value:#x}"))
    }
}

impl<'de> DeserializeAs<'de, u64> for NumAsHex {
    fn deserialize_as<D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match u64::from_str_radix(&value[2..], 16) {
            Ok(value) => Ok(value),
            Err(err) => Err(serde::de::Error::custom(format!(
                "invalid hex string: {err}"
            ))),
        }
    }
}

pub mod base64 {
    use base64::engine::general_purpose;
    use base64::Engine;
    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = general_purpose::STANDARD.encode(v);
        String::serialize(&base64, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        general_purpose::STANDARD
            .decode(base64.as_bytes())
            .map_err(serde::de::Error::custom)
    }
}

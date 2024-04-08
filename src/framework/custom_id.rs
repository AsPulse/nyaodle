use bson::oid::ObjectId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CustomId(pub ObjectId);

impl From<CustomId> for String {
    fn from(value: CustomId) -> Self {
        format!("nyaodle-{}", value.0.to_hex())
    }
}

impl CustomId {
    pub fn as_object_id(&self) -> ObjectId {
        self.0
    }

    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }

    pub fn from_string(value: &str) -> Option<Self> {
        Some(CustomId(
            ObjectId::parse_str(value.chars().skip(8).collect::<String>()).ok()?,
        ))
    }
}

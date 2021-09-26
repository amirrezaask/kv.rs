use rocket::serde::json::Json;
use rocket::serde::ser::{SerializeStruct, Serializer};
use rocket::serde::{Deserialize, Serialize};

pub struct KVResponse<T>(String, T)
where
    T: Serialize;

impl<T> KVResponse<T>
where
    T: Serialize,
{
    pub fn new(msg: String, payload: T) -> Self {
        KVResponse(msg, payload)
    }
}
impl<T> Serialize for KVResponse<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("KVResponse", 2)?;
        s.serialize_field("message", &self.0)?;
        s.serialize_field("payload", &self.1)?;
        s.end()
    }
}
impl<T> From<Json<KVResponse>> for KVResponse<T> {}
pub type Response<T> = KVResponse<T>;

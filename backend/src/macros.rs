#[macro_export]
macro_rules! uuid_key {
    ($TypeName: ident) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            Eq,
            Hash,
            Ord,
            PartialEq,
            PartialOrd,
            serde::Deserialize,
            serde::Serialize,
        )]
        #[serde(transparent)]
        pub struct $TypeName(uuid::Uuid);

        impl $TypeName {
            pub fn new() -> Self {
                $TypeName(uuid::Uuid::new_v4())
            }

            pub fn try_from_bytes(id: &[u8]) -> Result<Self, uuid::Error> {
                Ok($TypeName(uuid::Uuid::from_slice(id)?))
            }

            pub fn parse_str(s: &str) -> Result<Self, uuid::Error> {
                Ok($TypeName(uuid::Uuid::parse_str(s)?))
            }

            pub fn inner(&self) -> uuid::Uuid {
                self.0
            }

            pub const fn from_u128(v: u128) -> Self {
                $TypeName(uuid::Uuid::from_u128(v))
            }

            pub fn as_bytes(&self) -> &[u8] {
                self.0.as_bytes().as_slice()
            }
        }

        impl PartialEq<uuid::Uuid> for $TypeName {
            fn eq(&self, other: &uuid::Uuid) -> bool {
                self.inner() == *other
            }
        }

        impl std::str::FromStr for $TypeName {
            type Err = uuid::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($TypeName(uuid::Uuid::parse_str(s)?))
            }
        }

        impl TryFrom<&str> for $TypeName {
            type Error = uuid::Error;

            fn try_from(s: &str) -> Result<Self, Self::Error> {
                $TypeName::parse_str(s)
            }
        }

        impl std::fmt::Display for $TypeName {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<uuid::Uuid> for $TypeName {
            fn from(id: uuid::Uuid) -> Self {
                $TypeName(id)
            }
        }

        impl From<&uuid::Uuid> for $TypeName {
            fn from(id: &uuid::Uuid) -> Self {
                $TypeName(*id)
            }
        }

        impl From<$TypeName> for uuid::Uuid {
            fn from(id: $TypeName) -> Self {
                id.inner()
            }
        }

        // SQLx implementations
        impl<'r> sqlx::types::Type<sqlx::Postgres> for $TypeName {
            fn type_info() -> sqlx::postgres::PgTypeInfo {
                <uuid::Uuid as sqlx::types::Type<sqlx::Postgres>>::type_info()
            }
        }

        impl<'r> sqlx::decode::Decode<'r, sqlx::Postgres> for $TypeName {
            fn decode(
                value: sqlx::postgres::PgValueRef<'r>,
            ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
                let uuid = <uuid::Uuid as sqlx::decode::Decode<sqlx::Postgres>>::decode(value)?;
                Ok($TypeName(uuid))
            }
        }

        impl<'q> sqlx::encode::Encode<'q, sqlx::Postgres> for $TypeName {
            fn encode_by_ref(
                &self,
                buf: &mut sqlx::postgres::PgArgumentBuffer,
            ) -> Result<sqlx::encode::IsNull, Box<dyn std::error::Error + 'static + Send + Sync>>
            {
                self.0.encode_by_ref(buf)
            }
        }
    };
}

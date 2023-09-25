

use super::{InvalidInput, InvalidityKind};

#[derive(Debug)]
pub struct Diet {
    pub id: i32,
    pub name: String,
    pub label: Option<String>,
}

#[derive(Debug)]
pub struct NewDiet {
    pub name: String,
    pub label: Option<String>,
}

#[derive(Debug, Default, PartialEq)]
pub struct InvalidDiet {
    pub name: Option<InvalidityKind>,
    pub label: Option<InvalidityKind>,
}

impl InvalidInput for InvalidDiet {}

impl<'r> ::sqlx::decode::Decode<'r, ::sqlx::Postgres> for Diet
where
    i32: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    i32: ::sqlx::types::Type<::sqlx::Postgres>,
    String: ::sqlx::types::Type<::sqlx::Postgres>,
    Option<String>: ::sqlx::decode::Decode<'r, ::sqlx::Postgres>,
    Option<String>: ::sqlx::types::Type<::sqlx::Postgres>,
{
    fn decode(
        value: ::sqlx::postgres::PgValueRef<'r>,
    ) -> ::std::result::Result<
        Self,
        ::std::boxed::Box<
            dyn ::std::error::Error + 'static + ::std::marker::Send + ::std::marker::Sync,
        >,
    > {
        let mut decoder = ::sqlx::postgres::types::PgRecordDecoder::new(value)?;
        let id = decoder.try_decode::<i32>()?;
        let name = decoder.try_decode::<String>()?;
        let label = decoder.try_decode::<Option<String>>()?;
        ::std::result::Result::Ok(Diet { id, name, label })
    }
}

impl ::sqlx::Type<::sqlx::Postgres> for Diet {
    fn type_info() -> ::sqlx::postgres::PgTypeInfo {
        ::sqlx::postgres::PgTypeInfo::with_name("Diet")
    }
}

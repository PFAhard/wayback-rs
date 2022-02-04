use std::{fmt, marker::PhantomData};

use serde::{
    de::{SeqAccess, Visitor},
    Deserialize, Deserializer,
};
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub(crate) struct IndColl(
    #[serde(deserialize_with = "deserialize_index_collection")]
    #[serde(rename(deserialize = "values"))]
    Vec<String>,
);

impl IndColl {
    pub(crate) fn get(self) -> Vec<String> {
        self.0
    }
}

pub(crate) fn deserialize_index_collection<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct IndCollVisitor(PhantomData<fn() -> Vec<String>>);

    impl<'de> Visitor<'de> for IndCollVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("sequence of arrays")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Vec<String>, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut cdx_api = vec![];

            while let Some(line) = seq.next_element::<Value>()? {
                let url = line.get("cdx-api").unwrap();
                cdx_api.push(serde_json::from_value::<String>(url.clone()).unwrap());
            }

            Ok(cdx_api)
        }
    }

    let visitor = IndCollVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}

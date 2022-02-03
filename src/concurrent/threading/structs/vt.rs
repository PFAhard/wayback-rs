use std::{fmt, marker::PhantomData};

use serde::{
    de::{IgnoredAny, SeqAccess, Visitor},
    Deserialize, Deserializer,
};

#[derive(Deserialize, Debug)]
pub struct VT {
    #[serde(deserialize_with = "deserialize_undetected_urls")]
    undetected_urls: Vec<String>,
    #[serde(deserialize_with = "deserialize_detected_urls")]
    detected_urls: Vec<String>,
}

impl VT {
    pub fn consume(self) -> Vec<String> {
        let det = self.detected_urls;
        let und = self.undetected_urls;
        [det, und].concat()
    }
}

#[derive(Deserialize)]
struct DetectedUrlsInner {
    url: String,
}

impl DetectedUrlsInner {
    fn consume(self) -> String {
        self.url
    }
}

#[derive(Deserialize)]
struct UndetectedUrlsInner(
    #[serde(deserialize_with = "deserialize_undetected_urls_inner")]
    #[serde(rename(deserialize = "values"))]
    String,
);

impl UndetectedUrlsInner {
    fn consume(self) -> String {
        self.0
    }
}

fn deserialize_detected_urls<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct DetectedUrlsVisitor(PhantomData<fn() -> Vec<String>>);

    impl<'de> Visitor<'de> for DetectedUrlsVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("sequence of map")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Vec<String>, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut urls = vec![];
            while let Some(url) = seq.next_element::<DetectedUrlsInner>()? {
                urls.push(url.consume());
            }
            Ok(urls)
        }
    }

    let visitor = DetectedUrlsVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}

fn deserialize_undetected_urls<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct UndetectedUrlsVisitor(PhantomData<fn() -> Vec<String>>);

    impl<'de> Visitor<'de> for UndetectedUrlsVisitor {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("sequence of arrays")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<Vec<String>, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let mut urls = vec![];
            while let Some(url) = seq.next_element::<UndetectedUrlsInner>()? {
                urls.push(url.consume());
            }
            Ok(urls)
        }
    }

    let visitor = UndetectedUrlsVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}

fn deserialize_undetected_urls_inner<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    struct UndetectedUrlsInnerVisitor(PhantomData<fn() -> String>);

    impl<'de> Visitor<'de> for UndetectedUrlsInnerVisitor {
        type Value = String;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("sequence of arrays")
        }

        fn visit_seq<S>(self, mut seq: S) -> Result<String, S::Error>
        where
            S: SeqAccess<'de>,
        {
            let url = seq.next_element::<String>()?.unwrap_or_default();
            while let Some(IgnoredAny) = seq.next_element::<IgnoredAny>()? {
                // ignore
            }

            Ok(url)
        }
    }

    let visitor = UndetectedUrlsInnerVisitor(PhantomData);
    deserializer.deserialize_seq(visitor)
}

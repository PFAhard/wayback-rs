use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_until, take_while},
    character::complete::{alphanumeric0, char, digit0, none_of, one_of},
    combinator::{cut, map, opt, recognize, value},
    error::{context, ContextError, ErrorKind, ParseError},
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};
use serde::Deserialize;
use std::collections::{HashMap, HashSet};

use crate::blocking::utils::WaybackError;

#[derive(Deserialize, Debug)]
pub(crate) struct Otx {
    url_list: HashSet<OtxMap>,
    has_next: bool,
}

#[derive(Deserialize, Debug, PartialEq, Eq, Hash)]
struct OtxMap {
    url: String,
}

impl OtxMap {
    pub(crate) fn consume(self) -> String {
        self.url
    }
}

impl Otx {
    /// Get a reference to the otx's has next.
    pub(crate) fn has_next(&self) -> bool {
        self.has_next
    }

    /// Get a reference to the otx's url list.
    pub(crate) fn url_list(self) -> HashSet<String> {
        self.url_list.into_iter().map(OtxMap::consume).collect()
    }
}

#[test]
fn otx_des_test() -> anyhow::Result<()> {
    let i = r#"[{
        "url": "https://onlyfans.com/lovalova",
        "date": "2022-02-07T03:00:50",
        "domain": "onlyfans.com",
        "hostname": "onlyfans.com",
        "result": {
            "urlworker": {
                "ip": "54.81.122.221",
                "http_code": 200
            },
            "safebrowsing": {
                "matches": []
            }
        },
        "httpcode": 200,
        "gsb": [],
        "encoded": "https%3A//onlyfans.com/lovalova"
    }, {
        "url": "https://onlyfans.com/Lauren%20burch/?ref=161881166",
        "date": "2022-02-05T20:43:43",
        "domain": "onlyfans.com",
        "hostname": "onlyfans.com",
        "result": {
            "urlworker": {
                "ip": "54.82.213.97",
                "http_code": 200
            },
            "safebrowsing": {
                "matches": []
            }
        },
        "httpcode": 200,
        "gsb": [],
        "encoded": "https%3A//onlyfans.com/Lauren%2520burch/%3Fref%3D161881166"
    }]"#;

    let o = otx_array::<'_, (_, ErrorKind)>(i)?;
    dbg!(o);

    Ok(())
}





#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    Str(String),
    Boolean(bool),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn sp<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    let chars = " \t\r\n";

    take_while(move |c| chars.contains(c))(i)
}

fn parse_str<'a, E: ParseError<&'a str>>(i: &'a str) -> IResult<&'a str, &'a str, E> {
    escaped(none_of("\"\n\\"), '\\', one_of("\"\n\\"))(i)
}

fn boolean<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, bool, E> {
    let parse_true = value(true, tag("true"));

    let parse_false = value(false, tag("false"));

    alt((parse_true, parse_false))(input)
}

fn null<'a, E: ParseError<&'a str>>(input: &'a str) -> IResult<&'a str, (), E> {
    value((), tag("null"))(input)
}

fn string<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, &'a str, E> {
    context(
        "string",
        preceded(char('\"'), cut(terminated(parse_str, char('\"')))),
    )(i)
}

fn otx_array<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<String>, E> {
    context(
        "array",
        preceded(
            char('['),
            cut(terminated(
                separated_list0(preceded(sp, tag(", ")), otx_hash),
                preceded(sp, char(']')),
            )),
        ),
    )(i)
}

fn array<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, Vec<JsonValue>, E> {
    context(
        "array",
        preceded(
            char('['),
            cut(terminated(
                separated_list0(preceded(sp, char(',')), json_value),
                preceded(sp, char(']')),
            )),
        ),
    )(i)
}

fn key_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, (&'a str, JsonValue), E> {
    separated_pair(
        preceded(sp, string),
        cut(preceded(sp, char(':'))),
        json_value,
    )(i)
}

fn otx_hash<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, String, E> {
    let mut ret = String::new();
    let (i, o) = context(
        "map",
        preceded(
            char('{'),
            cut(terminated(
                map(
                    separated_list0(preceded(sp, char(',')), key_value),
                    |tuple_vec| {
                        tuple_vec
                            .into_iter()
                            .map(|(k, v)| {
                                let v = if k == "url" {
                                    match v {
                                        JsonValue::Str(v) => {
                                            ret = v;
                                            JsonValue::Str(String::new())
                                        }
                                        _ => v,
                                    }
                                } else {
                                    v
                                };
                                (k, v)
                            })
                            .collect::<Vec<(&str, JsonValue)>>()
                    },
                ),
                preceded(sp, char('}')),
            )),
        ),
    )(i)?;
    Ok((i, ret))
}

fn hash<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, HashMap<String, JsonValue>, E> {
    context(
        "map",
        preceded(
            char('{'),
            cut(terminated(
                map(
                    separated_list0(preceded(sp, char(',')), key_value),
                    |tuple_vec| {
                        tuple_vec
                            .into_iter()
                            .map(|(k, v)| (String::from(k), v))
                            .collect()
                    },
                ),
                preceded(sp, char('}')),
            )),
        ),
    )(i)
}


fn json_value<'a, E: ParseError<&'a str> + ContextError<&'a str>>(
    i: &'a str,
) -> IResult<&'a str, JsonValue, E> {
    preceded(
        sp,
        alt((
            map(hash, JsonValue::Object),
            map(array, JsonValue::Array),
            map(string, |s| JsonValue::Str(String::from(s))),
            map(double, JsonValue::Num),
            map(boolean, JsonValue::Boolean),
            map(null, |_| JsonValue::Null),
        )),
    )(i)
}

use super::{ErrorKind, Failure, MAX_WIRE_ID};
use serde::{
    Serialize,
    de::{self, DeserializeSeed, IgnoredAny, MapAccess, Visitor},
};
use std::{borrow::Cow, fmt, io};

pub(super) enum Kind<'a> {
    Reply { id: u32, outcome: super::ReplyKind },
    Event { method: Cow<'a, str> },
    UncorrelatedError(i32),
}
pub(super) struct Envelope<'a> {
    pub session: Option<Cow<'a, str>>,
    pub kind: Kind<'a>,
}
struct Name(usize);
impl<'de> DeserializeSeed<'de> for Name {
    type Value = Cow<'de, str>;
    fn deserialize<D: de::Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        struct V(usize);
        impl<'de> Visitor<'de> for V {
            type Value = Cow<'de, str>;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("bounded string")
            }
            fn visit_borrowed_str<E: de::Error>(self, s: &'de str) -> Result<Self::Value, E> {
                if s.is_empty() || s.len() > self.0 {
                    return Err(E::custom("invalid metadata"));
                }
                Ok(Cow::Borrowed(s))
            }
            fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
                if s.is_empty() || s.len() > self.0 {
                    return Err(E::custom("invalid metadata"));
                }
                super::copy_text(s, self.0)
                    .map(Cow::Owned)
                    .map_err(|_| E::custom("metadata allocation refused"))
            }
        }
        d.deserialize_str(V(self.0))
    }
}
struct Object;
impl<'de> DeserializeSeed<'de> for Object {
    type Value = ();
    fn deserialize<D: de::Deserializer<'de>>(self, d: D) -> Result<(), D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = ();
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<(), A::Error> {
                while map.next_entry::<IgnoredAny, IgnoredAny>()?.is_some() {}
                Ok(())
            }
        }
        d.deserialize_map(V)
    }
}
struct Text;
impl<'de> DeserializeSeed<'de> for Text {
    type Value = ();
    fn deserialize<D: de::Deserializer<'de>>(self, d: D) -> Result<(), D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = ();
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("string")
            }
            fn visit_str<E: de::Error>(self, _: &str) -> Result<(), E> {
                Ok(())
            }
        }
        d.deserialize_str(V)
    }
}
struct RemoteError(usize);
impl<'de> DeserializeSeed<'de> for RemoteError {
    type Value = i32;
    fn deserialize<D: de::Deserializer<'de>>(self, d: D) -> Result<i32, D::Error> {
        struct V(usize);
        impl<'de> Visitor<'de> for V {
            type Value = i32;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("CDP error object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<i32, A::Error> {
                let (mut code, mut message, mut data) = (None, false, false);
                while let Some(name) = map.next_key_seed(Name(self.0))? {
                    match name.as_ref() {
                        "code" if code.is_none() => code = Some(map.next_value::<i32>()?),
                        "message" if !message => {
                            map.next_value_seed(Text)?;
                            message = true;
                        }
                        "data" if !data => {
                            map.next_value::<IgnoredAny>()?;
                            data = true;
                        }
                        _ => return Err(de::Error::custom("invalid CDP error fields")),
                    }
                }
                if !message {
                    return Err(de::Error::custom("missing CDP error message"));
                }
                code.ok_or_else(|| de::Error::custom("missing CDP error code"))
            }
        }
        d.deserialize_map(V(self.0))
    }
}
struct EnvelopeSeed {
    metadata: usize,
    command: bool,
}
impl<'de> DeserializeSeed<'de> for EnvelopeSeed {
    type Value = Envelope<'de>;
    fn deserialize<D: de::Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        struct V {
            metadata: usize,
            command: bool,
        }
        impl<'de> Visitor<'de> for V {
            type Value = Envelope<'de>;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("CDP envelope")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let (mut id, mut session, mut method, mut params, mut result, mut error) =
                    (None, None, None, false, false, None);
                let mut seen = 0u8;
                while let Some(field) = map.next_key_seed(Name(self.metadata))? {
                    let bit = match field.as_ref() {
                        "id" => 1,
                        "sessionId" => 2,
                        "method" => 4,
                        "params" => 8,
                        "result" => 16,
                        "error" => 32,
                        _ => return Err(de::Error::custom("unknown CDP envelope field")),
                    };
                    if seen & bit != 0 {
                        return Err(de::Error::custom("duplicate CDP envelope field"));
                    }
                    seen |= bit;
                    match bit {
                        1 => {
                            let n = map.next_value::<u64>()?;
                            if n == 0 || n > u64::from(MAX_WIRE_ID) {
                                return Err(de::Error::custom("invalid CDP integer ID"));
                            }
                            id = Some(n as u32);
                        }
                        2 => session = Some(map.next_value_seed(Name(self.metadata))?),
                        4 => method = Some(map.next_value_seed(Name(self.metadata))?),
                        8 => {
                            map.next_value_seed(Object)?;
                            params = true;
                        }
                        16 => {
                            map.next_value_seed(Object)?;
                            result = true;
                        }
                        32 => error = Some(map.next_value_seed(RemoteError(self.metadata))?),
                        _ => unreachable!("closed field vocabulary"),
                    }
                }
                if self.command {
                    if let (Some(id), Some(_)) = (id, method)
                        && params
                        && !result
                        && error.is_none()
                    {
                        return Ok(Envelope {
                            session,
                            kind: Kind::Reply {
                                id,
                                outcome: super::ReplyKind::Result,
                            },
                        });
                    }
                    return Err(de::Error::custom("invalid command shape"));
                }
                let kind = match (id, method, params, result, error) {
                    (Some(id), None, false, true, None) => Kind::Reply {
                        id,
                        outcome: super::ReplyKind::Result,
                    },
                    (Some(id), None, false, false, Some(code)) => Kind::Reply {
                        id,
                        outcome: super::ReplyKind::Error { code },
                    },
                    (None, Some(method), true, false, None) => Kind::Event { method },
                    (None, None, false, false, Some(code)) => Kind::UncorrelatedError(code),
                    _ => return Err(de::Error::custom("ambiguous CDP envelope")),
                };
                Ok(Envelope { session, kind })
            }
        }
        d.deserialize_map(V {
            metadata: self.metadata,
            command: self.command,
        })
    }
}
pub(super) fn inspect(wire: &str, metadata: usize) -> Result<Envelope<'_>, Failure> {
    parse(wire, metadata, false)
}
fn parse(wire: &str, metadata: usize, command: bool) -> Result<Envelope<'_>, Failure> {
    let mut decoder = serde_json::Deserializer::from_str(wire);
    let result = EnvelopeSeed { metadata, command }
        .deserialize(&mut decoder)
        .map_err(|_| Failure::plain(ErrorKind::Envelope))?;
    decoder
        .end()
        .map_err(|_| Failure::plain(ErrorKind::Envelope))?;
    Ok(result)
}
struct BoundedWriter {
    bytes: Vec<u8>,
    limit: usize,
}
impl io::Write for BoundedWriter {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        if b.len() > self.limit.saturating_sub(self.bytes.len()) {
            return Err(io::Error::other("CDP encoding budget"));
        }
        self.bytes.extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
pub(super) fn encode<P: Serialize>(
    id: u32,
    method: &str,
    params: &P,
    session: Option<&str>,
    limit: usize,
    metadata: usize,
) -> Result<String, Failure> {
    #[derive(Serialize)]
    struct Command<'a, P> {
        id: u32,
        method: &'a str,
        params: &'a P,
        #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
        session: Option<&'a str>,
    }
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(limit)
        .map_err(|_| Failure::plain(ErrorKind::Budget))?;
    if bytes.capacity() > limit {
        return Err(Failure::plain(ErrorKind::Budget));
    }
    let mut writer = BoundedWriter { bytes, limit };
    serde_json::to_writer(
        &mut writer,
        &Command {
            id,
            method,
            params,
            session,
        },
    )
    .map_err(|_| Failure::plain(ErrorKind::Encoding))?;
    let wire = String::from_utf8(writer.bytes).map_err(|_| Failure::plain(ErrorKind::Encoding))?;
    parse(&wire, metadata, true)?;
    Ok(wire)
}

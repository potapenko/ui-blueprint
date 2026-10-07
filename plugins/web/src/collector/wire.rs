//! Private method DTOs, not a normalized graph. Unknown protocol members are skipped.
use serde::{
    Deserialize,
    de::{self, MapAccess, Visitor},
};
use std::fmt;
use uiblueprint_schema::model::Rect;

// CDP records are objects; derived struct deserialization alone also accepts arrays.
macro_rules! object_record {
 ($name:ident { $($(#[$attr:meta])* $field:ident : $ty:ty),* $(,)? }) => {
  pub(crate) struct $name {$(pub $field:$ty),*}
  impl<'de> Deserialize<'de> for $name {
   fn deserialize<D:de::Deserializer<'de>>(d:D)->Result<Self,D::Error>{
    #[derive(Deserialize)]
    #[serde(rename_all="camelCase")]
    struct Fields{$($(#[$attr])* $field:$ty),*}
    struct Object;
    impl<'de> Visitor<'de> for Object{type Value=$name;
     fn expecting(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{f.write_str("CDP object")}
     fn visit_map<A:MapAccess<'de>>(self,a:A)->Result<$name,A::Error>{let Fields{$($field),*}=Fields::deserialize(de::value::MapAccessDeserializer::new(a))?;Ok($name{$($field),*})}
    }
    d.deserialize_map(Object)
   }
  }
 }
}
object_record!(DomRead {
 connected:bool, same_document:bool, tag:Option<String>, sensitive:bool,
 rect:Option<Rect>, input_kind:Option<String>, value:Option<String>, placeholder:Option<String>,
 required:Option<bool>, enabled:Option<bool>, readonly:Option<bool>, checked:Option<bool>,
 selected:Option<bool>, expanded:Option<bool>, focused:Option<bool>, invalid:Option<bool>,
 controls:Option<Vec<usize>>, declared_anchor:Option<usize>, active_descendant:Option<usize>
});
object_record!(AxValue { r#type:String, value:Option<Scalar> });
object_record!(AxProperty {
    name: String,
    value: AxValue
});
object_record!(AxNode { node_id:String, ignored:bool, role:Option<AxValue>, name:Option<AxValue>,
 description:Option<AxValue>, value:Option<AxValue>, properties:Option<Vec<AxProperty>>,
 #[serde(rename="backendDOMNodeId")] backend_dom_node_id:Option<u32>, frame_id:Option<String> });

pub(crate) enum Scalar {
    Text(String),
    Flag(bool),
    Number(f64),
    Other,
}
impl<'de> Deserialize<'de> for Scalar {
    fn deserialize<D: de::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Scalar;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("CDP scalar")
            }
            fn visit_str<E: de::Error>(self, s: &str) -> Result<Scalar, E> {
                Ok(Scalar::Text(s.into()))
            }
            fn visit_string<E: de::Error>(self, s: String) -> Result<Scalar, E> {
                Ok(Scalar::Text(s))
            }
            fn visit_bool<E: de::Error>(self, b: bool) -> Result<Scalar, E> {
                Ok(Scalar::Flag(b))
            }
            fn visit_f64<E: de::Error>(self, n: f64) -> Result<Scalar, E> {
                Ok(Scalar::Number(n))
            }
            fn visit_i64<E: de::Error>(self, n: i64) -> Result<Scalar, E> {
                Ok(if (n as f64) as i128 == i128::from(n) {
                    Scalar::Number(n as f64)
                } else {
                    Scalar::Other
                })
            }
            fn visit_u64<E: de::Error>(self, n: u64) -> Result<Scalar, E> {
                Ok(if (n as f64) as u128 == u128::from(n) {
                    Scalar::Number(n as f64)
                } else {
                    Scalar::Other
                })
            }
            fn visit_unit<E: de::Error>(self) -> Result<Scalar, E> {
                Ok(Scalar::Other)
            }
            fn visit_map<A: MapAccess<'de>>(self, mut a: A) -> Result<Scalar, A::Error> {
                while a.next_entry::<de::IgnoredAny, de::IgnoredAny>()?.is_some() {}
                Ok(Scalar::Other)
            }
            fn visit_seq<A: de::SeqAccess<'de>>(self, mut a: A) -> Result<Scalar, A::Error> {
                while a.next_element::<de::IgnoredAny>()?.is_some() {}
                Ok(Scalar::Other)
            }
        }
        d.deserialize_any(V)
    }
}
object_record!(TargetInfo { target_id: String });
object_record!(TargetResult {
    target_info: TargetInfo
});
object_record!(Frame {
    id: String,
    loader_id: String
});
object_record!(FrameTree { frame: Frame });
object_record!(FrameResult {
    frame_tree: FrameTree
});
object_record!(DomNode {
    backend_node_id: u32,
    node_type: u32
});
object_record!(DocumentResult { root: DomNode });
object_record!(WorldResult {
    execution_context_id: i32
});
object_record!(RemoteNode { r#type:String, subtype:Option<String>, object_id:Option<String> });
object_record!(ResolveResult { object: RemoteNode });
object_record!(ReadRemote { r#type:String, value:Option<DomRead> });
object_record!(ReadResult { result:ReadRemote, exception_details:Option<de::IgnoredAny> });
object_record!(AxResult { nodes:Vec<AxNode> });
object_record!(Empty {});

object_record!(Continuity { current: bool });
object_record!(VerifyRemote { r#type:String, value:Option<Continuity> });
object_record!(VerifyResult { result:VerifyRemote, exception_details:Option<de::IgnoredAny> });

object_record!(SelectedCall { result:RemoteNode, exception_details:Option<de::IgnoredAny> });
object_record!(PropertyValue { r#type:String, subtype:Option<String>,object_id:Option<String>,value:Option<Scalar> });
object_record!(Descriptor { name:String,value:Option<PropertyValue>,get:Option<de::IgnoredAny>,set:Option<de::IgnoredAny>,was_thrown:Option<bool>,symbol:Option<de::IgnoredAny> });
object_record!(Properties { result:Vec<Descriptor>,exception_details:Option<de::IgnoredAny> });
object_record!(Described { node: DomNode });

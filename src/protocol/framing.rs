use bytes::Bytes;

// TODO: Investigate into optimizing core types via references / lifetimes
/// Protocol frame represented by a type and associated value
///
/// The framing protocol and serialization technique is heavily inspired by
/// Redis and their [`RESP protocol`][RESP]
///
/// [RESP]: https://redis.io/docs/reference/protocol-spec/
pub enum Type {
    Simple(String),
    Error(String),
    Integer(i64),
    BulkStr(Bytes),
    Array(Vec<Type>),
}

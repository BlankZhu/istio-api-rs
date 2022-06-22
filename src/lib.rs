extern crate url;

extern crate schemars;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

extern crate k8s_openapi;
extern crate kube;

pub mod analysis;
pub mod authentication;
pub mod envoy;
pub mod extensions;
pub mod mcp;
pub mod mesh;
pub mod meta;
pub mod networking;
pub mod operator;
pub mod security;
pub mod telemetry;
pub mod r#type;

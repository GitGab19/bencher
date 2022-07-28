pub mod auth;
pub mod project;
pub mod report;
pub mod testbed;

pub use auth::{
    JsonLogin,
    JsonSignup,
    JsonUser,
};
#[cfg(not(feature = "wasm"))]
pub use project::{
    JsonNewProject,
    JsonProject,
};
pub use report::{
    JsonAdapter,
    JsonBenchmark,
    JsonBenchmarks,
    JsonLatency,
    JsonNewReport,
    JsonReport,
};
pub use testbed::JsonNewTestbed;

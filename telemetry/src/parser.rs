//! Soroban invocation log parser used by the Prometheus exporter.

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct InvocationRecord<'a> {
    #[serde(default)]
    pub contract_id: &'a str,
    #[serde(default)]
    pub cpu_instructions: u64,
    #[serde(default)]
    pub memory_bytes: u64,
    #[serde(default)]
    pub wasm_execution_duration_us: u64,
    #[serde(default)]
    pub storage_fee_stroops: u64,
    #[serde(default)]
    pub host_function: &'a str,
    #[serde(default = "default_success")]
    pub success: bool,
}

fn default_success() -> bool {
    true
}

pub fn parse_invocation_line(line: &str) -> Option<InvocationRecord<'_>> {
    serde_json::from_str(line).ok()
}

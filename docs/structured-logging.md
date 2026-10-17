# Structured Logging

The Stellar-K8s operator and every sidecar emit **structured JSON logs** to
`stdout`/`stderr` by default. Machine-readable, field-stable log lines are the
contract that aggregation, alerting, and redaction builds on (issue #1381).

## JSON log schema

Each line is a single JSON object. The exact keys depend on the emitter, but
the stable core is:

| Field | Meaning |
|---|---|
| `timestamp` | RFC 3339 timestamp of the event |
| `level` | `TRACE`/`DEBUG`/`INFO`/`WARN`/`ERROR` |
| `message` | Human-readable event message |
| `target` | The Rust module that emitted the event |
| `trace_id` / `span_id` | Active `tracing` span context (see OpenTelemetry) |
| `namespace` | Kubernetes namespace (from `K8S_NAMESPACE` env or log-field) |
| `k8s_node` | Kubernetes node (host) name (from `K8S_NODE_NAME` env) |
| `reconcile_id` | Monotonic reconcile counter |
| `correlation_id` | Request correlation ID across service boundaries |
| `*` | Additional context fields from `tracing` events (e.g. `node_type`, `phase`, `duration_ms`) |

> **Field-name contract (issue #933):** The `StructuredLog` struct serialises
> the Kubernetes namespace as `namespace` (matching [`fields::NAMESPACE`]) — an
> earlier version erroneously used `k8s_namespace`.  Log aggregation pipelines
> and alert rules **must** use `namespace`.

Example:

```json
{"timestamp":"2026-08-28T10:00:00.000000Z","level":"INFO","message":"Reconciled StellarNode","target":"stellar_k8s::controller::reconciler","node":"testnet-validator","namespace":"stellar","reconcile_id":42}
```

## Canonical field names

All call-sites **must** use the constants from `src/logging/fields.rs` instead
of bare string literals. Import the module as:

```rust
use stellar_k8s::logging::fields as F;

tracing::info!(
    { F::NODE }         = %node_name,
    { F::NAMESPACE }    = %namespace,
    { F::RECONCILE_ID } = reconcile_id,
    "Reconciliation started",
);
```

The full list of constants is documented in the `fields` module inline doc and
in the table at the top of this page.

## What emits JSON

| Binary | Logging setup | JSON by default |
|---|---|---|
| `stellar-operator` | `init_subscriber(SubscriberConfig)`; `RUST_LOG` + `--log-format` (`json`/`pretty`) | Yes |
| `stellar-webhook` | same as operator | Yes |
| `stellar-health-sidecar` | `init_binary_subscriber(Level::INFO, Json)` | Yes |
| `stellar-watcher` | `init_binary_subscriber(log_level, Json)` | Yes |
| `stellar-log-shipper` | `registry().with(fmt::layer().json())` | Yes |
| `stellar-fork-detector` | `registry().with(fmt::layer().json())` | Yes |
| `stellar-logs` | `registry().with(fmt::layer().json())` + `EnvFilter` | Yes |

`operator.logLevel` (Helm) and `RUST_LOG` (env) control verbosity; they do not
change the JSON envelope.

## Aggregation

1. **Chart log shipper (optional).** Set `logShipper.enabled: true` in
   `charts/stellar-operator/values.yaml` to deploy a Fluent Bit DaemonSet that
   tails container logs on every node and forwards them to Loki or
   Elasticsearch. It is disabled by default so the committed Helm drift goldens
   stay stable; the operator's JSON stdout works with any aggregator.
2. **External aggregators.** Loki + Promtail, Filebeat, or the OTel Collector's
   `logs` pipeline (see `values.yaml` → `otel`) consume the JSON without
   transformation.

See [Log Aggregation Guide](../docs/log-aggregation-guide.md) for reference
setups.

## Alerting on logs

`monitoring/log-alerts.yaml` ships LogQL alert rules (Loki Ruler) for
error-rate spikes, panic/fatal events, log-volume anomalies, and a downed log
shipper. Deploy them next to your Loki/Prometheus instance.

## Testing JSON log output

The test suite includes `tests/json_logging_test.rs` which verifies that
structured span fields (`node_name`, `namespace`, `reconcile_id`) appear in
the JSON output when the JSON formatter is active. Run it with:

```bash
cargo test json_log_output_contains_node_namespace_reconcile_id_fields
```

The `src/logging/mod.rs` unit tests additionally confirm that:

- `StructuredLog` serialises `namespace` (not `k8s_namespace`) for the
  Kubernetes namespace field.
- Round-trip deserialisation preserves all fields.

## Verifying locally

```bash
# Run the operator binary and inspect its log line as JSON
cargo run --bin stellar-logs -- list --help
kubectl logs -n stellar-system deploy/stellar-operator | head -1 | python3 -m json.tool
```

Redaction of sensitive fields (validator seeds etc.) is handled separately by
`src/logging/log_scrub.rs` — see [Log Redaction Policy](../docs/log-redaction-policy.md).
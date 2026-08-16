# retained event logs zero experiment

- scope
  - local event-log cleanup when `buck2.retained_event_logs` is zero
  - the number of old files skipped before the current event log is written
- exclusions
  - log ordering, concurrent deletion failures, parsing, and nonzero retention behavior
- correspondence
  - Rust and `verus/retained_event_logs.rs` use the same zero-or-subtract-one branch
  - the focused async test executes zero-retention cleanup against real temporary files
- verification
  - run `verus app/buck2_event_log/verus/retained_event_logs.rs` from the repository root

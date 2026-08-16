use vstd::prelude::*;

verus! {

/// Models the number of existing logs left untouched before the current log is written.
fn skipped_old_logs(retained_event_logs: usize) -> (skipped: usize)
    ensures
        skipped <= retained_event_logs,
        retained_event_logs == 0 ==> skipped == 0,
        retained_event_logs > 0 ==> skipped + 1 == retained_event_logs,
{
    if retained_event_logs == 0 {
        0
    } else {
        retained_event_logs - 1
    }
}

}

fn main() {}

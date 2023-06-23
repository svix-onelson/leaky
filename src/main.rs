//! Leaking on purpose
use std::collections::BTreeMap;
use std::time::{Duration, Instant};

fn handle(n: isize) -> bool {
    if n % 10 == 0 {
        println!("handling n={}", n);
        let x = Box::new([n; 2_048]);
        std::mem::forget(x);
        return true;
    }
    false
}

fn main() {
    let mut handled = BTreeMap::new();
    let mut n = 0;

    loop {
        let now = Instant::now();
        if handle(n) {
            handled.insert(now, [n; 1_024]);
        } else {
            // When the thing is not handled flip the signage, for reasons.
            handled.insert(now, [-n; 1_024]);
        }
        std::thread::sleep(Duration::from_millis(100));
        n += 1;
    }
}

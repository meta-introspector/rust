use std::time::Instant;

pub fn capture_end_time_and_duration(start_time: Instant) -> i64 {
    let end_time = Instant::now();
    end_time.duration_since(start_time).as_nanos() as i64
}
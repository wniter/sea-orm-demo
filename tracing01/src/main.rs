/**
 * tokio-rs/tracing 强大的日志框架，同时还支持OpenTelemetry格式，无缝打通未来的监控
 * https://github.com/tokio-rs/tracing
 * https://tracing.rs/tracing/
 * rust-lang/log 官方日志库，事实上的API标准, 但是三方库未必遵循
 * https://github.com/rust-lang/log
 * https://docs.rs/log/latest/log/
 *
 * 这个是固定模式
 */
use tracing::{span, Level};

fn main() {
    // Construct a new span named "my span" with trace log level.
    let span = span!(Level::TRACE, "my span");

    // Enter the span, returning a guard object.
    let _enter = span.enter();

    // Any trace events that occur before the guard is dropped will occur
    // within the span.

    // Dropping the guard will exit the span.
}

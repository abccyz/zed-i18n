use std::{fmt::Write, thread, time::Duration};

use gpui::{AsyncApp, Context, Render, WeakEntity, Window};
use human_bytes::human_bytes;
use log::warn;
use sysinfo::{ProcessesToUpdate, System};
use ui::{ButtonLike, Color, Label, LabelSize, Tooltip, h_flex, prelude::*};
use workspace::{StatusItemView, item::ItemHandle};

const REFRESH_INTERVAL: Duration = Duration::from_secs(2);
const WARN_MEMORY_RATIO: f32 = 0.70;
const CRITICAL_MEMORY_RATIO: f32 = 0.85;
const WARN_CPU_RATIO: f32 = 0.70;
const CRITICAL_CPU_RATIO: f32 = 0.90;

#[derive(Clone, Debug, Default)]
struct MetricsSnapshot {
    process_rss: u64,
    process_virtual: u64,
    system_used_memory: u64,
    system_total_memory: u64,
    process_cpu_percent: f32,
}

pub struct PerformanceMonitor {
    snapshot: Option<MetricsSnapshot>,
    peak_rss: u64,
}

impl PerformanceMonitor {
    pub fn new(cx: &mut Context<Self>) -> Self {
        cx.spawn(async move |this: WeakEntity<Self>, cx: &mut AsyncApp| {
            let pid = match sysinfo::get_current_pid() {
                Ok(pid) => pid,
                Err(err) => {
                    warn!("performance_monitor: failed to determine current pid: {err}");
                    return;
                }
            };

            let mut system = System::new_all();

            loop {
                system.refresh_memory();
                system.refresh_processes(ProcessesToUpdate::All);
                if let Some(process) = system.process(pid) {
                    let cpu_percent = normalize_process_cpu(process.cpu_usage());
                    let snapshot = MetricsSnapshot {
                        process_rss: process.memory(),
                        process_virtual: process.virtual_memory(),
                        system_used_memory: system.used_memory(),
                        system_total_memory: system.total_memory(),
                        process_cpu_percent: cpu_percent,
                    };

                    if this
                        .update(cx, |monitor, cx| {
                            monitor.peak_rss = monitor.peak_rss.max(snapshot.process_rss);
                            monitor.snapshot = Some(snapshot);
                            cx.notify();
                        })
                        .is_err()
                    {
                        break;
                    }
                } else {
                    warn!("performance_monitor: failed to locate process {pid:?}");
                }

                cx.background_executor().timer(REFRESH_INTERVAL).await;
            }
        })
        .detach();

        Self {
            snapshot: None,
            peak_rss: 0,
        }
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self {
            snapshot: None,
            peak_rss: 0,
        }
    }
}

impl Render for PerformanceMonitor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let Some(snapshot) = self.snapshot.clone() else {
            return Label::new("监控初始化中…")
                .size(LabelSize::Small)
                .color(Color::Muted)
                .into_any_element();
        };

        let memory_fraction = fraction(snapshot.process_rss, snapshot.system_total_memory);
        let cpu_fraction = (snapshot.process_cpu_percent / 100.0).clamp(0.0, 1.0);
        let memory_color =
            color_for_usage(memory_fraction, WARN_MEMORY_RATIO, CRITICAL_MEMORY_RATIO);
        let cpu_color = color_for_usage(cpu_fraction, WARN_CPU_RATIO, CRITICAL_CPU_RATIO);

        let memory_label = format!(
            "RAM {} ({})",
            format_bytes(snapshot.process_rss),
            format_percent(memory_fraction * 100.0)
        );
        let cpu_label = format!("CPU {}", format_percent(snapshot.process_cpu_percent));

        let tooltip_text = build_tooltip(&snapshot, self.peak_rss);

        ButtonLike::new("performance-monitor")
            .child(
                h_flex()
                    .gap_1()
                    .child(
                        Label::new(memory_label)
                            .size(LabelSize::Small)
                            .color(memory_color),
                    )
                    .child(
                        Label::new(cpu_label)
                            .size(LabelSize::Small)
                            .color(cpu_color),
                    ),
            )
            .tooltip(Tooltip::text(tooltip_text))
            .into_any_element()
    }
}

impl StatusItemView for PerformanceMonitor {
    fn set_active_pane_item(
        &mut self,
        _active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        cx.notify();
    }
}

fn normalize_process_cpu(raw_cpu_percent: f32) -> f32 {
    let cpu_count = thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1) as f32;
    if cpu_count == 0.0 {
        return 0.0;
    }
    (raw_cpu_percent / cpu_count).clamp(0.0, 100.0)
}

fn fraction(numerator: u64, denominator: u64) -> f32 {
    if denominator == 0 {
        0.0
    } else {
        (numerator as f32 / denominator as f32).clamp(0.0, 1.0)
    }
}

fn format_bytes(bytes: u64) -> String {
    if bytes == 0 {
        "0 B".to_string()
    } else {
        human_bytes(bytes as f64)
    }
}

fn format_percent(value: f32) -> String {
    if value >= 99.5 {
        "100%".to_string()
    } else if value >= 10.0 {
        format!("{value:.0}%")
    } else {
        format!("{value:.1}%")
    }
}

fn color_for_usage(fraction: f32, warn_threshold: f32, critical_threshold: f32) -> Color {
    if fraction >= critical_threshold {
        Color::Error
    } else if fraction >= warn_threshold {
        Color::Warning
    } else {
        Color::Muted
    }
}

fn build_tooltip(snapshot: &MetricsSnapshot, peak_rss: u64) -> String {
    let mut tooltip = String::new();
    let _ = writeln!(
        tooltip,
        "进程物理内存: {}",
        format_bytes(snapshot.process_rss)
    );
    if peak_rss > 0 {
        let _ = writeln!(tooltip, "峰值物理内存: {}", format_bytes(peak_rss));
    }
    let _ = writeln!(
        tooltip,
        "进程虚拟内存: {}",
        format_bytes(snapshot.process_virtual)
    );
    let memory_fraction = fraction(snapshot.process_rss, snapshot.system_total_memory);
    let _ = writeln!(
        tooltip,
        "系统内存: {} / {}",
        format_bytes(snapshot.system_used_memory),
        format_bytes(snapshot.system_total_memory)
    );
    let _ = write!(
        tooltip,
        "CPU 占用: {} (阈值 {:.0}%/{:.0}%)",
        format_percent(snapshot.process_cpu_percent),
        WARN_CPU_RATIO * 100.0,
        CRITICAL_CPU_RATIO * 100.0
    );
    let _ = write!(
        tooltip,
        "\nRAM 占用: {} (阈值 {:.0}%/{:.0}%)",
        format_percent(memory_fraction * 100.0),
        WARN_MEMORY_RATIO * 100.0,
        CRITICAL_MEMORY_RATIO * 100.0
    );
    tooltip
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_cpu_does_not_exceed_100() {
        assert_eq!(normalize_process_cpu(0.0), 0.0);
        assert!(normalize_process_cpu(100.0) <= 100.0);
        assert!(normalize_process_cpu(400.0) <= 100.0);
    }

    #[test]
    fn format_percent_formats_values() {
        assert_eq!(format_percent(0.0), "0.0%");
        assert_eq!(format_percent(9.49), "9.5%");
        assert_eq!(format_percent(10.0), "10%");
        assert_eq!(format_percent(99.6), "100%");
    }

    #[test]
    fn fraction_handles_zero_denominator() {
        assert_eq!(fraction(0, 0), 0.0);
        assert_eq!(fraction(5, 0), 0.0);
        assert!((fraction(5, 10) - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn color_for_usage_respects_thresholds() {
        assert_eq!(
            color_for_usage(0.5, WARN_MEMORY_RATIO, CRITICAL_MEMORY_RATIO),
            Color::Muted
        );
        assert_eq!(
            color_for_usage(
                WARN_MEMORY_RATIO + 0.01,
                WARN_MEMORY_RATIO,
                CRITICAL_MEMORY_RATIO
            ),
            Color::Warning
        );
        assert_eq!(
            color_for_usage(
                CRITICAL_MEMORY_RATIO + 0.01,
                WARN_MEMORY_RATIO,
                CRITICAL_MEMORY_RATIO
            ),
            Color::Error
        );
    }
}

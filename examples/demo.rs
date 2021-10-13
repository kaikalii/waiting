use std::{thread::sleep, time::Duration};

use waiting::*;

fn main() {
    const SLEEP_DUR: Duration = Duration::from_millis(10);

    for _ in (0..150)
        .progress()
        .title("Fraction")
        .text_style(ProgressStyle::Fraction)
    {
        sleep(SLEEP_DUR);
    }

    for _ in (0..150)
        .progress()
        .title("Percent with Custom Bar")
        .text_style(ProgressStyle::Percent)
        .bar_style(BarStyle {
            text: "--==##==".into(),
            left_end: "[".into(),
            right_end: "]".into(),
            ..Default::default()
        })
    {
        sleep(SLEEP_DUR);
    }

    let mut i = 0;
    for _ in (0..).progress().title("Unknown Size").with_elapsed() {
        i += 1;
        sleep(SLEEP_DUR);
        if i == 300 {
            break;
        }
    }

    let mut i = 0;
    for _ in (0..)
        .progress()
        .title("Unknown Size Smooth")
        .bar_style(BarStyle {
            slide: SlideStyle::Smooth,
            ..Default::default()
        })
        .with_elapsed()
    {
        i += 1;
        sleep(SLEEP_DUR);
        if i == 400 {
            break;
        }
    }

    let mut i = 0;
    for _ in (0..)
        .progress()
        .title("Unknown Size with WAVE")
        .with_elapsed()
        .bar_style(BarStyle {
            text: "▁▁▂▂▃▄▅▆▇▇███▇▇▆▅▄▃▂▂▁".into(),
            rotation_speed: -30.0,
            slide_ratio: 1.0,
            ..Default::default()
        })
    {
        i += 1;
        sleep(SLEEP_DUR);
        if i == 300 {
            break;
        }
    }
}

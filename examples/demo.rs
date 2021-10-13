use std::{thread::sleep, time::Duration};

use waiting::*;

fn main() {
    const SLEEP_DUR: Duration = Duration::from_millis(10);

    for _ in (0..200)
        .progress()
        .title("Fraction")
        .text_style(ProgressStyle::Fraction)
    {
        sleep(SLEEP_DUR);
    }

    for _ in (0..200)
        .progress()
        .title("Percent")
        .text_style(ProgressStyle::Percent)
    {
        sleep(SLEEP_DUR);
    }

    for _ in (0..300)
        .progress()
        .title("Custom Bar")
        .text_style(ProgressStyle::Bare)
        .bar_style(BarStyle {
            rotation_speed: 20.0,
            ..Default::default()
        })
        .bar("____/‾‾‾‾\\")
        .bar_ends("[start: ", ":done]")
    {
        sleep(SLEEP_DUR);
    }

    let mut i = 0;
    for _ in (0..)
        .progress()
        .title("Unknown Size")
        .bar("♠♥♣♦")
        .bar_ends("[", "]")
        .with_elapsed()
    {
        i += 1;
        sleep(SLEEP_DUR);
        if i == 500 {
            break;
        }
    }
}

use std::{
    io::{stdout, Write},
    time::Instant,
};

use unicode_width::{UnicodeWidthChar, UnicodeWidthStr};

#[derive(Debug, Clone)]
pub struct Progress<I> {
    iter: I,
    progress: usize,
    text_style: ProgressStyle,
    bar_style: BarStyle,
    title: String,
    max_width: usize,
    start_time: Option<Instant>,
    print_elapsed: bool,
    clear_on_end: bool,
}

impl<I> Progress<I>
where
    I: Iterator,
{
    pub fn new(iter: I) -> Self {
        let prog = Progress {
            iter,
            progress: 0,
            text_style: ProgressStyle::default(),
            bar_style: BarStyle::default(),
            title: String::new(),
            max_width: usize::MAX,
            start_time: None,
            clear_on_end: true,
            print_elapsed: false,
        };
        prog.print_progress();
        prog
    }
    pub fn title<S>(mut self, title: S) -> Self
    where
        S: Into<String>,
    {
        self.title = title.into();
        self
    }
    pub fn max_width(mut self, max_width: usize) -> Self {
        self.max_width = max_width;
        self
    }
    pub fn text_style(mut self, style: ProgressStyle) -> Self {
        self.text_style = style;
        self
    }
    pub fn bar_style(mut self, style: BarStyle) -> Self {
        self.bar_style = style;
        self
    }
    pub fn no_clear_on_end(mut self) -> Self {
        self.clear_on_end = false;
        self
    }
    pub fn with_elapsed(mut self) -> Self {
        self.print_elapsed = true;
        self
    }
    fn print_progress(&self) {
        let elapsed = if let Some(start) = self.start_time {
            (Instant::now() - start).as_secs_f32()
        } else {
            0.0
        };
        let time = if self.print_elapsed {
            let seconds = elapsed;
            if seconds < 1.0 {
                format!(" | {:.0}ms", seconds * 1000.0)
            } else if seconds < 10.0 {
                format!(" | {:.2}s", seconds)
            } else if seconds < 60.0 {
                format!(" | {:.0}s", seconds)
            } else {
                let minutes = seconds / 60.0;
                let seconds = seconds % 60.0;
                if minutes < 60.0 {
                    format!(" | {:.0}m {:.0}s", minutes, seconds)
                } else {
                    let hours = minutes / 60.0;
                    let minutes = minutes % 60.0;
                    if hours < 24.0 {
                        format!(" | {:.0}h {:.0}m {:.0}s", hours, minutes, seconds)
                    } else {
                        let days = hours / 24.0;
                        let hours = hours % 24.0;
                        format!(
                            " | {:.2}d {:.0}h {:.0}m {:.0}s",
                            days, hours, minutes, seconds
                        )
                    }
                }
            }
        } else {
            String::new()
        };
        print!("\r");
        let max = self.iter.size_hint().1.map(|max| max + self.progress);
        let title_suffix = match (self.text_style, max) {
            (ProgressStyle::Bare, _) => String::new(),
            (ProgressStyle::Percent, Some(max)) => {
                format!("{:.2}% ", (self.progress as f32 / max as f32) * 100.0,)
            }
            (ProgressStyle::Fraction, Some(max)) => {
                format!("{}/{} ", self.progress, max)
            }
            (_, None) => format!("{}", self.progress),
        };
        let title = format!(
            "{}{}{}{} ",
            self.title,
            if self.title.is_empty() { "" } else { " " },
            title_suffix,
            time
        );
        let width = term_size::dimensions()
            .map_or(40, |(w, _)| w)
            .saturating_sub(
                title.width() + self.bar_style.left_end.width() + self.bar_style.right_end.width(),
            )
            .min(self.max_width);
        print!("{}{}", title, self.bar_style.left_end);
        let chars_count = self.bar_style.text.chars().count() as isize;
        let avg_bar_char_width = self
            .bar_style
            .text
            .chars()
            .map(|c| c.width().unwrap_or(1) as f32)
            .sum::<f32>()
            / chars_count as f32;
        let rotation = (elapsed * self.bar_style.rotation_speed).round() as isize;
        if let Some(max) = max {
            let scaled_progress =
                ((self.progress as f32 / (max as f32).max(1.0)) * width as f32).round() as usize;
            let progress_width = (scaled_progress as f32 / avg_bar_char_width).round() as usize;
            for i in 0..progress_width {
                print!(
                    "{}",
                    self.bar_style
                        .text
                        .chars()
                        .nth(modulus(i as isize + rotation, chars_count) as usize)
                        .unwrap()
                );
            }
            for _ in 0..width - progress_width {
                print!(" ");
            }
        } else {
            let bar_frac = self.bar_style.slide_ratio.min(1.0);
            let mut bar_width = (width as f32 * bar_frac).round().max(1.0) as usize;
            let offset = (match self.bar_style.slide {
                SlideStyle::Smooth => -(elapsed * self.bar_style.slide_speed).cos() * 0.5 + 0.5,
                SlideStyle::Linear => {
                    let t = elapsed;
                    let f = self.bar_style.slide_speed / std::f32::consts::PI;
                    2.0 * (t * f - (t * f + 0.5).floor()).abs()
                }
                SlideStyle::Wrapping => {
                    let period =
                        std::f32::consts::PI / self.bar_style.slide_speed * (1.0 - bar_frac);
                    (elapsed % period) / period / (1.0 - bar_frac)
                }
            } * width as f32
                * (1.0 - bar_frac)) as usize;
            let full_bar_width = bar_width;
            bar_width = bar_width.min(width - offset);
            let wrapped_bar_width = full_bar_width - bar_width;
            for i in 0..(wrapped_bar_width as f32 / avg_bar_char_width).round() as usize {
                print!(
                    "{}",
                    self.bar_style
                        .text
                        .chars()
                        .nth(modulus(i as isize + rotation, chars_count) as usize)
                        .unwrap()
                );
            }
            for _ in 0..offset - wrapped_bar_width {
                print!(" ");
            }
            for i in 0..(bar_width as f32 / avg_bar_char_width).round() as usize {
                print!(
                    "{}",
                    self.bar_style
                        .text
                        .chars()
                        .nth(modulus(i as isize + rotation + offset as isize, chars_count) as usize)
                        .unwrap()
                );
            }
            for _ in 0..width - bar_width - offset {
                print!(" ");
            }
        }
        print!("{}", self.bar_style.right_end);
        let _ = stdout().flush();
    }
}

impl<I> Drop for Progress<I> {
    fn drop(&mut self) {
        let width = term_size::dimensions().map_or(40, |(w, _)| w);
        print!("\r");
        for _ in 0..width {
            print!(" ");
        }
        print!("\r");
        let _ = stdout().flush();
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProgressStyle {
    Bare,
    Percent,
    Fraction,
}

impl Default for ProgressStyle {
    fn default() -> Self {
        ProgressStyle::Percent
    }
}

#[derive(Debug, Clone)]
pub struct BarStyle {
    pub text: String,
    pub slide: SlideStyle,
    pub slide_speed: f32,
    pub rotation_speed: f32,
    pub slide_ratio: f32,
    pub left_end: String,
    pub right_end: String,
}

impl Default for BarStyle {
    fn default() -> Self {
        BarStyle {
            text: '█'.into(),
            slide: SlideStyle::default(),
            slide_speed: 1.0,
            rotation_speed: 0.0,
            slide_ratio: 1.0 / 6.0,
            left_end: String::new(),
            right_end: String::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SlideStyle {
    Wrapping,
    Linear,
    Smooth,
}

impl Default for SlideStyle {
    fn default() -> Self {
        SlideStyle::Wrapping
    }
}

impl<I> Iterator for Progress<I>
where
    I: Iterator,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        self.start_time.get_or_insert(Instant::now());
        let item = if let Some(item) = self.iter.next() {
            self.progress += 1;
            Some(item)
        } else {
            None
        };
        self.print_progress();
        item
    }
}

pub trait ToProgress<I> {
    fn progress(self) -> Progress<I>;
}

impl<I> ToProgress<I> for I
where
    I: Iterator,
{
    fn progress(self) -> Progress<I> {
        Progress::new(self)
    }
}

fn modulus(x: isize, m: isize) -> isize {
    (x % m + m) % m
}

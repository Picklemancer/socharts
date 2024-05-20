use crate::styles::text_color;

#[derive(Default)]
pub struct Config {
    width: u32,
    height: u32,
    offset: u32,
    colors: Vec<String>,
    caption: String,
}

impl Config {
    // pub fn with_caption(mut self, caption: String) -> Self {
    //     self.caption = caption;
    //     self
    // }

    pub fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    // pub fn with_offset(mut self, offset: u32) -> Self {
    //     self.offset = offset;
    //     self
    // }

    pub fn with_colors(mut self, colors: Vec<String>) -> Self {
        self.colors = colors;
        self
    }
}

// pub fn plot(series: Vec<f64>, config: Config) -> String {
//     plot_many(vec![series], config)
// }

pub fn plot_many(mut series: Vec<Vec<f64>>, mut config: Config) -> String {
    let mut len_max = series.iter().map(|s| s.len()).max().unwrap_or(0);
    if config.width > 0 {
        series.iter_mut().for_each(|s| {
            if s.len() < len_max {
                s.extend(vec![f64::NAN].repeat(len_max - s.len()))
            }
            *s = interpolate(s, config.width);
        });
        len_max = config.width as usize;
    }

    let mut min = f64::MAX;
    let mut max = f64::MIN;

    (min, max) = series.iter().map(|s| min_max(s)).fold(
        (min, max),
        |(current_min, current_max), (next_min, next_max)| {
            (
                f64::min(next_min, current_min),
                f64::max(next_max, current_max),
            )
        },
    );

    let interval = (max - min).abs();
    if config.height == 0 {
        if interval == 0f64 {
            config.height = 3;
        } else if interval <= 1f64 {
            config.height =
                (interval * f64::from(10i32.pow((-interval.log10()).ceil() as u32))) as u32;
        } else {
            config.height = interval as u32;
        }
    }

    if config.offset == 0 {
        config.offset = 3;
    }

    let ratio = if interval != 0f64 {
        f64::from(config.height) / interval
    } else {
        1f64
    };

    let min2 = (min * ratio).round();
    let max2 = (max * ratio).round();

    let int_min2 = min2 as i32;
    let int_max2 = max2 as i32;

    let rows = f64::from(int_max2 - int_min2).abs() as i32;
    let width = len_max as u32 + config.offset;

    let mut plot: Vec<Vec<String>> = Vec::new();

    for _i in 0..=rows {
        let mut line = Vec::<String>::new();
        for _j in 0..width {
            line.push(" ".to_string());
        }
        plot.push(line);
    }

    let mut precision = 2;
    let log_maximum = if min == 0f64 && max == 0f64 {
        -1f64
    } else {
        f64::max(max.abs(), min.abs()).log10()
    };

    if log_maximum < 0f64 {
        if log_maximum % 1f64 != 0f64 {
            precision += log_maximum.abs() as i32;
        } else {
            precision += (log_maximum.abs() - 1f64) as i32;
        }
    } else if log_maximum > 2f64 {
        precision = 0;
    }

    let max_number_label_length = format!("{:.*}", precision as usize, max).len();
    let min_number_label_length = format!("{:.*}", precision as usize, min).len();

    let max_label_width = usize::max(max_number_label_length, min_number_label_length);

    for y in int_min2..=int_max2 {
        let magnitude = if rows > 0 {
            max - f64::from(y - int_min2) * interval / f64::from(rows)
        } else {
            f64::from(y)
        };
        let label = format!(
            "{number:LW$.PREC$}",
            LW = max_label_width + 1,
            PREC = precision as usize,
            number = magnitude
        );
        let w = (y - int_min2) as usize;
        let h = f64::max(f64::from(config.offset) - label.len() as f64, 0f64) as usize;
        plot[w][h] = label;
        plot[w][(config.offset - 1) as usize] = "┤".to_string();
    }

    for (i, series_inner) in series.iter().enumerate() {
        let mut y0;
        let mut y1;
        let color = &config.colors[i];

        if !series_inner[0].is_nan() {
            y0 = ((series_inner[0] * ratio).round() - min2) as i32;
            plot[(rows - y0) as usize][(config.offset - 1) as usize] = text_color("┼", &color);
        }

        for x in 0..series_inner.len() - 1 {
            if series_inner[x].is_nan() && series_inner[x + 1].is_nan() {
                continue;
            }
            if series_inner[x + 1].is_nan() && !series_inner[x].is_nan() {
                y0 = ((series_inner[x] * ratio).round() - f64::from(int_min2)) as i32;
                plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] =
                    text_color("─", &color);
                continue;
            }
            if series_inner[x].is_nan() && !series_inner[x + 1].is_nan() {
                y1 = ((series_inner[x + 1] * ratio).round() - f64::from(int_min2)) as i32;
                plot[(rows - y1) as usize][(x as u32 + config.offset) as usize] =
                    text_color("─", &color);
                continue;
            }
            y0 = ((series_inner[x] * ratio).round() - f64::from(int_min2)) as i32;
            y1 = ((series_inner[x + 1] * ratio).round() - f64::from(int_min2)) as i32;

            if y0 == y1 {
                plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] =
                    text_color("─", &color);
            } else {
                if y0 > y1 {
                    plot[(rows - y1) as usize][(x as u32 + config.offset) as usize] =
                        text_color("╰", &color);
                    plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] =
                        text_color("╮", &color);
                } else {
                    plot[(rows - y1) as usize][(x as u32 + config.offset) as usize] =
                        text_color("╭", &color);
                    plot[(rows - y0) as usize][(x as u32 + config.offset) as usize] =
                        text_color("╯", &color);
                }

                let start = f64::min(f64::from(y0), f64::from(y1)) as i32 + 1;
                let end = f64::max(f64::from(y0), f64::from(y1)) as i32;

                for y in start..end {
                    plot[(rows - y) as usize][(x as u32 + config.offset) as usize] =
                        text_color("│", &color);
                }
            }
        }
    }

    let mut res: String = plot
        .into_iter()
        .map(|line| line.join(""))
        .collect::<Vec<String>>()
        .join("\n");
    res.pop();
    if !config.caption.is_empty() {
        res.push('\n');
        res.push_str(
            std::iter::repeat(" ")
                .take(config.offset as usize + max_label_width + 2)
                .collect::<String>()
                .as_ref(),
        );
        res.push_str(config.caption.as_ref());
    }
    res
}

fn interpolate(series: &[f64], count: u32) -> Vec<f64> {
    let mut result = Vec::new();
    let spring_factor = (series.len() - 1) as f64 / f64::from(count - 1);
    result.push(series[0]);
    for i in 1..count - 1 {
        let spring = f64::from(i) * spring_factor;
        let before = spring.floor();
        let after = spring.ceil();
        let at_point = spring - before;
        result.push(linear_interpolate(
            series[before as usize],
            series[after as usize],
            at_point,
        ))
    }
    result.push(series[series.len() - 1]);
    result
}

fn linear_interpolate(before: f64, after: f64, at_point: f64) -> f64 {
    before + (after - before) * at_point
}

fn min_max(series: &[f64]) -> (f64, f64) {
    let min = series
        .iter()
        .fold(std::f64::MAX, |accu, &x| if x < accu { x } else { accu });
    let max = series
        .iter()
        .fold(std::f64::MIN, |accu, &x| if x > accu { x } else { accu });
    (min, max)
}

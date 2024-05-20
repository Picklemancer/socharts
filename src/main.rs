mod config;
mod properties;
mod rasciigraph;
mod styles;
mod utils;

// https://github.com/26tanishabanik/ascii_graph_rust
// https://github.com/loony-bean/textplots-rs
// https://github.com/ratatui-org/ratatui
// https://ratatui.rs/tutorials/hello-world/

struct Property<'a> {
    label: &'a str,
    get_value: properties::GetValue,
    values: Vec<f64>,
    color: &'a str,
    // quizas u32?
    max: f64,
    min: f64,
    sum: f64,
}

struct Graph<'a> {
    label: &'a str,
    series: Vec<Property<'a>>,
}

fn main() {
    println!("> Booting...");

    let mut samples = 0;
    let x_margin = 8;
    let y_margin = 2;

    let config = config::get_config();
    let duration = (config.sample_duration * 1000.0) as u32;
    let os_version = utils::get_os_version();
    let (width, _) = utils::get_terminal_size();

    let mut graphs = vec![];

    for graph in config.graphs.iter() {
        let mut current_graph = Graph {
            label: &graph.label,
            series: vec![],
        };

        for property in graph.properties.iter() {
            let get_value = match properties::get_property(&os_version, &property.key) {
                Some(value) => value,
                None => continue,
            };

            current_graph.series.push(Property {
                label: &property.label,
                get_value,
                color: &property.color,
                values: vec![0.0; width],
                min: f64::MAX,
                max: f64::MIN,
                sum: 0.0,
            })
        }

        graphs.push(current_graph);
    }

    println!("> Starting...");

    loop {
        let (cli_width, cli_height) = utils::get_terminal_size();
        let width = cli_width - x_margin;
        let graph_height = (cli_height - y_margin) / graphs.len();

        // sleep(Duration::from_secs(2));
        let result = match utils::get_powermetrics_value(duration) {
            Ok(value) => value,
            Err(_) => continue,
        };

        samples += 1;

        for graph in graphs.iter_mut() {
            let mut height = graph_height;

            let mut series: Vec<Vec<f64>> = vec![];
            let mut colors: Vec<String> = vec![];

            let mut line = vec![
                format!("{:^cli_width$}", graph.label),
                // "Property\tNow\tMin\tAvg\tMax".to_string(),
            ];

            for property in graph.series.iter_mut() {
                property.values.push(
                    // rand::thread_rng().gen_range(150.0..3000.0)
                    (property.get_value)(&result, config.sample_duration),
                );
            }

            graph.series.sort_by(|a, b| {
                b.values
                    .last()
                    .unwrap()
                    .partial_cmp(a.values.last().unwrap())
                    .unwrap()
            });

            for property in graph.series.iter_mut() {
                let value = property.values.last().unwrap();

                property.sum += *value;
                if *value < property.min {
                    property.min = *value
                }
                if *value > property.max {
                    property.max = *value
                }

                colors.push(property.color.to_string());

                series.push(
                    property
                        .values
                        .as_slice()
                        .get(property.values.len() - width..)
                        .unwrap()
                        .to_vec(),
                );

                line.push(format!(
                    "{:^width$}",
                    format!(
                        "{:<20}{:^6}{:^6}{:^6}{:^6}",
                        styles::text_color(property.label, property.color),
                        // pensar si mostrar algun decimal
                        // ahora mismo no se muestra ninguno
                        format!("{:.0}", value),
                        format!("{:.0}", property.min),
                        format!("{:.0}", (property.sum as u32 / samples)),
                        format!("{:.0}", property.max),
                    ),
                    width = cli_width + 10
                ));
            }

            height -= line.len();

            if height > 0 {
                line.insert(
                    0,
                    rasciigraph::plot_many(
                        series,
                        rasciigraph::Config::default()
                            .with_width(width as u32)
                            .with_height(height as u32)
                            .with_colors(colors),
                    ),
                );
            }

            println!("{}", line.join("\n"));
        }
    }
}

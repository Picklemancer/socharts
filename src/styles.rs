use std::collections::HashMap;

pub fn text_color(text: &str, color: &str) -> String {
    let text_colors = HashMap::from([
        ("default", [39, 39]),
        ("black", [30, 39]),
        ("red", [31, 39]),
        ("green", [32, 39]),
        ("yellow", [33, 39]),
        ("blue", [34, 39]),
        ("magenta", [35, 39]),
        ("cyan", [36, 39]),
        ("light_gray", [37, 39]),
        ("crimson", [38, 39]),
        ("dark_gray", [90, 39]),
        ("light_red", [91, 39]),
        ("light_green", [92, 39]),
        ("light_yellow", [93, 39]),
        ("light_blue", [94, 39]),
        ("light_magenta", [95, 39]),
        ("light_cyan", [96, 39]),
        ("white", [97, 39]),
    ]);
    let clr = text_colors
        .get(&color)
        .expect(&format!("{} missing", color));
    format!("\x1b[{}m{}\x1b[{}m", clr[0], text, clr[1])
}

// pub fn bg_color(text: &str, color: &str) -> String {
//     let bg_colors = HashMap::from([
//         ("default", [49, 49]),
//         ("black", [40, 49]),
//         ("red", [41, 49]),
//         ("green", [42, 49]),
//         ("yellow", [43, 49]),
//         ("blue", [44, 49]),
//         ("magenta", [45, 49]),
//         ("cyan", [46, 49]),
//         ("light_gray", [47, 49]),
//         ("crimson", [48, 49]),
//         ("dark_gray", [100, 49]),
//         ("light_red", [101, 49]),
//         ("light_green", [102, 49]),
//         ("light_yellow", [103, 49]),
//         ("light_blue", [104, 49]),
//         ("light_magenta", [105, 49]),
//         ("light_cyan", [106, 49]),
//         ("white", [107, 49]),
//     ]);
//     let clr = bg_colors.get(&color).expect(&format!("{} missing", color));
//     format!("\x1b[{}m{}\x1b[{}m", clr[0], text, clr[1])
// }

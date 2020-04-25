use badge::{Badge, BadgeOptions};
use clap::{crate_version, App, Arg};
use scraper::{Html, Selector};
use std::fs::File;
use std::io::prelude::*;
use std::io::Read;

fn main() {
    // CLI arguments handling
    let matches = App::new("lcov_badge_generator")
        .version(crate_version!())
        .about("Parse html output from gcov/lcov/genhtml to generate a coverage badge.")
        .arg(
            Arg::with_name("html_file")
                .help("The generated index.html")
                .required(true),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("OUTPUT_FILE")
                .help("The output file")
                .default_value("badge.svg")
                .takes_value(true),
        )
        .get_matches();
    // Read the HTML file
    let mut file = File::open(matches.value_of("html_file").unwrap()).expect("file not found");
    let mut html_content = String::new();
    file.read_to_string(&mut html_content)
        .expect("something went wrong reading the file");
    let document = Html::parse_document(&html_content);
    // Parse the html to find the element with selector
    let selector = Selector::parse("body > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(3) > td:nth-child(1) > table:nth-child(1) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(7)").unwrap();
    let lines_coverage_element = document.select(&selector).next().unwrap();
    let lines_coverage = lines_coverage_element.inner_html();
    // Generate the badge
    let badge = Badge::new(BadgeOptions {
        subject: "coverage".to_owned(),
        status: lines_coverage,
        color: "#4c1".to_owned(),
    })
    .unwrap();
    // Write the svg file
    let mut badge_file =
        File::create(matches.value_of("output").unwrap()).expect("Failed to create badege file");
    badge_file
        .write_all(badge.to_svg().as_bytes())
        .expect("Failed to write badgesvg file");
}

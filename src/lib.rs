use anyhow::{anyhow, Result};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "css-grammar.pest"]
pub struct CSSParser;

pub fn parse_selector(input: &str) -> Result<String> {
    CSSParser::parse(Rule::selector, input)
        .map(|pairs| pairs.as_str().to_string())
        .map_err(|e| anyhow!("Parsing failed for selector '{}': {}", input, e))
}

pub fn parse_property(input: &str) -> Result<String> {
    CSSParser::parse(Rule::property, input)
        .map(|pairs| {
            pairs
                .into_iter()
                .next()
                .map(|pair| pair.as_str().to_string())
                .ok_or_else(|| anyhow!("No valid property found in input: '{}'", input))
        })
        .map_err(|e| anyhow!("Parsing failed for property '{}': {}", input, e))
        .and_then(|res| res)
}

pub fn parse_hex_color(input: &str) -> Result<String> {
    let hex_color_regex = regex::Regex::new(r"^#([0-9A-Fa-f]{6})$").unwrap();

    if hex_color_regex.is_match(input) {
        Ok(format!("color: {}; ", input))
    } else {
        Err(anyhow!("Invalid hex color code '{}'", input))
    }
}

pub fn parse_dimension(input: &str) -> Result<String> {
    CSSParser::parse(Rule::dimension, input)
        .map(|pairs| pairs.as_str().to_string())
        .map_err(|e| anyhow!("Parsing failed for dimension '{}': {}", input, e))
}

pub fn parse_hex_digit(input: &str) -> Result<String> {
    CSSParser::parse(Rule::hex_digit, input)
        .map(|pairs| pairs.as_str().to_string())
        .map_err(|e| anyhow!("Parsing failed for hex digit '{}': {}", input, e))
}

pub fn parse_css_file(input: &str) -> Result<String> {
    CSSParser::parse(Rule::css_block, input)
        .map(|pairs| {
            let mut css_output = String::new();
            for pair in pairs {
                match pair.as_rule() {
                    Rule::css_block => {
                        let mut block_str = String::new();
                        let mut inner_pairs = pair.into_inner();

                        if let Some(selector_pair) = inner_pairs.next() {
                            let selector = selector_pair.as_str();
                            block_str.push_str(&format!("Selector:\n  {}\n", selector));
                        }

                        if let Some(properties_pair) = inner_pairs.next() {
                            let properties = parse_properties(properties_pair);
                            block_str.push_str(&format!("Properties:\n{}\n", properties));
                        }

                        css_output.push_str(&block_str);
                    }
                    _ => {
                        eprintln!("Unexpected rule: {:?}", pair.as_rule());
                    }
                }
            }
            css_output
        })
        .map_err(|e| anyhow!("Parsing failed for CSS file '{}': {}", input, e))
}

fn parse_properties(pair: pest::iterators::Pair<Rule>) -> String {
    pair.into_inner()
        .filter_map(|p| {
            if p.as_rule() == Rule::property {
                Some(format!("  {}", p.as_str()))
            } else {
                None
            }
        })
        .collect::<Vec<String>>()
        .join("\n")
}

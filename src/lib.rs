use pest::Parser;
use pest_derive::Parser;
use regex::Regex;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "css-grammar.pest"]
pub struct CSSParser;

#[derive(Error, Debug)]
pub enum CSSParseError {
    #[error("Could not parse this selector: {0}")]
    SelectorParse(String),
    
    #[error("Could not parse this property: {0}")]
    PropertyParse(String),

    #[error("Wrong hex-digit for color used: '{0}'")]
    HexDigitParse(String),
    
    #[error("Wrong hex-code for color used: '{0}'")]
    InvalidHexColor(String),
    
    #[error("Wrong dimension for property used: '{0}'")]
    DimensionParse(String),
    
    #[error("Could not parse this CSS-block: {0}")]
    CSSBlockParse(String),
}

type Result<T> = std::result::Result<T, CSSParseError>;

pub fn parse_selector(input: &str) -> Result<String> {
    CSSParser::parse(Rule::selector, input)
        .map(|pairs| pairs.as_str().to_string())
        .map_err(|e| CSSParseError::SelectorParse(format!("'{}': {}", input, e)))
}

pub fn parse_property(input: &str) -> Result<String> {
    CSSParser::parse(Rule::property, input)
        .map_err(|e| CSSParseError::PropertyParse(format!("'{}': {}", input, e))) 
        .map(|pairs| {
            pairs
                .into_iter()
                .next()
                .map(|pair| pair.as_str().to_string())
                .ok_or_else(|| CSSParseError::PropertyParse(format!("No valid property found in '{}'", input)))
        })
        .and_then(|res| res) 
}


pub fn parse_hex_color(input: &str) -> Result<String> {
    let hex_color_regex = Regex::new(r"^#([0-9A-Fa-f]{6})$").unwrap();
    if hex_color_regex.is_match(input) {
        Ok(format!("color: {}; ", input))
    } else {
        Err(CSSParseError::InvalidHexColor(input.to_string()))
    }
}

pub fn parse_dimension(input: &str) -> Result<String> {
    CSSParser::parse(Rule::dimension, input)
        .map(|pairs| pairs.as_str().to_string())
        .map_err(|e| CSSParseError::DimensionParse(format!("'{}': {}", input, e)))
}

pub fn parse_hex_digit(input: &str) -> Result<String> {
    CSSParser::parse(Rule::hex_digit, input)
        .map(|pairs| pairs.as_str().to_string())
        .map_err(|e| CSSParseError::HexDigitParse(format!("'{}': {}", input, e)))
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
        .map_err(|e| CSSParseError::CSSBlockParse(format!("'{}': {}", input, e)))
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

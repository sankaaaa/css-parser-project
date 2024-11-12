use anyhow::{anyhow, Result};
use css_parser_project::{
    parse_dimension, parse_hex_color, parse_hex_digit, parse_property, parse_selector,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_test() -> Result<()> {
        println!("!!Selector test started!!");

        let css_code = r#"
            .header { color: blue; }
            #headerBlock { font-size: 16px; }
            div { margin: 10px; }
            .fa-icons { font-family: 'FontAwesome'; }
        "#;

        let selectors = vec![
            (".header", Ok::<(), anyhow::Error>(())),
            ("#headerBlock", Ok::<(), anyhow::Error>(())),
            ("div", Ok::<(), anyhow::Error>(())),
            (".fa-icons", Ok::<(), anyhow::Error>(())),
        ];

        for (input, expected_result) in selectors {
            println!("Testing selector -> '{}'", input);

            let found = css_code.contains(input);
            let result = parse_selector(input);
            match expected_result {
                Ok(_) => {
                    if result.is_err() {
                        return Err(anyhow!("Parsing failed for selector '{}'", input));
                    }
                    if !found {
                        return Err(anyhow!("Selector '{}' not found in the CSS code", input));
                    }
                }
                Err(_) => {
                    if result.is_ok() {
                        return Err(anyhow!(
                            "Parsing should have failed for selector '{}'",
                            input
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    #[test]
    fn property_test() -> Result<()> {
        let css_properties = vec![
            (
                "color: blue;",
                Ok::<String, anyhow::Error>("color: blue;".to_string()),
            ),
            (
                "margin: 10px;",
                Ok::<String, anyhow::Error>("margin: 10px;".to_string()),
            ),
            (
                "padding: 20px;",
                Ok::<String, anyhow::Error>("padding: 20px;".to_string()),
            ),
            (
                "color: #ff0000;",
                Ok::<String, anyhow::Error>("color: #ff0000;".to_string()),
            ),
        ];

        for (input, expected_result) in css_properties {
            println!("Testing property -> '{}'", input);

            let result = parse_property(input)?;
            println!("Parsing result for '{}': {}", input, result);

            match expected_result {
                Ok(expected) => {
                    if result != expected {
                        return Err(anyhow!("Expected '{}' but got '{}'", expected, result));
                    }
                }
                Err(_) => {
                    return Err(anyhow!(
                        "Parsing should have failed for property '{}'",
                        input
                    ));
                }
            }
        }

        Ok(())
    }

    #[test]
    fn hex_color_test() -> Result<()> {
        let hex_code = "#e4524c";
        let parsed_color = parse_hex_color(hex_code)?.trim_end().to_string();
        let expected = "color: #e4524c;";

        if parsed_color != expected {
            return Err(anyhow!(
                "Expected '{}' but got '{}'",
                expected,
                parsed_color
            ));
        }

        Ok(())
    }

    #[test]
    fn dimension_test() -> Result<()> {
        let dimensions = vec![
            ("10px", Ok::<String, anyhow::Error>("10px".to_string())),
            ("10em", Ok::<String, anyhow::Error>("10em".to_string())),
            ("20%", Ok::<String, anyhow::Error>("20%".to_string())),
            ("5pt", Ok::<String, anyhow::Error>("5pt".to_string())),
        ];

        for (input, expected_result) in dimensions {
            println!("Testing dimension -> '{}'", input);

            let result = parse_dimension(input)?;
            match expected_result {
                Ok(expected) => {
                    if result != expected {
                        return Err(anyhow!("Expected '{}' but got '{}'", expected, result));
                    }
                }
                Err(_) => {
                    return Err(anyhow!(
                        "Parsing should have failed for dimension '{}'",
                        input
                    ));
                }
            }
        }

        Ok(())
    }

    #[test]
    fn hex_digit_test() -> Result<()> {
        let hex_digits = vec![
            ("a", Ok::<String, anyhow::Error>("a".to_string())),
            ("f", Ok::<String, anyhow::Error>("f".to_string())),
            ("0", Ok::<String, anyhow::Error>("0".to_string())),
            ("9", Ok::<String, anyhow::Error>("9".to_string())),
        ];

        for (input, expected_result) in hex_digits {
            println!("Testing hex digit -> '{}'", input);

            let result = parse_hex_digit(input);

            match (result, expected_result) {
                (Ok(ref result_val), Ok(ref expected_val)) => {
                    if result_val != expected_val {
                        return Err(anyhow!(
                            "Expected '{}' but got '{}'",
                            expected_val,
                            result_val
                        ));
                    }
                }
                (Err(_), Err(_)) => {}
                (Ok(_), Err(_)) => {
                    return Err(anyhow!(
                        "Parsing should have failed for hex digit '{}'",
                        input
                    ));
                }
                (Err(_), Ok(_)) => {
                    return Err(anyhow!(
                        "Parsing should have succeeded for hex digit '{}'",
                        input
                    ));
                }
            }
        }

        Ok(())
    }
}

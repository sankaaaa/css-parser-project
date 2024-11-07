use pest::Parser;
use pest_derive::Parser;
use anyhow::{anyhow, Result};
use std::fs;

#[derive(Parser)]
#[grammar = "src/css-grammar.pest"]
struct CSSParser;

#[test]
fn selector_test() -> Result<()> {
    println!("!!Selector test started!!");

    let css_code = fs::read_to_string("tests/css_code.txt")
        .map_err(|e| anyhow!("Failed to read CSS file: {}", e))?;

    let selectors = vec![
        (".header", Ok::<(), anyhow::Error>(())),
        ("#headerBlock", Ok::<(), anyhow::Error>(())),
        ("div", Ok::<(), anyhow::Error>(())),
        (".fa-icons", Ok::<(), anyhow::Error>(())),
        ("54dd", Err(anyhow!("Invalid selector: 54dd"))),
    ];

    for (input, expected_result) in selectors {
        println!("Testing selector -> '{}'", input);

        let found = css_code.contains(input);

        let result: Result<(), anyhow::Error> = CSSParser::parse(Rule::selector, input)
            .map(|_pairs| ())
            .map_err(|e| anyhow!("Parsing failed for input '{}': {}", input, e));

        match expected_result {
            Ok(_) => {
                if result.is_err() {
                    return Err(anyhow!("Parsing failed for input '{}'", input));
                }
                if !found {
                    return Err(anyhow!("Selector '{}' not found in the CSS file", input));
                }
            },
            Err(_) => {
                if result.is_ok() {
                    return Err(anyhow!("Parsing should have failed for input '{}'", input));
                }
            }
        }
    }

    Ok(())
}

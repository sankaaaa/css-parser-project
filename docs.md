### CSS parser project

### General description
This is a parser program, developed on Rust, that will parse the CSS code (its structure, selectors, properties and basic values). 

### Technical description of parsing process
This parser processes the CSS rules and breaks it into selectors and structures. The grammar includes selectors, values, properties. Parser takes the input CSS code, and on the base of rules creates structure, and saves it to the output like "selector -> declaration" (like the table key:value).

### Grammar
CSS block: selector and properties wrapped in curly braces.
css_block = { selector ~ WHITESPACE* ~ "{" ~ WHITESPACE* ~ properties ~ WHITESPACE* ~ "}" }

CSS selector, which can represent a class, ID, or tag.
selector = { ("#" ~ ASCII_ALPHANUMERIC+ | "." ~ ASCII_ALPHANUMERIC+ | ASCII_ALPHANUMERIC+) }

CSS property consisting of an identifier and a value.
property = { identifier ~ ": " ~ value ~ ";" }

List of properties for a CSS selector.
properties = { property+ }

The value of a CSS property, which can be a color, dimension, or identifier.
value = { color | dimension | identifier }

Size measurement, for example, `10px`, `2em`.
dimension = { DIGIT+ ~ ("." ~ DIGIT+)? ~ (("px" | "em" | "%" | "pt")) }

A digit from 0 to 9.
DIGIT = { '0'..'9' }

Hexadecimal color, for example, `#ff0000`.
hex_color = { "#" ~ hex_digit ~ hex_digit ~ hex_digit ~ hex_digit ~ hex_digit ~ hex_digit }

Hexadecimal digit (0-9, a-f).
hex_digit = { "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "a" | "b" | "c" | "d" | "e" | "f" }

Standard color or hexadecimal color value.
color = { "blue" | "red" | "green" | "yellow" | "black" | "white" | hex_color }

Identifier consisting of alphabetic characters and digits.
identifier = { ASCII_ALPHANUMERIC+ }

Space or newline character for separating elements.
WHITESPACE = _{ " " | "\t" | "\n" | "\r" }

### Struct CSSParser
The CSSParser struct is a parser that uses the Pest library to parse CSS code according to a predefined grammar.

### CSSParseError
The CSSParseError enum defines different error types that occur during the CSS parsing process, each carrying a String describing the specific error encountered.

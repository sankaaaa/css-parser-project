### CSS parser

### General description
This is a parser program, developed on Rust, that will parse the CSS code (its structure, selectors, properties and basic values). 

### Technical description of parsing process
This parser processes the CSS rules and breaks it into selectors and structures. The grammar includes selectors, values, properties. Parser takes the input CSS code, and on the base of rules creates structure, and saves it to the output like "selector -> declaration" (like the table key:value).

### Example CSS input
```CSS
.header {
    background-color: black;
    width: 100%
}
```

![CSS parsing process](assets/css_parser.png)
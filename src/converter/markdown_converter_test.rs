use crate::converter::markdown_converter::MarkdownConverter;

#[test]
fn test_convert_simple_html() {
    let html = r#"
        <html>
            <head><title>Test Page</title></head>
            <body>
                <h1>Hello World</h1>
                <p>This is a test paragraph.</p>
            </body>
        </html>
    "#;

    let markdown_converter = MarkdownConverter::new();
    let markdown = markdown_converter.convert(html).expect("Failed to convert HTML to Markdown");
    assert!(markdown.contains("# Hello World"));
    assert!(markdown.contains("This is a test paragraph"));
}

#[test]
fn test_convert_with_lists() {
    let html = r#"
        <html>
            <body>
                <ul>
                    <li>Item 1</li>
                    <li>Item 2</li>
                </ul>
                <ol>
                    <li>First</li>
                    <li>Second</li>
                </ol>
            </body>
        </html>
    "#;

    let markdown_converter = MarkdownConverter::new();
    let markdown = markdown_converter.convert(html).expect("Failed to convert HTML to Markdown");
    assert!(markdown.contains("* Item 1"));
    assert!(markdown.contains("* Item 2"));
    assert!(markdown.contains("1. First"));
    assert!(markdown.contains("2. Second"));
}

#[test]
fn test_convert_with_table() {
    let html = r#"
        <html>
            <body>
                <table>
                    <thead>
                        <tr>
                            <th>Header 1</th>
                            <th>Header 2</th>
                        </tr>
                    </thead>
                    <tbody>
                        <tr>
                            <td>Cell 1</td>
                            <td>Cell 2</td>
                        </tr>
                    </tbody>
                </table>
            </body>
        </html>
    "#;

    let markdown_converter = MarkdownConverter::new();
    let markdown = markdown_converter.convert(html).expect("Failed to convert HTML to Markdown");
    assert!(markdown.contains("|Header 1|Header 2|"));
    assert!(markdown.contains("|Cell 1|Cell 2|"));
    assert!(markdown.contains("|---|---|"));
}

use anyhow::Result;
use scraper::{Html, Selector, ElementRef};
use once_cell::sync::Lazy;

static TITLE_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("title").unwrap());
static MAIN_CONTENT_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("main, article, .content, #content, .main, #main").unwrap());
static BODY_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("body").unwrap());
static LI_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("li").unwrap());
static TD_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("td").unwrap());
static TH_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("th").unwrap());
static TR_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("tr").unwrap());
static TBODY_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("tbody").unwrap());
static THEAD_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("thead").unwrap());
static CODE_SELECTOR: Lazy<Selector> = Lazy::new(|| Selector::parse("code").unwrap());
static SKIP_HEADER_SELECTOR: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("header.header, header#site-header, header.site-header, header.page-header, header.banner, header#masthead").unwrap()
});
static SKIP_LANGUAGE_SELECTOR: Lazy<Selector> = Lazy::new(|| {
    Selector::parse("#p-lang, .interlanguage-link, #p-lang-btn, .language-list, #language-list, .mw-interlanguage-selector").unwrap()
});
static SKIP_INFOBOX_SELECTOR: Lazy<Selector> = Lazy::new(|| {
    Selector::parse(".infobox, .vcard, .infobox.vcard, .infobox.biography.vcard").unwrap()
});

pub struct MarkdownConverter {
    skip_tags: Vec<&'static str>,
}

impl MarkdownConverter {
    pub fn new() -> Self {
        Self {
            skip_tags: vec![
                "nav", "footer", "script", "style", "noscript", "iframe", "meta",
                "link", // For external stylesheets
                "svg", // Often contains styling
                "path", // SVG elements
                "defs", // SVG definitions
                "symbol", // SVG symbols
                "use", // SVG use elements
                "template", // HTML templates that might contain styles
                "input", // Input elements
                "button", // Button elements
                "form", // Form elements
                "select", // Select elements
                "option", // Option elements
                "textarea", // Textarea elements
            ],
        }
    }

    pub fn convert(&self, html: &str) -> Result<String> {
        let document = Html::parse_document(html);
        let mut markdown = String::with_capacity(html.len() / 2); // Pre-allocate buffer

        // Extract title if available
        if let Some(title) = document.select(&TITLE_SELECTOR)
            .next()
            .map(|title| title.text().collect::<Vec<_>>().join(" ").trim().to_string()) {
            markdown.push_str(&format!("Title: {}\n\n", title));
        }

        // Add Markdown Content header
        markdown.push_str("Markdown Content:\n");

        // Process main content
        if let Some(main_content) = document.select(&MAIN_CONTENT_SELECTOR).next() {
            self.process_element(main_content, &mut markdown, 0);
        } else {
            if let Some(body) = document.select(&BODY_SELECTOR).next() {
                self.process_body_content(body, &mut markdown);
            } else {
                let root = ElementRef::from(document.root_element());
                self.process_body_content(root, &mut markdown);
            }
        }

        Ok(markdown)
    }

    fn process_body_content(&self, element: ElementRef, markdown: &mut String) {
        for child in element.children() {
            if let Some(child) = ElementRef::wrap(child) {
                if !self.skip_tags.contains(&child.value().name.local.as_ref()) {
                    self.process_element(child, markdown, 0);
                }
            }
        }
    }

    fn process_element(&self, element: ElementRef, markdown: &mut String, depth: usize) {
        let tag_name = element.value().name();

        // Skip style elements completely
        if tag_name == "style" {
            return;
        }
        
        // Skip elements with style-related attributes
        if element.value().attr("style").is_some() || 
           element.value().attr("class").map(|c| c.contains("style-scope")).unwrap_or(false) {
            return;
        }
        
        // Skip elements in skip_tags list
        if self.skip_tags.contains(&tag_name) {
            return;
        }
        
        // Skip site headers
        if SKIP_HEADER_SELECTOR.matches(&element) {
            return;
        }

        // Skip language list
        if SKIP_LANGUAGE_SELECTOR.matches(&element) {
            return;
        }

        // Skip infobox/vcard elements
        if SKIP_INFOBOX_SELECTOR.matches(&element) {
            return;
        }

        // Skip elements with infobox/vcard-related classes
        if element.value().attr("class").map(|c| {
            c.contains("infobox") || 
            c.contains("vcard") || 
            c.contains("metadata")
        }).unwrap_or(false) {
            return;
        }

        // Skip elements with language-related classes
        if element.value().attr("class").map(|c| {
            c.contains("interwiki") || 
            c.contains("language-list") || 
            c.contains("lang-list") ||
            c.contains("mw-interlanguage")
        }).unwrap_or(false) {
            return;
        }
        
        match tag_name {
            "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                let level = tag_name.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;
                let text = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
                if !text.is_empty() {
                    // Add separator line before headers
                    if level <= 2 {
                        markdown.push_str(&format!("\n{}\n", "-".repeat(74)));
                    }
                    markdown.push_str(&format!("\n{} {}\n", "#".repeat(level), text));
                }
            },
            "p" => {
                let text = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
                if !text.is_empty() {
                    markdown.push_str(&text);
                    markdown.push_str("\n\n");
                }
            },
            "blockquote" => {
                for child in element.children() {
                    if let Some(child_ref) = ElementRef::wrap(child) {
                        let text = child_ref.text().collect::<Vec<_>>().join(" ").trim().to_string();
                        if !text.is_empty() {
                            markdown.push_str(&format!("> {}\n", text));
                        }
                    }
                }
                markdown.push('\n');
            },
            "a" => {
                if let Some(href) = element.value().attr("href") {
                    let text = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
                    if !text.is_empty() {
                        markdown.push_str(&format!("[{}]({})", text, href));
                    }
                }
            },
            "img" => {
                if let Some(src) = element.value().attr("src") {
                    let alt = element.value().attr("alt").unwrap_or("");
                    markdown.push_str(&format!("![{}]({})\n\n", alt, src));
                }
            },
            "ul" => {
                let mut items = Vec::new();
                for li in element.select(&LI_SELECTOR) {
                    let text = li.text().collect::<Vec<_>>().join(" ").trim().to_string();
                    if !text.is_empty() {
                        items.push(text);
                    }
                }
                if !items.is_empty() {
                    markdown.push('\n');
                    for item in items {
                        markdown.push_str(&format!("* {}\n", item));
                    }
                    markdown.push('\n');
                }
            },
            "ol" => {
                let mut items = Vec::new();
                for li in element.select(&LI_SELECTOR) {
                    let text = li.text().collect::<Vec<_>>().join(" ").trim().to_string();
                    if !text.is_empty() {
                        items.push(text);
                    }
                }
                if !items.is_empty() {
                    markdown.push('\n');
                    for (i, item) in items.iter().enumerate() {
                        markdown.push_str(&format!("{}. {}\n", i + 1, item));
                    }
                    markdown.push('\n');
                }
            },
            "table" => {
                self.process_table(element, markdown);
            },
            "pre" => {
                if let Some(code) = element.select(&CODE_SELECTOR).next() {
                    let language = code.value().attr("class")
                        .unwrap_or("")
                        .split_whitespace()
                        .find(|class| class.starts_with("language-"))
                        .unwrap_or("language-text")
                        .replace("language-", "");
                    
                    let code_text = code.text().collect::<Vec<_>>().join("").trim().to_string();
                    if !code_text.is_empty() {
                        markdown.push_str(&format!("```{}\n{}\n```\n\n", language, code_text));
                    }
                }
            },
            "hr" => {
                markdown.push_str(&format!("{}\n", "-".repeat(74)));
            },
            "br" => {
                markdown.push_str("  \n");
            },
            _ => {
                // Process children for other elements
                for child in element.children() {
                    if let Some(child_ref) = ElementRef::wrap(child) {
                        self.process_element(child_ref, markdown, depth + 1);
                    }
                }
            }
        }
    }

    fn process_table(&self, table: ElementRef, markdown: &mut String) {
        let mut headers = Vec::new();
        let mut rows = Vec::new();

        // Process headers
        if let Some(thead) = table.select(&THEAD_SELECTOR).next() {
            for th in thead.select(&TH_SELECTOR) {
                headers.push(th.text().collect::<Vec<_>>().join(" ").trim().to_string());
            }
        }

        // Process rows
        for tr in table.select(&TBODY_SELECTOR).next().unwrap().select(&TR_SELECTOR) {
            let row: Vec<String> = tr.select(&TD_SELECTOR)
                .map(|td| td.text().collect::<Vec<_>>().join(" ").trim().to_string())
                .collect();
            if !row.is_empty() {
                rows.push(row);
            }
        }

        // Output table in markdown format
        if !headers.is_empty() {
            markdown.push_str(&format!("|{}|\n", headers.join("|")));
            markdown.push_str(&format!("|{}|\n", headers.iter().map(|_| "---").collect::<Vec<_>>().join("|")));
        }

        for row in rows {
            markdown.push_str(&format!("|{}|\n", row.join("|")));
        }
        markdown.push('\n');
    }
}

#[cfg(test)]
#[path = "markdown_converter_test.rs"]
mod tests;

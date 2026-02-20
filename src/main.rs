use dioxus::prelude::*;
use dioxus_markdown::Markdown;
use mermaid_rs_renderer::render;
use regex::Regex;
use std::borrow::Cow;

//const FAVICON: Asset = asset!("/assets/favicon.ico");
//const MAIN_CSS: Asset = asset!("/assets/main.css");
//const HEADER_SVG: Asset = asset!("/assets/header.svg");
//const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

fn wrap_mermaid_blocks(markdown: &str) -> String {
    // Regex to capture fenced code blocks with the 'mermaid' language identifier.
    // It captures the content inside the block.
    // The pattern uses `(?s)` to enable dotall mode, allowing `.` to match newlines.
    // The capture group 1 (`(.*?)`) gets the actual diagram content.
    let re = Regex::new(r"(?s)```mermaid\s*\n(.*?)\n```").expect("Invalid regex");

    // Use replace_all with a closure to process each match.
    // The closure receives a `&Captures` object.
    let result: Cow<str> = re.replace_all(markdown, |caps: &regex::Captures| {
        // Extract the inner content of the mermaid block (capture group 1).
        let content = caps.get(1).map_or("", |m| m.as_str());

        // Format the replacement string, wrapping the content in a <div> block
        // and also including the original "mermaid" class in a <pre> tag 
        // to maintain compatibility with most Mermaid renderers.
        // Many renderers look for the <pre class="mermaid"> syntax.
        let new_content = mermaid_rs_renderer::render(content).unwrap();
        format!(
            r#"<div class="mermaid-wrap">{}</div>"#,
            new_content
        )
    });

    // Convert Cow<str> to String
    result.into_owned()
}

#[component]
fn App() -> Element {
    let mermaid_code1 = "graph TD; A-->B; B-->C;";
    let mermaid_code2 = "graph TD; D-->E; E-->F;";
    let svg_image = mermaid_rs_renderer::render(mermaid_code1).unwrap();
    let initial_text = format!("# Welcome\n```mermaid\n{mermaid_code1}\n```\n```mermaid\n{mermaid_code2}\n```\n").to_string();
    let mut text = use_signal(|| initial_text.to_string());
    let mut filtered_text = use_signal(|| initial_text.to_string());
    let mut show_first = use_signal(|| true);

    rsx! {
        button {
            onclick: move |_| { text.set("# Today's Note".to_string()); show_first.set(true); },
            "Today's Note"
        }
        button {
            onclick: move |_| { text.set("# TODO".to_string()); show_first.set(true); },
            "TODO"
        }
        button {
            onclick: move |_| { filtered_text.set(wrap_mermaid_blocks(&text().clone())); show_first.toggle(); },
            "Edit/Preview"
        }
        if *show_first.read() {
                div {
                    style: "display: flex; gap: 20px; padding: 20px; height: 90vh;",
                    
                    // 1. Markdown Editor (Input)
                    textarea {
                        style: "flex: 1; padding: 10px; font-family: monospace;",
                        value: "{text}",
                        oninput: move |evt| text.set(evt.value())
                    }
            }
        } else {
            div {
                    style: "flex: 1; padding: 10px; border: 1px solid #ccc; overflow-y: auto;",
                    // The library renders the string directly into Dioxus VNodes
                    Markdown { src: {filtered_text} }
                }
        }
    }
}


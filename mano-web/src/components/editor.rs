use leptos::*;

#[component]
pub fn Editor(
    source_code: ReadSignal<String>,
    set_source_code: WriteSignal<String>,
) -> impl IntoView {
    let line_count = move || source_code.get().lines().count().max(1);

    let on_input = move |ev| {
        set_source_code.set(event_target_value(&ev));
    };

    // Syntax highlighting function
    let highlight_syntax = move || {
        let code = source_code.get();
        code.lines()
            .map(|line| highlight_line(line))
            .collect::<Vec<_>>()
            .join("\n")
    };

    view! {
        <div class="editor-pane">
            <h2 class="pane-title">"Program Input"</h2>
            <div class="editor-container">
                <div class="line-numbers">
                    {move || (1..=line_count()).map(|n|
                        view! { <div class="line-number">{n}</div> }
                    ).collect_view()}
                </div>
                <div class="editor-text-container">
                    <pre class="editor-highlight" inner_html=move || highlight_syntax()></pre>
                    <textarea
                        class="editor-textarea"
                        prop:value=move || source_code.get()
                        on:input=on_input
                        spellcheck="false"
                        placeholder="Enter Mano assembly code here..."
                    />
                </div>
            </div>
        </div>
    }
}

fn highlight_line(line: &str) -> String {
    // If line is empty, return a space to maintain line height
    if line.trim().is_empty() {
        return " ".to_string();
    }

    // Check if line starts with a comment
    if line.trim_start().starts_with('/') {
        return format!("<span class=\"hl-comment\">{}</span>", html_escape(line));
    }

    let mut result = String::new();
    let mut chars = line.chars().peekable();
    let mut current_token = String::new();
    let mut in_comment = false;

    while let Some(ch) = chars.next() {
        if in_comment {
            current_token.push(ch);
            continue;
        }

        match ch {
            '/' => {
                // Flush current token
                if !current_token.is_empty() {
                    result.push_str(&highlight_token(&current_token));
                    current_token.clear();
                }
                // Start comment
                in_comment = true;
                current_token.push(ch);
            }
            ',' | ' ' | '\t' => {
                // Flush current token
                if !current_token.is_empty() {
                    result.push_str(&highlight_token(&current_token));
                    current_token.clear();
                }
                result.push(ch);
            }
            _ => {
                current_token.push(ch);
            }
        }
    }

    // Flush remaining token
    if !current_token.is_empty() {
        if in_comment {
            result.push_str(&format!("<span class=\"hl-comment\">{}</span>", html_escape(&current_token)));
        } else {
            result.push_str(&highlight_token(&current_token));
        }
    }

    // If result is empty, return space to maintain line height
    if result.trim().is_empty() {
        " ".to_string()
    } else {
        result
    }
}

fn highlight_token(token: &str) -> String {
    let trimmed = token.trim();
    if trimmed.is_empty() {
        return token.to_string();
    }

    // Check if it's an instruction
    let upper = trimmed.to_uppercase();
    let instructions = [
        "AND", "ADD", "LDA", "STA", "BUN", "BSA", "ISZ",
        "CLA", "CLE", "CMA", "CME", "CIR", "CIL", "INC",
        "SPA", "SNA", "SZA", "SZE", "HLT",
        "INP", "OUT", "SKI", "SKO", "ION", "IOF"
    ];

    if instructions.contains(&upper.as_str()) {
        return format!("<span class=\"hl-instruction\">{}</span>", html_escape(token));
    }

    // Check if it's a pseudo-instruction
    let pseudo = ["ORG", "DEC", "HEX", "END"];
    if pseudo.contains(&upper.as_str()) {
        return format!("<span class=\"hl-pseudo\">{}</span>", html_escape(token));
    }

    // Check if it's a number
    if trimmed.chars().all(|c| c.is_ascii_digit() || c == '-') {
        return format!("<span class=\"hl-number\">{}</span>", html_escape(token));
    }

    // Check if it's a label (ends with comma in context, but here we just check if it's alphanumeric)
    // Labels are just identifiers, so we'll use a default color
    html_escape(token)
}

fn html_escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

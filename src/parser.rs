use mailparse::parse_mail;

pub struct ParsedEmail {
    pub urls: Vec<String>,
    pub attachments: Vec<Attachment>,
}

pub struct Attachment {
    pub filename: String,
    pub content: Vec<u8>,
}

pub fn parse_email(raw: &[u8]) -> ParsedEmail {
    let mut urls = Vec::new();
    let mut attachments = Vec::new();

    let parsed = parse_mail(raw).expect("Failed to parse mail");

    for part in parsed.subparts.iter() {
        let content_type = part.ctype.mimetype.to_lowercase();
        if content_type.starts_with("text/") {
            let body = part.get_body().unwrap_or_default();
            let found_urls = extract_urls(&body);
            urls.extend(found_urls);
        } else {
            if let Some(filename) = part.ctype.params.get("name") {
                let content = part.get_body_raw().unwrap_or_default();
                attachments.push(Attachment {
                    filename: filename.clone(),
                    content,
                })
            }
        }
    }
    ParsedEmail { urls, attachments }
}

fn extract_urls(text: &str) -> Vec<String> {
    let mut urls = Vec::new();

    for word in text.split_whitespace() {
        let clean = word.trim_matches(|c: char| c == '"' || c == '\'' || c == '<' || c == '>');
        if clean.starts_with("https://") || clean.starts_with("http://") {
            urls.push(clean.to_string());
        }
    }
    urls
}

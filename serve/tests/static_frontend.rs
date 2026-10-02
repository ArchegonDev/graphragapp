use std::fs;
use std::path::PathBuf;

const PAGES: [&str; 4] = [
    "index.html",
    "dcceew/index.html",
    "parkinsons/index.html",
    "little-bunnies/index.html",
];
const USE_CASES: [&str; 3] = [
    "dcceew/index.html",
    "parkinsons/index.html",
    "little-bunnies/index.html",
];

fn page(rel: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("static")
        .join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

fn tags<'a>(html: &'a str, name: &str) -> Vec<&'a str> {
    let open = format!("<{name}");
    let mut found = Vec::new();
    let mut rest = html;
    while let Some(i) = rest.find(&open) {
        let after = &rest[i + open.len()..];
        if after.starts_with(|c: char| c.is_whitespace() || c == '>' || c == '/') {
            let end = after.find('>').map_or(after.len(), |e| e + 1);
            found.push(&rest[i..i + open.len() + end]);
        }
        rest = after;
    }
    found
}

#[test]
fn every_page_has_document_basics() {
    for rel in PAGES {
        let html = page(rel);
        assert!(html.contains("<html lang=\"en\""), "{rel}: html lang");
        assert!(html.contains("name=\"viewport\""), "{rel}: viewport meta");
        assert_eq!(tags(&html, "h1").len(), 1, "{rel}: exactly one h1");
        assert!(
            html.contains("class=\"skip\" href=\"#main\""),
            "{rel}: skip link to #main"
        );
        assert!(html.contains("id=\"main\""), "{rel}: main target");
    }
}

#[test]
fn every_page_shares_primary_navigation() {
    for rel in PAGES {
        let html = page(rel);
        assert!(
            html.contains("<nav class=\"site-nav\" aria-label=\"Primary\">"),
            "{rel}: shared primary nav"
        );
        for href in ["/#how-it-works", "/#use-cases", "/#technical-guide"] {
            assert!(html.contains(&format!("href=\"{href}\"")), "{rel}: {href}");
        }
    }
}

#[test]
fn homepage_has_story_anchors() {
    let html = page("index.html");
    for id in ["how-it-works", "use-cases", "technical-guide"] {
        assert!(html.contains(&format!("id=\"{id}\"")), "homepage: #{id}");
    }
}

#[test]
fn use_case_pages_announce_demo_status() {
    for rel in USE_CASES {
        let html = page(rel);
        let status = tags(&html, "p")
            .into_iter()
            .chain(tags(&html, "div"))
            .any(|t| t.contains("class=\"demo-status\"") && t.contains("aria-live=\"polite\""));
        assert!(status, "{rel}: demo status region with aria-live=polite");
    }
}

#[test]
fn every_image_declares_alt_text() {
    for rel in PAGES {
        let html = page(rel);
        for img in tags(&html, "img") {
            assert!(img.contains(" alt=\""), "{rel}: image without alt: {img}");
        }
    }
}

#[test]
fn legacy_chrome_tabs_are_gone() {
    for rel in PAGES {
        assert!(
            !page(rel).contains("class=\"chrome\""),
            "{rel}: legacy chrome tabs"
        );
    }
    assert!(!page("guide.css").contains(".chrome"), "guide.css: .chrome");
}

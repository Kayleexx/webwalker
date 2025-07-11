use rusty_crawler::crawler::parser::extract_links;
use url::Url;

#[test]
fn extract_rel_and_abs_links() {
    let html = r#"<a href="/a">a</a><a href="https://x.com/b">b</a>"#;
    let base = Url::parse("https://example.com").unwrap();
    let links = extract_links(html, &base);
    let urls: Vec<_> = links.into_iter().map(|u| u.to_string()).collect();
    assert!(urls.contains(&"https://example.com/a".to_string()));
    assert!(urls.contains(&"https://x.com/b".to_string()));
}

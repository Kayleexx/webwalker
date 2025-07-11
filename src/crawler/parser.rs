use scraper::{Html, Selector};
use url::Url;

/// Extract absolute URLs from an HTML string.
pub fn extract_links(html: &str, base: &Url) -> Vec<Url> {
    let doc = Html::parse_document(html);
    let sel = Selector::parse("a[href]").unwrap();
    doc.select(&sel)
        .filter_map(|el| el.value().attr("href"))
        .filter_map(|href| base.join(href).ok())
        .collect()
}

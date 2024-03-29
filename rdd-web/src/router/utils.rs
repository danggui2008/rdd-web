//根据'/'路径分割：如：/p/blog切分后[p,blog]
pub(crate) fn parse_pattern(pattern: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let list = pattern.split('/').into_iter().collect::<Vec<&str>>();
    for part in list {
        if part != "" {
            parts.push(part);
            if part.starts_with("*") {
                break;
            }
        }
    }
    parts
}

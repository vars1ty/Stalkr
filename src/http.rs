/// Checks if the alias on the specified website is valid and returned a `success` HTTP code.
pub fn check_if_valid(source: &str, alias: &str) -> bool {
    let client = reqwest::blocking::Client::new();
    let source = source.replace("[user]", alias);
    client
        .get(source)
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; rv:108.0) Gecko/20100101 Firefox/108.0",
        )
        .send()
        .expect("[ERROR] Failed sending request!")
        .status()
        .as_u16()
        == 200
}

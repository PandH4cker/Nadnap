pub fn exclude_hosts(t: &mut Vec<String>, exclude_list: Vec<String>) {
    t.retain(|x| !exclude_list.contains(
        &x
            .strip_suffix("\r\n")
            .or(x.strip_suffix("\n"))
            .unwrap_or(x)
            .to_string()
    ))
}
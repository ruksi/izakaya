use url::Host;

// as our API server and frontend are on different subdomains, we want to assign cookie
// domain to the registrable domain instead of the subdomain scope to share the cookie
// note _does_ make the cookie insecure on shared domains like "onrender.com",
// domain-scoped cookies are only secure if you control all subdomains of the domain.

pub fn cookie_domain_from(urls: &Vec<String>) -> Result<Option<String>, url::ParseError> {
    let urls = urls
        .iter()
        .map(|u| url::Url::parse(u))
        .collect::<Result<Vec<_>, _>>()?;

    let domains = urls
        .iter()
        .map(|url| match url.host() {
            Some(Host::Domain(domain)) => domain.to_string(),
            Some(Host::Ipv4(ip)) => ip.to_string(),
            Some(Host::Ipv6(ip)) => ip.to_string(),
            None => "".to_string(),
        })
        .collect::<Vec<_>>();

    let segmented = domains
        .iter()
        .map(|domain| domain.split('.').rev().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut common = segmented[0].clone();
    for segment in segmented.into_iter().skip(1) {
        common = common
            .into_iter()
            .zip(segment.into_iter())
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect();
    }

    let prefix = common
        .into_iter()
        .rev()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(".");

    match prefix.is_empty() {
        true => Ok(None),
        false => Ok(Some(prefix)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cookie_domain_from_singles() -> Result<(), String> {
        let cases = [
            (vec!["http://127.0.0.1".into()], "127.0.0.1"),
            (vec!["http://localhost:5173".into()], "localhost"),
            (vec!["https://example.com".into()], "example.com"),
            (vec!["https://sub.example.com/".into()], "sub.example.com"),
        ];
        for (case, expected) in cases.iter() {
            let result = cookie_domain_from(case).unwrap().unwrap();
            assert_eq!(result, *expected);
        }
        Ok(())
    }

    #[test]
    fn cookie_domain_from_pairs() -> Result<(), String> {
        #[rustfmt::skip]
            let cases = [
            (vec!["http://127.0.0.1".into(), "http://localhost".into()], None),
            (vec!["http://localhost:5173".into(), "http://localhost:3000".into()], Some("localhost".into())),
            (vec!["https://alpha.example.com".into(), "https://beta.example.com".into()], Some("example.com".into())),
            (vec!["https://a.b.c.com".into(), "http://z.b.c.com".into()], Some("b.c.com".into())),
        ];
        for (case, expected) in cases.into_iter() {
            let result = cookie_domain_from(&case).unwrap();
            assert_eq!(result, expected);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_match_http_domain() {
        let ingres_description = String::from(
            "
            Labels:           acme.cert-manager.io/http-domain=416419534
                                                     acme.cert-manager.io/http-token=1210846832
                                                     acme.cert-manager.io/http01-solver=true
            ",
        );

        let mut v: Vec<&str> = ingres_description.split("http-domain").collect();
        v = v[1].split("http-token=").collect();
        let mut v1: Vec<&str> = v[0].split("\n").collect();
        v1 = v1[0].split("=").collect();
        assert_eq!(v1[1], "416419534")
    }

    #[test]
    fn test_match_fail_http_domain() {
        let ingres_description = String::from(
            "
            Labels:           acme.cert-manager.io/http-domain=416419534
                                                     acme.cert-manager.io/http-token=1210846832
                                                     acme.cert-manager.io/http01-solver=true
            ",
        );

        let mut v: Vec<&str> = ingres_description.split("http-domain").collect();
        v = v[1].split("http-token=").collect();
        let mut v1: Vec<&str> = v[0].split("\n").collect();
        v1 = v1[0].split("=").collect();
        assert!(v1[1] != "416419535");
    }

    #[test]
    fn test_match_http_token() {
        let ingres_description = String::from(
            "
            Labels:           acme.cert-manager.io/http-domain=416419534
                                                     acme.cert-manager.io/http-token=1210846832
                                                     acme.cert-manager.io/http01-solver=true
            ",
        );

        let mut v: Vec<&str> = ingres_description.split("http-domain").collect();
        v = v[1].split("http-token=").collect();
        let v2: Vec<&str> = v[1].split("\n").collect();
        assert_eq!(v2[0], "1210846832")
    }

    #[test]
    fn test_match_fail_http_token() {
        let ingres_description = String::from(
            "
                Labels:           acme.cert-manager.io/http-domain=416419534
                                                         acme.cert-manager.io/http-token=1210846832
                                                         acme.cert-manager.io/http01-solver=true
                ",
        );

        let mut v: Vec<&str> = ingres_description.split("http-domain").collect();
        v = v[1].split("http-token=").collect();
        let v2: Vec<&str> = v[1].split("\n").collect();
        assert!(v2[0] != "1210846831");
    }
}

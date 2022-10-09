use crate::UrlParsing;
use std::ops::Deref;
use std::ops::Add;

impl UrlParsing {
    /// Computes a `used_text` from an input URL according to the passed in `UrlParsing` object.
    /// Aims to be kinda compatible to Passwordmaker Pro.
    pub(super) fn make_used_text_from_url(&self, input : &str, ) -> String {
        parse_url(input).filter_by_settings(self).recombine()
    }

    fn is_protocol_used(&self) -> bool{
        match self.use_protocol{
            crate::ProtocolUsageMode::Ignored => false,
            crate::ProtocolUsageMode::Used
             | crate::ProtocolUsageMode::UsedWithUndefinedIfEmpty
             => true,
        }
    }
    fn use_protocol_undefined_fallback(&self) -> bool{
        match self.use_protocol{
            crate::ProtocolUsageMode::Ignored
             | crate::ProtocolUsageMode::Used
             => false,
            crate::ProtocolUsageMode::UsedWithUndefinedIfEmpty => true,
        }
    }
}

#[cfg_attr(test, derive(PartialEq, Debug, Clone))]
struct UrlParts<'s> {
    protocol : &'s str,
    userinfo : &'s str, //Treating this field separate is an addition to the functionaliyt offered by PasswordMaker Pro
    subdomain : &'s str, //this is not part of the official URI spec. But PasswordMaker Pro uses it.
    domain: &'s str,
    port: &'s str, //this would not need to be separated from path_query_fragment, but it's easier to parse if it's separate.
    path_query_fragment: &'s str //we don't need to separate those. Passwordmaker doesn't either.
}

impl<'s> UrlParts<'s> {
    fn filter_by_settings(self, settings : &UrlParsing) -> UsedUrlParts<'s>{
        let has_protocol = settings.is_protocol_used() && !self.protocol.is_empty();
        UsedUrlParts{
            protocol: //PasswordMaker Pro compatibility: Protocol is handled _weird_...
                if has_protocol { self.protocol }
                else if settings.use_protocol_undefined_fallback() { "undefined" }
                else { <&str>::default() },
            protocol_separator: if has_protocol { "://" } else { <&str>::default() }, //this is again some PasswordMaker Pro weirdness...
            userinfo: if settings.use_userinfo { self.userinfo } else { <&str>::default() },
            subdomain: if settings.use_subdomains { self.subdomain } else { <&str>::default() },
            domain: if settings.use_domain { self.domain } else { <&str>::default() },
            port: if settings.use_port_path { self.port } else { <&str>::default() },
            path_query_fragment: if settings.use_port_path { self.path_query_fragment } else { <&str>::default() },
        }
    }
}

#[cfg_attr(test, derive(PartialEq, Debug))]
struct UsedUrlParts<'s> {
    protocol : &'s str,
    protocol_separator : &'s str,
    userinfo : &'s str, //Treating this field separate is an addition to the functionaliyt offered by PasswordMaker Pro
    subdomain : &'s str, //this is not part of the official URI spec. But PasswordMaker Pro uses it.
    domain: &'s str,
    port: &'s str, //this would not need to be separated from path_query_fragment, but it's easier to parse if it's separate.
    path_query_fragment: &'s str //we don't need to separate those. Passwordmaker doesn't either.
}

impl<'s> UsedUrlParts<'s> {
    #[allow(clippy::doc_markdown)]
    /// Tries to do assemble a string in a way that's at least somehow compatible to PasswordMaker Pro.
    /// This prioritizes ease of use ("what the user expects") over correct URI parsing.
    fn recombine(self) -> String {
        //matching would need 64 arms... Too much work, soooo, a couple of ifs and less sanity instead.
        let has_userinfo = !self.userinfo.is_empty();
        let has_subdomain = !self.subdomain.is_empty();
        let has_domain = !self.domain.is_empty();
        let has_port = !self.port.is_empty();
        let has_path_query_fragment = !self.path_query_fragment.is_empty();
        
        //by doing all logic on &str, we save allocations to the very last moment. Also, the syntax is more readable.
        let parts = [
            self.protocol,
            self.protocol_separator,
            self.userinfo,
            if has_userinfo && (has_domain || has_subdomain || has_port|| has_path_query_fragment) { "@" } else { <&str>::default() },
            self.subdomain,
            if has_subdomain && has_domain { "." } else { <&str>::default() },
            self.domain,
            if has_port && (has_userinfo || has_domain || has_subdomain) { ":" } else { <&str>::default() },
            self.port,
            self.path_query_fragment,
        ];

        let needed_size = parts.iter().map(Deref::deref).map(<str>::len).sum();
        parts.iter().map(Deref::deref).fold(String::with_capacity(needed_size), String::add)
    }
}

#[allow(clippy::doc_markdown)]
/// Parses the input URI in a way that resembles the behaviour of PasswordMaker Pro. This is intentionally not following the URI standard.
/// It priorizes ease-of-use over strictly following the URI standard.
/// The idea here is that users tend to input strings of the form "www.somedomain.com", what is not a valid URI (authority is not optional).
/// Input of this form should still work though, in order not to confuse users.
fn parse_url(input : &str) -> UrlParts{
    let maybe_protocol = input.split_once(':');
    let has_protocol = maybe_protocol.is_some();
    let (protocol, rest) = maybe_protocol.unwrap_or((<&str>::default(), input));
    let removed_authority_marker = rest.strip_prefix("//");
    let has_authority = removed_authority_marker.is_some();
    let rest = removed_authority_marker.unwrap_or(rest);

    //Authority stops at first / character. Or, if none encountered, at end of input. Slash is part of path.
    //If there is a protocol, but no authority, we must treat everything after the intial ':' as path though.
    let first_character_of_path = if has_protocol && !has_authority {Some(0)} else {rest.find('/')};
    let (authority, path_query_fragment) = first_character_of_path.map_or((rest, <&str>::default()),|mid| rest.split_at(mid));
    //must split authority at '@' characters. Otherwise ':' is ambigious.
    let (userinfo, host_and_port) = authority.split_once('@').unwrap_or((<&str>::default(), authority));
    let (address, port) = host_and_port.split_once(':').unwrap_or((host_and_port, <&str>::default()));
    let separator_between_subdom_and_domain = address.rmatch_indices('.').nth(1);
    let (subdomain, domain_with_leading_dot) = separator_between_subdom_and_domain.map_or((<&str>::default(), address), |(i, _)| address.split_at(i));
    let domain = domain_with_leading_dot.strip_prefix('.').unwrap_or(domain_with_leading_dot);
    UrlParts{protocol, userinfo, subdomain, domain, port, path_query_fragment}
}

#[cfg(test)]
mod url_parsing_tests {
    use crate::ProtocolUsageMode;

    use super::*;

    /// Just tries to split some example urls and checks if the result is as expected. This tests against PasswordMaker Pro behaviour, not proper URI format.
    #[test]
    fn uri_splitting_test_full_uri(){
        let input = "http://anon:12345@some.subdomain.of.some.domain.com:8080/some/path/with?query&and#fragment";
        let expected = UrlParts{
            protocol: "http",
            userinfo: "anon:12345",
            subdomain: "some.subdomain.of.some",
            domain: "domain.com",
            port: "8080",
            path_query_fragment: "/some/path/with?query&and#fragment",
        };
        let result = parse_url(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn uri_splitting_test_no_userinfo(){
        let input = "http://some.subdomain.of.some.domain.com:8080/some/path/with?query&and#fragment";
        let expected = UrlParts{
            protocol: "http",
            userinfo: <&str>::default(),
            subdomain: "some.subdomain.of.some",
            domain: "domain.com",
            port: "8080",
            path_query_fragment: "/some/path/with?query&and#fragment",
        };
        let result = parse_url(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn uri_splitting_test_no_port(){  
        let input = "http://anon:12345@some.subdomain.of.some.domain.com/some/path/with?query&and#fragment";
        let expected = UrlParts{
            protocol: "http",
            userinfo: "anon:12345",
            subdomain: "some.subdomain.of.some",
            domain: "domain.com",
            port: <&str>::default(),
            path_query_fragment: "/some/path/with?query&and#fragment",
        };
        let result = parse_url(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn uri_splitting_test_no_domain(){  
        let input = "http://anon:12345@:8080/some/path/with?query&and#fragment";
        let expected = UrlParts{
            protocol: "http",
            userinfo: "anon:12345",
            subdomain: <&str>::default(),
            domain: <&str>::default(),
            port: "8080",
            path_query_fragment: "/some/path/with?query&and#fragment",
        };
        let result = parse_url(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn uri_splitting_test_no_domain_no_port(){  
        let input = "http://anon:12345@/some/path/with?query&and#fragment";
        let expected = UrlParts{
            protocol: "http",
            userinfo: "anon:12345",
            subdomain: <&str>::default(),
            domain: <&str>::default(),
            port: <&str>::default(),
            path_query_fragment: "/some/path/with?query&and#fragment",
        };
        let result = parse_url(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn uri_splitting_test_empty_path(){
        let input = "http://anon:12345@some.subdomain.of.some.domain.com:8080";
        let expected = UrlParts{
            protocol: "http",
            userinfo: "anon:12345",
            subdomain: "some.subdomain.of.some",
            domain: "domain.com",
            port: "8080",
            path_query_fragment: <&str>::default(),
        };
        let result = parse_url(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn uri_splitting_test_only_protocol_and_path(){
        let input = "http:some/path/";
        let expected = UrlParts{
            protocol: "http",
            userinfo: <&str>::default(),
            subdomain: <&str>::default(),
            domain: <&str>::default(),
            port: <&str>::default(),
            path_query_fragment: "some/path/",
        };
        let result = parse_url(input);
        assert_eq!(result, expected);
    }

    /// This triggers me. It should not work (scheme isn't optional), but users would miss it.
    /// Password and Port are not included too, because those would be (correctly) identified as schemes.
    #[test]
    fn uri_splitting_missing_protocol(){
        let input = "anon@some.subdomain.of.some.domain.com/some/path/with?query&and#fragment";
        let expected = UrlParts{
            protocol: <&str>::default(),
            userinfo: "anon",
            subdomain: "some.subdomain.of.some",
            domain: "domain.com",
            port: <&str>::default(),
            path_query_fragment: "/some/path/with?query&and#fragment",
        };
        let result = parse_url(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn uri_splitting_just_domain_and_path(){
        let input = "some.subdomain.of.some.domain.com/some/path/with?query&and#fragment";
        let expected = UrlParts{
            protocol: <&str>::default(),
            userinfo: <&str>::default(),
            subdomain: "some.subdomain.of.some",
            domain: "domain.com",
            port: <&str>::default(),
            path_query_fragment: "/some/path/with?query&and#fragment",
        };
        let result = parse_url(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn uri_splitting_just_domain_and_subdomain(){
        let input = "some.subdomain.of.some.domain.com";
        let expected = UrlParts{
            protocol: <&str>::default(),
            userinfo: <&str>::default(),
            subdomain: "some.subdomain.of.some",
            domain: "domain.com",
            port: <&str>::default(),
            path_query_fragment: <&str>::default(),
        };
        let result = parse_url(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn uri_splitting_just_domain(){
        let input = "domain.com";
        let expected = UrlParts{
            protocol: <&str>::default(),
            userinfo: <&str>::default(),
            subdomain: <&str>::default(),
            domain: "domain.com",
            port: <&str>::default(),
            path_query_fragment: <&str>::default(),
        };
        let result = parse_url(input);
        assert_eq!(result, expected);
    }
    #[test]
    fn uri_splitting_only_protocol(){
        let input = "ftp:";
        let expected = UrlParts{
            protocol: "ftp",
            userinfo: <&str>::default(),
            subdomain: <&str>::default(),
            domain: <&str>::default(),
            port: <&str>::default(),
            path_query_fragment: <&str>::default(),
        };
        let result = parse_url(input);
        assert_eq!(result, expected);
    }

    // Above tests are incomplete. I mean, there are 64 combinations... And then there could be errors...
    // Soo, let's just pretend it's fine, and if there are bugs, add the specific buggy input.
    //-----------------------------------------------------------------------------
    // Reassembly tests
    // Again our valid input range is 64 values... And again we just test some samples that are known to be weird.
    // For everything else, let's wait for bug reports.

    /// However, for settings application, every combination can be tested.
    #[test]
    fn apply_settings_to_url_parts_no_undefined_protocol(){
        for i in 0..64 {
            let settings = UrlParsing {
                use_protocol: if i%2 == 0 { ProtocolUsageMode::Used } else { ProtocolUsageMode::Ignored },
                use_userinfo: (i/2)%2 == 0,
                use_subdomains: (i/4)%2 == 0,
                use_domain: (i/8)%2 == 0,
                use_port_path: (i/16)%2 == 0,
            };
            
            let inputs = UrlParts {
                protocol: if (i/32)%2 == 0 {"proto"} else {""},
                userinfo: "plasmic",
                subdomain: "pirate",
                domain: "hordes",
                port: "420",
                path_query_fragment: "under/blackened#banners",
            };

            let output = inputs.clone().filter_by_settings(&settings);
            if settings.is_protocol_used() { assert_eq!(output.protocol, inputs.protocol) } else { assert_eq!(output.protocol, "") };
            if settings.is_protocol_used() && !inputs.protocol.is_empty() { assert_eq!(output.protocol_separator, "://") } else { assert_eq!(output.protocol_separator, "") };
            if settings.use_userinfo { assert_eq!(output.userinfo, inputs.userinfo) } else { assert_eq!(output.userinfo, "")};
            if settings.use_subdomains { assert_eq!(output.subdomain, inputs.subdomain) } else { assert_eq!(output.subdomain, "")};
            if settings.use_domain { assert_eq!(output.domain, inputs.domain) } else { assert_eq!(output.domain, "")};
            if settings.use_port_path { assert_eq!(output.port, inputs.port) } else { assert_eq!(output.port, "")};
            if settings.use_port_path { assert_eq!(output.path_query_fragment, inputs.path_query_fragment) } else { assert_eq!(output.path_query_fragment, "")};
        }
    }
    #[test]
    fn apply_settings_to_url_parts_undefined_protocol(){
        for i in 0..64 {
            let settings = UrlParsing {
                use_protocol: if i%2 == 0 { ProtocolUsageMode::UsedWithUndefinedIfEmpty } else { ProtocolUsageMode::Ignored },
                use_userinfo: (i/2)%2 == 0,
                use_subdomains: (i/4)%2 == 0,
                use_domain: (i/8)%2 == 0,
                use_port_path: (i/16)%2 == 0,
            };
            
            let inputs = UrlParts {
                protocol: if (i/32)%2 == 0 {"proto"} else {""},
                userinfo: "plasmic",
                subdomain: "pirate",
                domain: "hordes",
                port: "420",
                path_query_fragment: "under/blackened#banners",
            };

            let output = inputs.clone().filter_by_settings(&settings);
            if settings.is_protocol_used() { 
                if !inputs.protocol.is_empty() {
                    assert_eq!(output.protocol, inputs.protocol) 
                } else {
                    assert_eq!(output.protocol, "undefined")
                }
            } else { 
                assert_eq!(output.protocol, "") 
            };
            if settings.is_protocol_used() && !inputs.protocol.is_empty() { assert_eq!(output.protocol_separator, "://") } else { assert_eq!(output.protocol_separator, "") };
            if settings.use_userinfo { assert_eq!(output.userinfo, inputs.userinfo) } else { assert_eq!(output.userinfo, "")};
            if settings.use_subdomains { assert_eq!(output.subdomain, inputs.subdomain) } else { assert_eq!(output.subdomain, "")};
            if settings.use_domain { assert_eq!(output.domain, inputs.domain) } else { assert_eq!(output.domain, "")};
            if settings.use_port_path { assert_eq!(output.port, inputs.port) } else { assert_eq!(output.port, "")};
            if settings.use_port_path { assert_eq!(output.path_query_fragment, inputs.path_query_fragment) } else { assert_eq!(output.path_query_fragment, "")};
        }
    }

    #[test]
    fn recombine_full_url_test() {
        let input = UsedUrlParts{
            protocol: "xmpp",
            protocol_separator: "://",
            userinfo: "horst:12345",
            subdomain: "www",
            domain: "example.com",
            port: "8080",
            path_query_fragment: "/some/path",
        };
        let result = input.recombine();
        assert_eq!(result, "xmpp://horst:12345@www.example.com:8080/some/path");
    }
    #[test]
    fn recombine_user_but_no_subdomain() {
        let input = UsedUrlParts{
            protocol: "xmpp",
            protocol_separator: "://",
            userinfo: "horst:12345",
            subdomain: <&str>::default(),
            domain: "example.com",
            port: "8080",
            path_query_fragment: "/some/path",
        };
        let result = input.recombine();
        assert_eq!(result, "xmpp://horst:12345@example.com:8080/some/path");
    }
    #[test]
    fn recombine_no_user_but_subdomain() {
        let input = UsedUrlParts{
            protocol: "xmpp",
            protocol_separator: "://",
            userinfo: <&str>::default(),
            subdomain: "w3",
            domain: "example.com",
            port: "8080",
            path_query_fragment: "/some/path",
        };
        let result = input.recombine();
        assert_eq!(result, "xmpp://w3.example.com:8080/some/path");
    }
    #[test]
    fn recombine_no_user_no_subdomain() {
        let input = UsedUrlParts{
            protocol: "xmpp",
            protocol_separator: "://",
            userinfo: <&str>::default(),
            subdomain: <&str>::default(),
            domain: "example.com",
            port: "8080",
            path_query_fragment: "/some/path",
        };
        let result = input.recombine();
        assert_eq!(result, "xmpp://example.com:8080/some/path");
    }
    #[test]
    fn recombine_no_user_no_subdomain_no_port() {
        let input = UsedUrlParts{
            protocol: "xmpp",
            protocol_separator: "://",
            userinfo: <&str>::default(),
            subdomain: <&str>::default(),
            domain: "example.com",
            port: <&str>::default(),
            path_query_fragment: "/some/path",
        };
        let result = input.recombine();
        assert_eq!(result, "xmpp://example.com/some/path");
    }
    #[test]
    fn recombine_undefined_protocol() {
        let input = UsedUrlParts{
            protocol: "undefined",
            protocol_separator: <&str>::default(),
            userinfo: "horst:12345",
            subdomain: "www",
            domain: "example.com",
            port: "8080",
            path_query_fragment: "/some/path",
        };
        let result = input.recombine();
        assert_eq!(result, "undefinedhorst:12345@www.example.com:8080/some/path");
    }
    #[test]
    fn recombine_undefined_protocol_no_user_no_subdomain() {
        let input = UsedUrlParts{
            protocol: "undefined",
            protocol_separator: <&str>::default(),
            userinfo: <&str>::default(),
            subdomain: <&str>::default(),
            domain: "example.com",
            port: <&str>::default(),
            path_query_fragment: "/some/path",
        };
        let result = input.recombine();
        assert_eq!(result, "undefinedexample.com/some/path");
    }
    #[test]
    fn recombine_no_protocol() {
        let input = UsedUrlParts{
            protocol: <&str>::default(),
            protocol_separator: <&str>::default(),
            userinfo: <&str>::default(),
            subdomain: "www",
            domain: "example.com",
            port: <&str>::default(),
            path_query_fragment: "/some/path",
        };
        let result = input.recombine();
        assert_eq!(result, "www.example.com/some/path");
    }
    #[test]
    fn recombine_empty_path() {
        let input = UsedUrlParts{
            protocol: "xmpp",
            protocol_separator: "://",
            userinfo: "horst:12345",
            subdomain: "www",
            domain: "example.com",
            port: "8080",
            path_query_fragment: <&str>::default(),
        };
        let result = input.recombine();
        assert_eq!(result, "xmpp://horst:12345@www.example.com:8080");
    }
}
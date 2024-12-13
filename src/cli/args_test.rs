#[cfg(test)]
mod tests {
    use crate::cli::args::{Args, parse_url};
    use clap::Parser;

    #[test]
    fn test_parse_url_valid() {
        let url = "https://example.com";
        let result = parse_url(url);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().as_str(), "https://example.com/");
    }

    #[test]
    fn test_parse_url_invalid() {
        let url = "not a url";
        let result = parse_url(url);
        assert!(result.is_err());
    }

    #[test]
    fn test_args_default_port() {
        let args = Args::parse_from(&["program"]);
        assert_eq!(args.port, 3000);
        assert!(args.url.is_none());
        assert!(args.output.is_none());
    }

    #[test]
    fn test_args_with_url() {
        let url = "https://example.com";
        let args = Args::parse_from(&["program", url]);
        assert_eq!(args.port, 3000);
        assert_eq!(args.url.unwrap().as_str(), "https://example.com/");
        assert!(args.output.is_none());
    }

    #[test]
    fn test_args_with_output() {
        let args = Args::parse_from(&["program", "-o", "output.md"]);
        assert_eq!(args.port, 3000);
        assert!(args.url.is_none());
        assert_eq!(args.output.unwrap(), "output.md");
    }

    #[test]
    fn test_args_with_custom_port() {
        let args = Args::parse_from(&["program", "-P", "8080"]);
        assert_eq!(args.port, 8080);
        assert!(args.url.is_none());
        assert!(args.output.is_none());
    }
}

use logos::Logos;

#[derive(Debug, Logos, PartialEq)]
#[logos(skip r"[ \t\n\f]+", utf8 = false)] // Ignore this regex pattern between tokens
pub enum Token<'a> {
    #[regex(r"#[^\n]*", allow_greedy = true)]
    Comment,
    #[token(".")]
    Dot,
    #[regex(r#"<[^<>\"{}|^`\\\u{00}-\u{20}]*>"#)]
    Iri(&'a [u8]),
    #[regex(r"_:(?:(?:[A-Za-z\u{00C0}-\u{00D6}\u{00D8}-\u{00F6}\u{00F8}-\u{02FF}\u{0370}-\u{037D}\u{037F}-\u{1FFF}\u{200C}-\u{200D}\u{2070}-\u{218F}\u{2C00}-\u{2FEF}\u{3001}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFFD}\u{10000}-\u{EFFFF}_])|(?:[0-9]))(?:[A-Za-z\u{00C0}-\u{00D6}\u{00D8}-\u{00F6}\u{00F8}-\u{02FF}\u{0370}-\u{037D}\u{037F}-\u{1FFF}\u{200C}-\u{200D}\u{2070}-\u{218F}\u{2C00}-\u{2FEF}\u{3001}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFFD}\u{10000}-\u{EFFFF}_0-9\u{00B7}\u{0300}-\u{036F}\u{203F}-\u{2040}\.-]*)?")]
    BlankNode(&'a [u8]),
    #[regex(r#""[^\u{27}\u{5C}\u{A}\u{D}"]*"(\^\^<[^<>\"{}|^`\\\u{00}-\u{20}]*>|@en)?"#)]
    Literal(&'a [u8]),
}

#[cfg(test)]
mod test {
    use super::Token;
    use logos::Logos;

    #[test]
    fn tokenize_triple() {
        let tokens = Token::lexer(br#"_:a <iri> "strings"@en"#);
        assert_eq!(
            tokens.into_iter().map(|token| token).collect::<Vec<_>>(),
            vec![
                Ok(Token::BlankNode(b"_:a")),
                Ok(Token::Iri(b"<iri>")),
                Ok(Token::Literal(br#""strings"@en"#))
            ]
        );
    }

    #[test]
    fn tokenize_blanknode() {
        let tokens = Token::lexer(br#"_:a _:n1 _:asda"#);
        assert_eq!(
            tokens.into_iter().map(|token| token).collect::<Vec<_>>(),
            vec![
                Ok(Token::BlankNode(b"_:a")),
                Ok(Token::BlankNode(b"_:n1")),
                Ok(Token::BlankNode(b"_:asda"))
            ]
        );
    }

    #[test]
    fn tokenize_iri() {
        let tokens = Token::lexer(
            br#"<asdasdsa> <http://www.wikidata.org/prop/> <http://www.wikidata.org/prop/statement/value-normalized/>"#,
        );
        assert_eq!(
            tokens.into_iter().map(|token| token).collect::<Vec<_>>(),
            vec![
                Ok(Token::Iri(b"<asdasdsa>")),
                Ok(Token::Iri(b"<http://www.wikidata.org/prop/>")),
                Ok(Token::Iri(
                    b"<http://www.wikidata.org/prop/statement/value-normalized/>"
                ))
            ]
        );
    }

    #[test]
    fn tokenize_literal() {
        let tokens = Token::lexer(br#""simple string" "hello"@en "x"^^<asdadasd>"#);
        assert_eq!(
            tokens.into_iter().map(|token| token).collect::<Vec<_>>(),
            vec![
                Ok(Token::Literal(br#""simple string""#)),
                Ok(Token::Literal(br#""hello"@en"#)),
                Ok(Token::Literal(br#""x"^^<asdadasd>"#))
            ]
        );
    }
}

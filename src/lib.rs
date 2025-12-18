mod lexer;

use lexer::Token;
use logos::Logos;

pub struct Triple<'a>(pub &'a [u8], pub &'a [u8], pub &'a [u8]);

pub fn parse<'a>(input: &'a [u8]) -> Result<Vec<Triple<'a>>, ()> {
    let tokens = Token::lexer(input);
    let mut triples = Vec::new();
    let mut counter: u8 = 0;
    let mut subject: &[u8] = &[];
    let mut predicate: &[u8] = &[];
    for token in tokens {
        let token = token.unwrap();
        match (counter, token) {
            (0, Token::Iri(bytes) | Token::BlankNode(bytes)) => {
                subject = bytes;
            }
            (1, Token::Iri(bytes)) => {
                predicate = bytes;
            }
            (2, Token::Literal(bytes) | Token::Iri(bytes)) => {
                triples.push(Triple(subject, predicate, bytes))
            }
            _ => {
                panic!()
            }
        }
        counter = (counter + 1) % 3;
    }
    return Ok(triples);
}

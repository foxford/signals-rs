use nom;

type NomError<'a> = nom::Err<nom::types::CompleteStr<'a>>;

#[derive(Fail, Debug)]
#[fail(display = "Parse error: {:?}", _0)]
pub struct ParseError(nom::ErrorKind);

impl<'a> From<NomError<'a>> for ParseError {
    fn from(e: NomError<'a>) -> Self {
        let kind = e.into_error_kind();
        ParseError(kind)
    }
}

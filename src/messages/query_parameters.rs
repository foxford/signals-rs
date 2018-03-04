use std::fmt;
use std::str::FromStr;

use nom::multispace;
use nom::types::CompleteStr;
use serde::{Deserialize, Deserializer};
use uuid::Uuid;

use errors::Error;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QueryParameters {
    #[serde(default)]
    #[serde(deserialize_with = "deserialize_empty_as_none")]
    pub fq: Option<String>,
}

fn deserialize_empty_as_none<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(d).map(|string| {
        if string.is_empty() {
            None
        } else {
            Some(string)
        }
    })
}

#[derive(PartialEq)]
pub enum Expr {
    Value(Filter),
    Or(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Parenthesis(Box<Expr>),
}

impl fmt::Debug for Expr {
    fn fmt(&self, format: &mut fmt::Formatter) -> fmt::Result {
        use self::Expr::*;
        match *self {
            Value(ref val) => write!(format, "{:?}", val),
            Or(ref left, ref right) => write!(format, "({:?} OR {:?})", left, right),
            And(ref left, ref right) => write!(format, "({:?} AND {:?})", left, right),
            Parenthesis(ref expr) => write!(format, "[{:?}]", expr),
        }
    }
}

impl FromStr for Expr {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, ex) = expr(CompleteStr(s))?;
        Ok(ex)
    }
}

#[derive(Debug)]
pub enum Oper {
    Or,
    And,
}

#[derive(Debug, PartialEq)]
pub enum Filter {
    RoomId(Uuid),
    OwnerId(Uuid),
    HolderId(Uuid),
}

named!(room_filter<CompleteStr, Filter>, preceded!(
    tag_s!("room_id:"),
    map!(
        map_res!(take_s!(36), |s: CompleteStr| Uuid::from_str(s.0)),
        Filter::RoomId
    )
));

named!(owner_filter<CompleteStr, Filter>, preceded!(
    tag_s!("owner_id:"),
    map!(
        map_res!(take_s!(36), |s: CompleteStr| FromStr::from_str(s.0)),
        Filter::OwnerId
    )
));

named!(holder_filter<CompleteStr, Filter>, preceded!(
    tag_s!("holders.id:"),
    map!(
        map_res!(take_s!(36), |s: CompleteStr| FromStr::from_str(s.0)),
        Filter::HolderId
    )
));

named!(filter<CompleteStr, Filter>, alt!(
    room_filter | owner_filter | holder_filter
));

named!(parenthesis<CompleteStr, Expr>, delimited!(
    delimited!(opt!(multispace), tag!("("), opt!(multispace)),
    map!(map!(expr, Box::new), Expr::Parenthesis),
    delimited!(opt!(multispace), tag!(")"), opt!(multispace))
));

named!(factor<CompleteStr, Expr>, alt_complete!(
    map!(
        delimited!(opt!(multispace), filter, opt!(multispace)),
        Expr::Value
    )
    | parenthesis
));

fn fold_exprs(initial: Expr, remainder: Vec<(Oper, Expr)>) -> Expr {
    remainder.into_iter().fold(initial, |acc, pair| {
        let (oper, expr) = pair;
        match oper {
            Oper::Or => Expr::Or(Box::new(acc), Box::new(expr)),
            Oper::And => Expr::And(Box::new(acc), Box::new(expr)),
        }
    })
}

named!(term<CompleteStr, Expr>, do_parse!(
    initial: factor >>
    remainder: many0!(
        do_parse!(tag!("AND") >> mul: factor >> (Oper::And, mul))
    ) >>
    (fold_exprs(initial, remainder))
));

named!(expr<CompleteStr, Expr>, do_parse!(
    initial: term >>
    remainder: many0!(
        do_parse!(tag!("OR") >> add: term >> (Oper::Or, add))
    ) >>
    (fold_exprs(initial, remainder))
));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_query_params_with_fq() {
        use serde_json;

        let json = r#"{ "fq": "123" }"#;
        let params = serde_json::from_str::<QueryParameters>(&json);

        assert!(params.is_ok());
        assert_eq!(params.unwrap().fq, Some("123".to_owned()));
    }

    #[test]
    fn deserialize_query_params_with_empty_fq() {
        use serde_json;

        let json = r#"{ "fq": "" }"#;
        let params = serde_json::from_str::<QueryParameters>(&json);

        assert!(params.is_ok());
        assert_eq!(params.unwrap().fq, None);
    }

    #[test]
    fn deserialize_query_params_without_fq() {
        use serde_json;

        let json = r#"{}"#;
        let params = serde_json::from_str::<QueryParameters>(&json);

        assert!(params.is_ok());
        assert_eq!(params.unwrap().fq, None);
    }

    #[test]
    fn parse_factor() {
        let room_id = Uuid::parse_str("7945cf5b-2c73-4936-80cb-5cce27e9950d").unwrap();
        let input = format!("  room_id:{}  ", room_id);

        assert_eq!(
            factor(CompleteStr(&input)),
            Ok((CompleteStr(""), Expr::Value(Filter::RoomId(room_id))))
        );
    }

    #[test]
    fn parse_term() {
        let room_id = Uuid::parse_str("7945cf5b-2c73-4936-80cb-5cce27e9950d").unwrap();
        let owner_id = Uuid::parse_str("55e813bc-0c9b-4270-9f7f-81e5ffcfc9ff").unwrap();

        let input = format!(" room_id:{} AND  owner_id:{}   ", room_id, owner_id);
        let expected = Expr::And(
            Box::new(Expr::Value(Filter::RoomId(room_id))),
            Box::new(Expr::Value(Filter::OwnerId(owner_id))),
        );

        assert_eq!(term(CompleteStr(&input)), Ok((CompleteStr(""), expected)));
    }

    #[test]
    fn parse_expr() {
        let room_id = Uuid::parse_str("7945cf5b-2c73-4936-80cb-5cce27e9950d").unwrap();
        let owner_id = Uuid::parse_str("55e813bc-0c9b-4270-9f7f-81e5ffcfc9ff").unwrap();
        let holder_id = Uuid::parse_str("30ae48f8-57c1-4704-ba48-19edb3c22b09").unwrap();

        let input = format!(
            " owner_id:{}  OR holders.id:{} AND room_id:{} ",
            owner_id, holder_id, room_id
        );
        let expected = Expr::Or(
            Box::new(Expr::Value(Filter::OwnerId(owner_id))),
            Box::new(Expr::And(
                Box::new(Expr::Value(Filter::HolderId(holder_id))),
                Box::new(Expr::Value(Filter::RoomId(room_id))),
            )),
        );

        assert_eq!(expr(CompleteStr(&input)), Ok((CompleteStr(""), expected)));
    }

    #[test]
    fn parse_parenthesis() {
        let room_id = Uuid::parse_str("7945cf5b-2c73-4936-80cb-5cce27e9950d").unwrap();
        let owner_id = Uuid::parse_str("55e813bc-0c9b-4270-9f7f-81e5ffcfc9ff").unwrap();
        let holder_id = Uuid::parse_str("30ae48f8-57c1-4704-ba48-19edb3c22b09").unwrap();

        let input = format!(
            " ( owner_id:{}  OR holders.id:{} ) AND room_id:{} ",
            owner_id, holder_id, room_id
        );
        let expected = Expr::And(
            Box::new(Expr::Parenthesis(Box::new(Expr::Or(
                Box::new(Expr::Value(Filter::OwnerId(owner_id))),
                Box::new(Expr::Value(Filter::HolderId(holder_id))),
            )))),
            Box::new(Expr::Value(Filter::RoomId(room_id))),
        );

        assert_eq!(expr(CompleteStr(&input)), Ok((CompleteStr(""), expected)));
    }

    #[test]
    fn expr_from_str() {
        let room_id = Uuid::parse_str("7945cf5b-2c73-4936-80cb-5cce27e9950d").unwrap();
        let input = format!("room_id:{}", room_id);
        let ex = Expr::from_str(&input);

        assert!(ex.is_ok());
        assert_eq!(ex.unwrap(), Expr::Value(Filter::RoomId(room_id)));

        let input = format!("room:{}", room_id);
        assert!(Expr::from_str(&input).is_err())
    }
}

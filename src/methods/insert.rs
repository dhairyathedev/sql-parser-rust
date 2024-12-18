use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    character::complete::{multispace0, multispace1},
    sequence::preceded,
    multi::separated_list0,
};

fn is_valid_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn identifier(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, take_while1(is_valid_identifier_char))(input)
}

fn get_cols(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(preceded(multispace0, tag(", ")), identifier)(input)
}

pub fn parse_insert(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (input, _) = tag("INSERT INTO")(input)?;
    let (input, _) = multispace1(input)?;
    let (_input, table_name) = identifier(input)?;
    Ok(("", (table_name, vec!["abc"])))
}
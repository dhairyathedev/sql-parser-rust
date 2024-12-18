use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{multispace0, multispace1},
    multi::separated_list0,
    sequence::preceded,
    IResult,
};

fn is_valid_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn identifier(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, take_while1(is_valid_identifier_char))(input)
}

fn column_list(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(preceded(multispace0, tag(",")), identifier)(input)
}

pub fn parse_select(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    let (input, _) = tag("SELECT")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, cols) = column_list(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("FROM")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, table) = identifier(input)?;
    Ok((input, (table, cols)))
}

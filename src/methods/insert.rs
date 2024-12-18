use nom::{
    bytes::complete::{is_not, tag, take_while1},
    character::complete::{multispace0, multispace1},
    multi::separated_list0,
    sequence::{delimited, preceded},
    IResult,
};

fn is_valid_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn identifier(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, take_while1(is_valid_identifier_char))(input)
}

fn get_cols(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        tag("("),
        separated_list0(preceded(multispace0, tag(",")), identifier),
        tag(")"),
    )(input)
}

fn parse_value(input: &str) -> IResult<&str, &str> {
    preceded(multispace0, is_not(",)"))(input)
}

fn parse_values(input: &str) -> IResult<&str, Vec<&str>> {
    delimited(
        tag("("),
        separated_list0(preceded(multispace0, tag(",")), parse_value),
        tag(")"),
    )(input)
}

pub fn parse_insert(input: &str) -> IResult<&str, (&str, Vec<&str>, Vec<&str>)> {
    let (input, _) = tag("INSERT INTO")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, table_name) = identifier(input)?;
    let (input, _) = multispace1(input)?;
    // let (input, _) = tag("(")(input)?;
    let (input, cols) = get_cols(input)?;
    // let (input, _) = tag(")")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, _) = tag("VALUES")(input)?;
    let (input, _) = multispace1(input)?;
    let (input, vals) = parse_values(input)?;
    Ok(("", (table_name, cols, vals)))
}
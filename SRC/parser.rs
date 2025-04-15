// src/parser.rs
use nom::{
    bytes::complete::{tag, tag_no_case},
    character::complete::{alpha1, multispace0},
    multi::separated_list0,
    sequence::delimited,
    IResult,
};
pub fn parse_from(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag_no_case("FROM")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, table_name) = alpha1(input)?;
    Ok((input, table_name))
}

pub fn parse_select(input: &str) -> IResult<&str, &str> {
    tag_no_case("SELECT")(input)
}

pub fn parse_columns(input: &str) -> IResult<&str, Vec<String>> {
    let comma_separator = delimited(multispace0, tag(","), multispace0);

    separated_list0(comma_separator, alpha1)(input).map(|(next_input, columns)| {
        let columns = columns.into_iter().map(|col| col.to_string()).collect();
        (next_input, columns)
    })
}
use nom::{
    branch::alt,
    character::complete::{digit1, space1},
    combinator::map_res,
    sequence::tuple,
};

pub fn parse_where(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let (input, _) = tag_no_case("WHERE")(input)?;
    let (input, _) = space1(input)?;

    // Parses patterns like: column = value
    let (input, (left, _, op, _, right)) = tuple((
        alpha1,
        space1,
        alt((tag("="), tag("!="), tag("<"), tag(">"))),
        space1,
        alt((alpha1, digit1)),
    ))(input)?;

    Ok((input, (left, op, right)))
}

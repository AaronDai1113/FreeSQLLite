use nom::{
    IResult,
    sequence::{tuple, preceded, terminated},
    combinator::{map, opt},
    bytes::complete::{tag, tag_no_case},
    character::complete::{alphanumeric1 as alphanumeric, multispace1 as space},
    branch::alt,
};

// Define a struct for your query - this is a simplified example
#[derive(Debug, PartialEq)]
pub struct SelectQuery {
    pub columns: Vec<String>,
    pub table: String,
}

// Parse identifiers like table names, column names
fn identifier(input: &str) -> IResult<&str, String> {
    map(alphanumeric, String::from)(input)
}

// Parse a comma-separated list of identifiers
fn identifier_list(input: &str) -> IResult<&str, Vec<String>> {
    nom::multi::separated_list0(preceded(opt(space), tag(",")), identifier)(input)
}

// Parse a SELECT query
pub fn parse_select(input: &str) -> IResult<&str, SelectQuery> {
    let (input, _) = tag_no_case("SELECT")(input)?;
    let (input, _) = space(input)?;
    let (input, columns) = identifier_list(input)?;
    let (input, _) = space(input)?;
    let (input, _) = tag_no_case("FROM")(input)?;
    let (input, _) = space(input)?;
    let (input, table) = identifier(input)?;

    Ok((input, SelectQuery { columns, table }))
}

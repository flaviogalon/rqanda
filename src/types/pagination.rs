use std::collections::HashMap;

use crate::error;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

pub fn extract_pagination(
    params: HashMap<String, String>,
    limit: usize,
) -> Result<Pagination, error::Error> {
    if !params.contains_key("start") || !params.contains_key("end") {
        return Err(error::Error::MissingParameters);
    }

    let start: usize = params
        .get("start")
        .unwrap()
        .parse::<usize>()
        .map_err(error::Error::ParseError)?;
    let mut end: usize = params
        .get("end")
        .unwrap()
        .parse::<usize>()
        .map_err(error::Error::ParseError)?;

    if start > end {
        return Err(error::Error::InvertedOrder);
    } else if end > limit {
        end = limit;
    }

    return Ok(Pagination {
        start: start,
        end: end,
    });
}

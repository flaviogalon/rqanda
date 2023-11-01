use std::collections::HashMap;

use crate::error;

/// Pagination struct that is getting extracted from query parameters
#[derive(Debug)]
pub struct Pagination {
    /// Index of the first item that has to be returned
    pub start: usize,
    /// Index of the last item that has to be returned
    pub end: usize,
}

impl std::fmt::Display for Pagination {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "start: {}, end: {}", self.start, self.end)
    }
}

/** Extract query parameters from the `/questions` route
    ## Example query
    GET requests to this route can have a pagination attached so we just return
    the questions we need

    `/questions?start=1&end=10`

    ## Example usage
    ```rust
    let mut query = HashMap::new();
    query.insert("start".to_string(), "1".to_string());
    query.inser("end".to_string(), "10".to_string());
    let p = types::pagination::extract_pagination(query).unwrap();
    assert_eq!(p.start, 1);
    assert_eq!(p.end, 10);
    ```
*/
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

    Ok(Pagination { start, end })
}

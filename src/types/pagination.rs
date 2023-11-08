use std::collections::HashMap;

use crate::error;

/// Pagination struct that is getting extracted from query parameters
#[derive(Debug, Default)]
pub struct Pagination {
    /// Index of the first item that has to be returned
    pub offset: i32,
    /// Index of the last item that has to be returned
    pub limit: Option<i32>,
}

impl std::fmt::Display for Pagination {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "start: {}, end: {:?}", self.offset, self.limit)
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
    query.insert("limit".to_string(), "1".to_string());
    query.inser("offset".to_string(), "10".to_string());
    let p = types::pagination::extract_pagination(query).unwrap();
    assert_eq!(p.limit, Some(1));
    assert_eq!(p.offset, 10);
    ```
*/
pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, error::Error> {
    if !params.contains_key("start") || !params.contains_key("end") {
        return Err(error::Error::MissingParameters);
    }

    let limit: Option<i32> = Some(
        params
            .get("limit")
            .unwrap()
            .parse::<i32>()
            .map_err(error::Error::ParseError)?,
    );
    let mut offset: i32 = params
        .get("offset")
        .unwrap()
        .parse::<i32>()
        .map_err(error::Error::ParseError)?;

    if limit.is_some() && limit.unwrap() > offset {
        return Err(error::Error::InvertedOrder);
    }

    Ok(Pagination { limit, offset })
}

struct Json {
    body: String,
    field1: u32,
}

fn get_body(Json{body, field1}: Json) -> String {
    body
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_body() {
        let message: String = "Hi".to_string();
        let json = Json{body: message, field1: 0};
        // message String is moved into the Json struct

        let body = get_body(json);

        let message = "Hi".to_string();

        assert_eq!(body, message);
    }
}

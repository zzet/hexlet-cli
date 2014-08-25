pub fn get_url(user_type: &str, path: &str) -> Url {
    match user_type {
        "teacher" => {
            let urls = teacher_urls();
            let request_path = urls.find(path);
            return prepare_url(request_path);
        },

        "student" => {
            let urls = student_urls();
            let request_path = urls.find(path);
            return prepare_url(request_path);
        },
        _ => { return Url }
    }
}

fn teacher_urls() -> HashMap<&str, &str> {
    let mut h = HashMap::new();

    // Fill map with availabled routes
    h.insert("check_auth", "user/check_auth.json");

    return h
}

fn student_urls() -> HashMap<&str, &str> {
    let mut h = HashMap::new();

    // Fill map with availabled routes
    h.insert("check_auth", "user/check_auth.json");

    return h
}


fn prepre_url(path: &str) -> Url {
    Url {
        scheme: "http".to_string(),
        host: "hexlet.io".to_string(),
        port: Some("80".to_string()),
        path: path.to_string()
    }
}

extern crate httpd = "tiny-http";

fn get_content_type(path: &Path) -> &'static str {
    let extension = match path.extension_str() {
        None => return "text/plain",
        Some(e) => e
    };

    match extension {
        "gif" => "image/gif",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "pdf" => "application/pdf",
        "htm" => "text/html; charset=utf8",
        "html" => "text/html; charset=utf8",
        "txt" => "text/plain; charset=utf8",
        _ => "text/plain; charset=utf8"
    }
}

fn main() {
    let server = httpd::ServerBuilder::new().with_random_port().build().unwrap();
    let port = server.get_server_addr().port;
    println!("Now listening on port {}", port);

    loop {
        let rq = match server.recv() {
            Ok(rq) => rq,
            Err(_) => break
        };

        println!("{}", rq);

        let path = Path::new(rq.get_url());
        let file = std::io::File::open(&path);

        if file.is_ok() {
            use std::ascii::AsciiCast;

            let response = httpd::Response::from_file(file.unwrap());

            let response = response.with_header(
                httpd::Header {
                    field: from_str("Content-Type").unwrap(),
                    value: get_content_type(&path).to_ascii().to_vec()
                }
            );

            rq.respond(response);

        } else {
            let rep = httpd::Response::new_empty(httpd::StatusCode(404));
            rq.respond(rep);
        }
    }
}

/// helper function to check if a request is HTTP or not
pub fn request_is_http(mut request: &Vec<String>) -> bool {
    let http_request_start_line_rules = vec![
        "GET", "POST", "TRACE", "PUT", "HEAD", "CONNECT", "DELETE", "PATCH",
    ];

    for rule in http_request_start_line_rules {
        if request[0].contains(rule) {
            println!("hahahaha that is an HTTP request congrats !!! ╰(*°▽°*)╯");
            return true;
        }
    }
    println!("VALIDITY IS false");

    false
}

#[cfg(test)]
mod tests {
    use crate::http::utils::request_is_http;

    #[test]
    fn GET_request() {
        let reqGET: Vec<String> = vec![
            "GET / HTTP/1.1",
            "Host: localhost:3000",
            "Connection: keep-alive",
            "Cache-Control: max-age=0",
            "sec-ch-ua: \"Microsoft Edge\";v=\"137\", \"Chromium\";v=\"137\", \"Not/A)Brand\";v=\"24\"",
            "sec-ch-ua-mobile: ?0",
            "sec-ch-ua-platform: \"Windows\"",
            "Upgrade-Insecure-Requests: 1",
            "User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/137.0.0.0 Safari/537.36 Edg/137.0.0.0",
            "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7",
            "Sec-Fetch-Site: none",
            "Sec-Fetch-Mode: navigate",
            "Sec-Fetch-User: ?1",
            "Sec-Fetch-Dest: document",
            "Accept-Encoding: gzip, deflate, br, zstd",
            "Accept-Language: en-US,en;q=0.9",
        ].iter().map(|line|line.to_string()).to_owned().collect();
        // let reqPOST = "";
        // let reqPUT = "";
        println!("{}", reqGET[0]);
        assert_eq!(true, request_is_http(&reqGET));
        // assert_eq!(true, request_is_http(vec!["no".to_string()]));
    }
}

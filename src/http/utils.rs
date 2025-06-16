/// helper function to check if a request is HTTP or not
pub fn request_is_http(mut request: Vec<String>) -> bool {
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
            "Cookie: __clerk_db_jwt=dvb_2y13MEX0tGL7RWi82f4JEqfsD8d; __clerk_db_jwt_YS3PnrMS=dvb_2y13MEX0tGL7RWi82f4JEqfsD8d; __refresh_YS3PnrMS=zhR6JbrAq76Hu1M3CLbS; __session=eyJhbGciOiJSUzI1NiIsImNhdCI6ImNsX0I3ZDRQRDExMUFBQSIsImtpZCI6Imluc18ycEpDSERrZmM3Zk5GdW1OOXBxWVJ5ZlV0c1ciLCJ0eXAiOiJKV1QifQ.eyJhenAiOiJodHRwOi8vbG9jYWxob3N0OjMwMDAiLCJleHAiOjE3NDg5ODg5MzMsImZ2YSI6WzgsLTFdLCJpYXQiOjE3NDg5ODg4NzMsImlzcyI6Imh0dHBzOi8vZXRlcm5hbC1kcnVtLTg2LmNsZXJrLmFjY291bnRzLmRldiIsIm5iZiI6MTc0ODk4ODg2Mywic2lkIjoic2Vzc18yeTEzZ3hhZ0FUSG9UUm94TDhpYTZUajROM2QiLCJzdWIiOiJ1c2VyXzJwSklzQnNGOTlhNmVSTUhuTE1xNzNsUTY2dCJ9.K4DIA3LuBAiEhidw9Fw_tGtzEetxMls8oS_fkTPUqplXp5Vmy5STvLd9iuHPef9ESTT-xciWgVPZBiEf9cvzmR9QX3XILFlNiuJFIJ9C6pvzbTc1P334UnO_-6hZ8v92hXeF9PBMfokjFqyhcLVgQbNQ4q8TfouNNcvnDUd82qptpKGxmFteHY2BiJno2bQfDMlLd-uTFEuhqLIlBHufDrcedpwqShdcDvCk0qXqNRcijzzGYifdOGvn7w__dvgC0gjF4Vmbo47D4MboqnyKJjLKqqseQ4UN3bq-ZPpaSxExSUSW2_PZzBfU35EtC1v8_C8NhHH4GBYggDhw8-v5UQ; __session_YS3PnrMS=eyJhbGciOiJSUzI1NiIsImNhdCI6ImNsX0I3ZDRQRDExMUFBQSIsImtpZCI6Imluc18ycEpDSERrZmM3Zk5GdW1OOXBxWVJ5ZlV0c1ciLCJ0eXAiOiJKV1QifQ.eyJhenAiOiJodHRwOi8vbG9jYWxob3N0OjMwMDAiLCJleHAiOjE3NDg5ODg5MzMsImZ2YSI6WzgsLTFdLCJpYXQiOjE3NDg5ODg4NzMsImlzcyI6Imh0dHBzOi8vZXRlcm5hbC1kcnVtLTg2LmNsZXJrLmFjY291bnRzLmRldiIsIm5iZiI6MTc0ODk4ODg2Mywic2lkIjoic2Vzc18yeTEzZ3hhZ0FUSG9UUm94TDhpYTZUajROM2QiLCJzdWIiOiJ1c2VyXzJwSklzQnNGOTlhNmVSTUhuTE1xNzNsUTY2dCJ9.K4DIA3LuBAiEhidw9Fw_tGtzEetxMls8oS_fkTPUqplXp5Vmy5STvLd9iuHPef9ESTT-xciWgVPZBiEf9cvzmR9QX3XILFlNiuJFIJ9C6pvzbTc1P334UnO_-6hZ8v92hXeF9PBMfokjFqyhcLVgQbNQ4q8TfouNNcvnDUd82qptpKGxmFteHY2BiJno2bQfDMlLd-uTFEuhqLIlBHufDrcedpwqShdcDvCk0qXqNRcijzzGYifdOGvn7w__dvgC0gjF4Vmbo47D4MboqnyKJjLKqqseQ4UN3bq-ZPpaSxExSUSW2_PZzBfU35EtC1v8_C8NhHH4GBYggDhw8-v5UQ; __client_uat_YS3PnrMS=1748988366; __client_uat=1748988366",
        ].iter().map(|line|line.to_string()).collect();
        // let reqPOST = "";
        // let reqPUT = "";
        println!("{}", reqGET[0]);
        assert_eq!(true, request_is_http(reqGET));
        // assert_eq!(true, request_is_http(vec!["no".to_string()]));
    }
}

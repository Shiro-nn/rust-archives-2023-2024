const TGBOT: &str = "";
const TGCHAT: &str = "";

fn main() {
    send_tg(&"test".to_string());
}

pub fn send_tg(content: &String) {
    if content.is_empty() {
        return;
    }

    let mut body = String::from("{\"text\":\"");
    body.push_str(&content);
    body.push_str("\",\"parse_mode\":\"Markdown\",\"disable_web_page_preview\":true");
    body.push_str(",\"disable_notification\":false,\"reply_to_message_id\":null,\"chat_id\":\"");
    body.push_str(crate::TGCHAT);
    body.push_str("\"}");

    match minreq::post(format!("http://api.telegram.org/bot{}/sendMessage", crate::TGBOT))
    .with_header("accept", "application/json")
    .with_header("content-type", "application/json")
    .with_body(body)
    .send() {
        Ok(_) => {},
        Err(err) => panic!("{}", err),
    }
}

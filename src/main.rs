fn main() {
    let sample_irc_message =
        "@id=234AB;test=;keywithnovalue; :dan!d@localhost PRIVMSG #chan :Hey what's up!\r\n";
    let (tag_string, _message) = get_tag_substring(sample_irc_message);

    let tags = get_tags(tag_string);

    println!("------");
    for tag in &tags {
        let IRCMessageToken::Tag(key, value) = tag;
        println!("{} -:- {}", key, value);
    }
}

enum IRCMessageToken<'a> {
    Tag(&'a str, &'a str),
}

fn get_tag_substring<'a>(message: &'a str) -> (&'a str, &'a str) {
    if let Some(pos) = message.find(' ') {
        return (&message[..pos], &message[pos + ' '.len_utf8()..]);
    }

    ("", message)
}

fn get_tags<'a>(message: &'a str) -> Vec<IRCMessageToken> {
    let key_value_pairs = message.split(";");
    let mut tokens: Vec<IRCMessageToken> = Vec::new();
    for kvp in key_value_pairs {
        if let Some(token) = create_token(kvp) {
            tokens.push(token);
        }
    }

    tokens
}

fn create_token<'a>(message: &'a str) -> Option<IRCMessageToken> {
    if message.is_empty() {
        return None;
    }

    let kvps: Vec<&'a str> = message.split("=").collect();
    if kvps.len() < 2 {
        return Some(IRCMessageToken::Tag(kvps[0], ""));
    }

    Some(IRCMessageToken::Tag(kvps[0], kvps[1]))
}

#[cfg(test)]
mod tests {
    use crate::{create_token, get_tag_substring, get_tags, IRCMessageToken};

    #[test]
    fn get_tag_substring_from_string() {
        let token_string =
            "@id=234AB;test=;keywithnovalue; :dan!d@localhost PRIVMSG #chan :Hey what's up!\r\n";
        let (tag_string, remainder) = get_tag_substring(token_string);
        assert_eq!(tag_string, "@id=234AB;test=;keywithnovalue;");
        assert_eq!(
            remainder,
            ":dan!d@localhost PRIVMSG #chan :Hey what's up!\r\n"
        );
    }

    #[test]
    fn create_tag_token_from_string() {
        let token_string = "@id=234AB";
        let IRCMessageToken::Tag(key, value) = create_token(token_string).unwrap();
        assert_eq!(key, "@id");
        assert_eq!(value, "234AB");
    }

    #[test]
    fn create_tag_token_from_empty_string() {
        let token_string = "";
        let token = create_token(token_string);
        assert!(token.is_none());
    }

    #[test]
    fn create_tag_tokens() {
        let sample_irc_message =
            "@id=234AB;test=;keywithnovalue; :dan!d@localhost PRIVMSG #chan :Hey what's up!\r\n";
        let (tag_string, _message) = get_tag_substring(sample_irc_message);

        let tags = get_tags(tag_string);
        assert_eq!(tags.len(), 3);
        let IRCMessageToken::Tag(e1, v1) = tags[0];
        assert_eq!(e1, "@id");
        assert_eq!(v1, "234AB");
        let IRCMessageToken::Tag(e2, v2) = tags[1];
        assert_eq!(e2, "test");
        assert_eq!(v2, "");
        let IRCMessageToken::Tag(e3, v3) = tags[2];
        assert_eq!(e3, "keywithnovalue");
        assert_eq!(v3, "");
    }
}

fn main() {
    let sample_irc_message =
        "@id=234AB;test=;keywithnovalue; :dan!d@localhost PRIVMSG #chan :Hey what's up!\r\n";
    //
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
        return (&message[..pos], &message[pos..]);
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

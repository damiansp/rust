fn main() {
    if let Some(cookie) = request.session_cookie {
        return restore_session(cookie);
    }

    if let Err(err) = show_anti_bot_task() {
        log_bot_attempt(err);
        politely_accuse_user_of_being_a_bot();
    } else {
        session.mark_as_human();
    }
 }
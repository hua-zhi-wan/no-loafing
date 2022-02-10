pub fn error_handler(etype: &str, msg: &str) {
    eprint!("!!! {} ERROR !!!\n{}", &etype, &msg);
}

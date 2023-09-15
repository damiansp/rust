// (Bogus) error: mismatched types: expected i32, found ()
fn wait_for_process(process: &mut Process) -> i32 {
    while true {
        if process.wait() {
            return process.get_exit_code();
        }
    }
}


fn serve_forever(socket: ServerSocket, handler: ServerHandler) -> ! {
    socket.listen();
    loop {
        let s = socket.accept();
        handler.handle(s);
    }
}
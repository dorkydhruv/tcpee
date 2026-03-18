// ref: https://blog.netherlabs.nl/articles/2009/01/18/the-ultimate-so_linger-page-or-why-is-my-tcp-not-reliable
fn main() {
    let mut remote: libc::sockaddr_in = unsafe { std::mem::zeroed() };
    remote.sin_family = libc::AF_INET as u8;
    remote.sin_port = u16::to_be(6969);
}
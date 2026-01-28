fn main() {
    let mut remote: libc::sockaddr_in = unsafe { std::mem::zeroed() };
    remote.sin_family = libc::AF_INET as u8;
    remote.sin_port = u16::to_be(6969);
}
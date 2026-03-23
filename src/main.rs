// ref: https://blog.netherlabs.nl/articles/2009/01/18/the-ultimate-so_linger-page-or-why-is-my-tcp-not-reliable

pub fn linger() {
    let linger = libc::linger {
        l_onoff: 1,
        l_linger: 0,
    };
    unsafe {
        libc::setsockopt(
            0, // socket fd
            libc::SOL_SOCKET,
            libc::SO_LINGER,
            &linger as *const _ as *const libc::c_void,
            std::mem::size_of::<libc::linger>() as libc::socklen_t,
        );
    }
}
fn main() {
    let mut remote: libc::sockaddr_in = unsafe { std::mem::zeroed() };
    remote.sin_family = libc::AF_INET as u8;
    remote.sin_port = u16::to_be(6969);
}
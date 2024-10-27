pub mod womscp {
    use std::net::TcpStream;
    use std::io::Read;


    const WOMSCP_REQ_LEN :usize = 10;


    #[derive(Debug)]
    pub enum RequestFlags {
        SrvrRdy    = 1,
        Dummy      = 1 >> 1
    }

    #[derive(Debug)]
    pub struct Request {
        pub version: u8,
        pub m_id: u16,
        pub s_id: u8,
        pub t: u8,
        pub data: u32,
        pub flags: u8
    }

    impl From<[u8; WOMSCP_REQ_LEN]> for Request {
        fn from(buf: [u8; WOMSCP_REQ_LEN]) -> Self {
            Request { 
                version: buf[0], 
                m_id: u16::from_be_bytes([buf[1], buf[2]]),
                s_id: buf[3], 
                t: buf[4], 
                data: u32::from_be_bytes([buf[5], buf[6], buf[7], buf[8]]),
                flags: buf[9]
            }
        }
    }

    impl TryFrom<TcpStream> for Request {
        type Error = std::io::Error;

        fn try_from(mut stream: TcpStream) -> Result<Self, Self::Error> {
            let mut buf :[u8; WOMSCP_REQ_LEN] = [0; WOMSCP_REQ_LEN];

            if let Err(e) = stream.read(&mut buf) {
                return Err(e);
            }

            return Ok(Self::from(buf));
        }
    }


    #[derive(Debug)]
    pub enum ResponseError {
        NotReady = 1,
        Version,
        Unrecognised,
        Database
    }

    #[derive(Debug)]
    pub struct Response {
        pub version :u8,
        pub response :Result<(), ResponseError>
    }
}

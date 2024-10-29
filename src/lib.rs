pub mod womscp {
    use std::net::TcpStream;
    use std::io::Read;


    pub const WOMSCP_VERSION :u8 = 1;
    pub const WOMSCP_REQ_LEN :usize = 10;
    pub const WOMSCP_RES_LEN :usize = 2;


    #[derive(Debug)]
    pub enum RequestFlags {
        SrvrRdy    = 1,
        Dummy      = 1 >> 1
    }

    #[derive(Debug, PartialEq)]
    pub struct Request {
        pub version: u8,
        pub m_id: u16,
        pub s_id: u8,
        pub sensor_type: u8,
        pub data: u32,
        pub flags: u8
    }

    impl TryFrom<[u8; WOMSCP_REQ_LEN]> for Request {
        type Error = ResponseError;

        fn try_from(buf: [u8; WOMSCP_REQ_LEN]) -> Result<Self, Self::Error> {
            let req = Request { 
                version: buf[0], 
                m_id: u16::from_be_bytes([buf[1], buf[2]]),
                s_id: buf[3], 
                sensor_type: buf[4], 
                data: u32::from_be_bytes([buf[5], buf[6], buf[7], buf[8]]),
                flags: buf[9]
            };

            if req.version != WOMSCP_VERSION {
                return Err(ResponseError::Version);
            }

            Ok(req)
        }
    }

    impl TryFrom<TcpStream> for Request {
        type Error = ResponseError;

        fn try_from(mut stream: TcpStream) -> Result<Self, Self::Error> {
            let mut buf :[u8; WOMSCP_REQ_LEN] = [0; WOMSCP_REQ_LEN];

            if let Err(e) = stream.read(&mut buf) {
                eprintln!("{:?}", e);
                return Err(ResponseError::Tcp);
            }

            let req =  Self::try_from(buf)?;

            if req.version != WOMSCP_VERSION {
                return Err(ResponseError::Version);
            }

            return Ok(req);
        }
    }


    #[derive(Debug, PartialEq)]
    pub enum ResponseError {
        NotReady = 1,
        Version,
        Unrecognised,
        Tcp,
        Database
    }

    #[derive(Debug, PartialEq)]
    pub struct Response {
        pub version :u8,
        pub response :Result<(), ResponseError>
    }

    impl Response {
        pub fn new(res :Result<(), ResponseError>) -> Self {
            Response {
                version: WOMSCP_VERSION,
                response: res
            }
        }
    }

    impl Into<[u8; WOMSCP_RES_LEN]> for Response {
        fn into(self) -> [u8; WOMSCP_RES_LEN] {
            let mut buf = [0; WOMSCP_RES_LEN];

            buf[0] = self.version;
            buf[1] = if self.response.is_ok() {
                0
            } else {
                match self.response.unwrap_err() {
                    ResponseError::NotReady     => 1,
                    ResponseError::Version      => 2,
                    ResponseError::Unrecognised => 3,
                    ResponseError::Tcp          => 4,
                    ResponseError::Database     => 5
                }
            };

            buf
        }
    }
}


mod tests;

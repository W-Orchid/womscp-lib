pub mod womscp {
    use std::net::TcpStream;
    use std::io::Read;


    pub const WOMSCP_VERSION :u8 = 1;

    const WOMSCP_REQ_LEN :usize = 10;


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


    #[derive(Debug)]
    pub enum ResponseError {
        NotReady = 1,
        Version,
        Unrecognised,
        Tcp,
        Database
    }

    #[derive(Debug)]
    pub struct Response {
        pub version :u8,
        pub response :Result<(), ResponseError>
    }
}

#[cfg(test)] 
mod tests {
    use crate::womscp::Request;

    #[test]
    fn parse_request() {
        let buf = [1, 0, 0x0d, 5, 3, 0, 0, 0, 0x7b, 0];
        let correct_res = Request {
            version: 1,
            m_id: 0x0d,
            s_id: 5,
            sensor_type: 3,
            data: 0x7b,
            flags: 0
        };

        let res :Request = Request::try_from(buf).expect("Couldn't parse byte string!");

        assert_eq!(res, correct_res);
    }
}

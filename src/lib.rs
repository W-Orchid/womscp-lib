pub mod womscp {
    use tokio::net::TcpStream;
    use tokio::io::AsyncReadExt;

    pub const WOMSCP_VERSION :u8 = 1;
    pub const WOMSCP_REQ_LEN :usize = 10;


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

    impl Request {
        pub async fn try_from_tcp(stream: &mut TcpStream) -> Result<Self, ResponseError>  {
            let mut buf :[u8; WOMSCP_REQ_LEN] = [0; WOMSCP_REQ_LEN];

            let mut bytes_read :usize = 0;
            while bytes_read < WOMSCP_REQ_LEN {
                match stream.read(&mut buf).await {
                    Ok(n) => bytes_read += n,
                    Err(e) => {
                        eprintln!("{:?}", e);
                        return Err(ResponseError::Tcp);
                    }
                }
            }

            let req =  Self::try_from(buf)?;

            if req.version != WOMSCP_VERSION {
                return Err(ResponseError::Version);
            }

            return Ok(req);
        }
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

    impl TryInto<[u8; WOMSCP_REQ_LEN]> for Request {
        type Error = ResponseError;

        fn try_into(self) -> Result<[u8; WOMSCP_REQ_LEN], Self::Error> {
            let mut buf :[u8; WOMSCP_REQ_LEN] = [0; WOMSCP_REQ_LEN];

            let m_id_bytes = self.m_id.to_be_bytes();
            let data_bytes = self.data.to_be_bytes();

            if self.version != WOMSCP_VERSION {
                return Err(ResponseError::Version);
            }

            buf[0] = self.version;
            buf[1] = m_id_bytes[0];
            buf[2] = m_id_bytes[1];
            buf[3] = self.s_id;
            buf[4] = self.sensor_type;
            buf[5] = data_bytes[0];
            buf[6] = data_bytes[1];
            buf[7] = data_bytes[2];
            buf[8] = data_bytes[3];
            buf[9] = self.flags;

            Ok(buf)
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
}


mod tests;

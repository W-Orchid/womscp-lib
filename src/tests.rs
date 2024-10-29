#[cfg(test)] 
use crate::womscp::*;

#[test]
fn parse_buf_to_request() {
    let buf :[u8; WOMSCP_REQ_LEN] = [WOMSCP_VERSION, 0, 0x0d, 5, 3, 0, 0, 0, 0x7b, 0];
    let correct_req = Request {
        version: WOMSCP_VERSION,
        m_id: 0x0d,
        s_id: 5,
        sensor_type: 3,
        data: 0x7b,
        flags: 0
    };

    let req :Request = Request::try_from(buf).expect("Couldn't parse byte string!");

    assert_eq!(req, correct_req);
}

#[test]
fn parse_request_to_buf() {
    let req = Request {
        version: WOMSCP_VERSION,
        m_id: 0x0d,
        s_id: 5,
        sensor_type: 3,
        data: 0x7b,
        flags: 0
    };

    let correct_buf :[u8; WOMSCP_REQ_LEN] = [WOMSCP_VERSION, 0, 0x0d, 5, 3, 0, 0, 0, 0x7b, 0];

    let buf :[u8; WOMSCP_REQ_LEN] = req.try_into().unwrap();

    assert_eq!(buf, correct_buf);
}

#[test]
fn wrong_version() {
    let buf :[u8; WOMSCP_REQ_LEN] = [0, 0, 0x0d, 5, 3, 0, 0, 0, 0x7b, 0];

    match Request::try_from(buf) {
        Ok(_) => panic!("Wrong version not detected!"),
        Err(e) => {
            match e {
                Response::Version => {},
                _ => panic!("Wrong version not detected! Some other error occured!")
            }
        }
    }
}

#[cfg(test)] 
use crate::womscp::*;

#[test]
fn parse_request() {
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
fn wrong_version() {
    let buf :[u8; WOMSCP_REQ_LEN] = [0, 0, 0x0d, 5, 3, 0, 0, 0, 0x7b, 0];
    let correct_res = Response::new(Err(ResponseError::Version));

    let mut res = Response::new(Ok(()));

    match Request::try_from(buf) {
        Ok(_) => panic!("Wrong version not detected!"),
        Err(e) => {
            match e {
                ResponseError::Version => res.response = Err(ResponseError::Version),
                _ => panic!("Wrong version not detected! Some other error occured!")
            }
        }
    }

    assert_eq!(res, correct_res);
}

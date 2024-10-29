#[cfg(test)] 
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

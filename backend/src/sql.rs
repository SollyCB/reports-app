use mysql_async::prelude::*;
use mysql_async::Conn;

pub struct Report {
    pupil_id: usize,
    subject_id: usize,
    term: usize,
    content: String,
    conn: Conn,
}

impl Report {

    pub fn new(pupil_id: usize, subject_id: usize, term: usize, content: String, conn: Conn) -> Report {
        Report {
            pupil_id,
            subject_id,
            term,
            content,
            conn,
        }
    }

}

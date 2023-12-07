use crate::session::course::Course;
use crate::utils::address::Address;
use sqlite::Connection;
use std::{collections::HashMap, fs::File, ops::Deref};

pub struct DataBase {
    connection: Connection,
}
impl Deref for DataBase {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.connection
    }
}
// self
impl DataBase {
    pub fn new() -> Self {
        let db_dir = crate::utils::CONFIG_DIR.join("cx.db");
        if db_dir.metadata().is_err() {
            File::create(db_dir.clone()).unwrap();
        }
        let connection = Connection::open(db_dir.to_str().unwrap()).unwrap();
        let db = Self { connection };
        db.create_table_account();
        db.create_table_course();
        db.create_table_pos();
        db
    }
}
// account
impl DataBase {
    const CREATE_ACCOUNT_SQL: &'static str =
        "CREATE TABLE account (uname CHAR (50) UNIQUE NOT NULL,pwd TEXT NOT NULL,name TEXT NOT NULL);";

    fn has_table_account(&self) -> bool {
        let mut query = self
            .connection
            .prepare("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='account';")
            .unwrap();
        query.next().unwrap();
        query.read::<i64, _>(0).unwrap() == 1
    }

    pub fn has_account(&self, uname: &str) -> bool {
        let mut query = self
            .connection
            .prepare("SELECT count(*) FROM account WHERE uname=?;")
            .unwrap();
        query.bind((1, uname)).unwrap();
        query.next().unwrap();
        query.read::<i64, _>(0).unwrap() > 0
    }

    pub fn delete_account(&self, uname: &str) {
        if self.has_account(uname) {
            let mut query = self
                .connection
                .prepare("DELETE FROM account WHERE uname=?;")
                .unwrap();
            query.bind((1, uname)).unwrap();
            query.next().unwrap();
        }
        std::fs::remove_file(super::CONFIG_DIR.join(uname.to_string() + ".json")).unwrap();
    }

    pub fn add_account_or<O: Fn(&DataBase, &str, &str, &str)>(
        &self,
        uname: &str,
        pwd: &str,
        name: &str,
        or: O,
    ) {
        let mut query = self
            .connection
            .prepare("INSERT INTO account(uname,pwd,name) values(:uname,:pwd,:name);")
            .unwrap();
        query
            .bind::<&[(_, sqlite::Value)]>(
                &[
                    (":uname", uname.into()),
                    (":pwd", pwd.into()),
                    (":name", name.into()),
                ][..],
            )
            .unwrap();
        match query.next() {
            Ok(_) => (),
            Err(_) => or(self, uname, pwd, name),
        };
    }

    pub fn update_account(&self, uname: &str, pwd: &str, name: &str) {
        let mut query = self
            .connection
            .prepare("UPDATE account SET pwd=:pwd,name=:name WHERE uname=:uname;")
            .unwrap();
        query
            .bind::<&[(_, sqlite::Value)]>(
                &[
                    (":uname", uname.into()),
                    (":pwd", pwd.into()),
                    (":name", name.into()),
                ][..],
            )
            .unwrap();
        query.next().unwrap();
    }

    fn create_table_account(&self) {
        if !self.has_table_account() {
            self.connection.execute(Self::CREATE_ACCOUNT_SQL).unwrap();
        }
    }

    pub fn get_accounts(&self) -> HashMap<String, (String, String)> {
        let mut query = self.connection.prepare("SELECT * FROM account;").unwrap();
        let mut accounts = HashMap::new();
        for c in query.iter() {
            if let Ok(row) = c {
                let uname: &str = row.read("uname");
                let pwd: &str = row.read("pwd");
                let name: &str = row.read("name");
                accounts.insert(uname.into(), (pwd.into(), name.into()));
            } else {
                eprintln!("账号解析行出错：{c:?}.");
            }
        }
        accounts
    }
}
// course
impl DataBase {
    const CREATE_COURSE_SQL: &'static str ="CREATE TABLE course (id INTEGER UNIQUE NOT NULL,clazzid INTEGER NOT NULL,name TEXT NOT NULL,teacher TEXT NOT NULL,image TEXT NOT NULL);";

    fn has_table_course(&self) -> bool {
        let mut query = self
            .connection
            .prepare("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='course';")
            .unwrap();
        query.next().unwrap();
        query.read::<i64, _>(0).unwrap() == 1
    }
    pub fn add_course_or<O: Fn(&DataBase, &Course)>(&self, course: &Course, or: O) {
        let id: i64 = course.get_id();
        let clazzid: i64 = course.get_class_id();
        let name: &str = course.get_name();
        let teacher: &str = course.get_teacher_name();
        let image: &str = course.get_image_url();
        let mut query =self.connection.prepare("INSERT INTO course(id,clazzid,name,teacher,image) values(:id,:clazzid,:name,:teacher,:image);").unwrap();
        query
            .bind::<&[(_, sqlite::Value)]>(
                &[
                    (":id", id.into()),
                    (":clazzid", clazzid.into()),
                    (":name", name.into()),
                    (":teacher", teacher.into()),
                    (":image", image.into()),
                ][..],
            )
            .unwrap();
        match query.next() {
            Ok(_) => (),
            Err(_) => or(self, course),
        }
    }
    pub fn delete_all_course(&self) {
        let mut query = self.connection.prepare("DELETE FROM course;").unwrap();
        query.next().unwrap();
        println!("已删除旧的课程信息。");
    }
    fn create_table_course(&self) {
        if !self.has_table_course() {
            self.connection.execute(Self::CREATE_COURSE_SQL).unwrap();
        }
    }
    pub fn get_courses(&self) -> HashMap<i64, Course> {
        let mut query = self.connection.prepare("SELECT * FROM course;").unwrap();
        let mut courses = HashMap::new();
        for c in query.iter() {
            if let Ok(row) = c {
                let id = row.read("id");
                let clazzid = row.read("clazzid");
                let teacher = row.read::<&str, _>("teacher");
                let image = row.read::<&str, _>("image");
                let name = row.read::<&str, _>("name");
                courses.insert(id, Course::new(id, clazzid, teacher, image, name));
            } else {
                eprintln!("课程解析行出错：{c:?}.");
            }
        }
        courses
    }
}
// pos
impl DataBase {
    const CREATE_POS_SQL: &'static str ="CREATE TABLE pos(posid INTEGER UNIQUE NOT NULL,courseid INTEGER NOT NULL,addr TEXT NOT NULL,lon TEXT NOT NULL,lat TEXT NOT NULL,alt TEXT NOT NULL);";

    fn has_table_pos(&self) -> bool {
        let mut query = self
            .connection
            .prepare("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='pos';")
            .unwrap();
        query.next().unwrap();
        query.read::<i64, _>(0).unwrap() == 1
    }
    pub fn has_pos(&self, posid: i64) -> bool {
        let mut query = self
            .connection
            .prepare("SELECT count(*) FROM pos WHERE posid=?;")
            .unwrap();
        query.bind((1, posid)).unwrap();
        query.next().unwrap();
        query.read::<i64, _>(0).unwrap() > 0
    }
    pub fn add_pos_or<O: Fn(&DataBase, i64, i64, &Address)>(
        &self,
        posid: i64,
        course_id: i64,
        pos: &Address,
        or: O,
    ) {
        let addr = pos.get_addr();
        let lat = pos.get_lat();
        let lon = pos.get_lon();
        let alt = pos.get_alt();
        let mut query =self.connection.prepare("INSERT INTO pos(posid,courseid,addr,lat,lon,alt) values(:posid,:courseid,:addr,:lat,:lon,:alt);").unwrap();
        query
            .bind::<&[(_, sqlite::Value)]>(
                &[
                    (":posid", posid.into()),
                    (":courseid", course_id.into()),
                    (":addr", addr.into()),
                    (":lat", lat.into()),
                    (":lon", lon.into()),
                    (":alt", alt.into()),
                ][..],
            )
            .unwrap();
        match query.next() {
            Ok(_) => (),
            Err(_) => or(self, posid, course_id, pos),
        }
    }
    pub fn delete_pos(&self, posid: i64) {
        self.connection
            .execute("DELETE FROM pos WHERE posid=".to_string() + posid.to_string().as_str() + ";")
            .unwrap();
    }
    pub fn delete_all_pos(&self) {
        self.connection.execute("DELETE FROM pos;").unwrap();
    }
    fn create_table_pos(&self) {
        if !self.has_table_pos() {
            self.connection.execute(Self::CREATE_POS_SQL).unwrap();
        }
    }
    pub fn get_poss(&self) -> HashMap<i64, (i64, Address)> {
        let mut query = self.connection.prepare("SELECT * FROM pos;").unwrap();
        let mut poss = HashMap::new();
        for c in query.iter() {
            if let Ok(row) = c {
                let posid = row.read("posid");
                let addr = row.read("addr");
                let lat = row.read("lat");
                let lon = row.read("lon");
                let alt = row.read("alt");
                let courseid = row.read("courseid");
                poss.insert(posid, (courseid, Address::new(addr, lon, lat, alt)));
            } else {
                eprintln!("位置解析行出错：{c:?}.");
            }
        }
        poss
    }
    pub fn get_pos(&self, posid: i64) -> (i64, Address) {
        let mut query = self
            .connection
            .prepare("SELECT * FROM pos WHERE posid=?;")
            .unwrap();
        query.bind((1, posid)).unwrap();
        let c: Vec<sqlite::Row> = query
            .iter()
            .filter_map(|e| if let Ok(e) = e { Some(e) } else { None })
            .collect();
        let row = &c[0];
        let addr = row.read("addr");
        let lat = row.read("lat");
        let lon = row.read("lon");
        let alt = row.read("alt");
        let courseid = row.read("courseid");
        (courseid, Address::new(addr, lon, lat, alt))
    }
    pub fn get_course_poss(&self, course_id: i64) -> HashMap<i64, Address> {
        let mut query = self
            .connection
            .prepare("SELECT * FROM pos WHERE courseid=?;")
            .unwrap();
        query.bind((1, course_id)).unwrap();
        let mut poss = HashMap::new();
        for c in query.iter() {
            if let Ok(row) = c {
                let posid = row.read("posid");
                let addr = row.read("addr");
                let lat = row.read("lat");
                let lon = row.read("lon");
                let alt = row.read("alt");
                poss.insert(posid, Address::new(addr, lon, lat, alt));
            } else {
                eprintln!("位置解析行出错：{c:?}.");
            }
        }
        poss
    }
    pub fn get_course_poss_without_posid(&self, course_id: i64) -> Vec<Address> {
        let mut query = self
            .connection
            .prepare("SELECT * FROM pos WHERE courseid=?;")
            .unwrap();
        query.bind((1, course_id)).unwrap();
        let mut poss = Vec::new();
        for c in query.iter() {
            if let Ok(row) = c {
                let addr = row.read("addr");
                let lat = row.read("lat");
                let lon = row.read("lon");
                let alt = row.read("alt");
                poss.push(Address::new(addr, lon, lat, alt));
            } else {
                eprintln!("位置解析行出错：{c:?}.");
            }
        }
        poss
    }
}

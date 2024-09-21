use crate::shared::entities::user::User;
use crate::shared::repository::db::DB;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Deserialize)]
pub struct UserDB {
    pub id: Thing,
    pub names: String,
    pub last_name1: String,
    pub last_name2: String,
    pub gender: bool,
    pub role: String,
}

impl UserDB {
    pub fn map(self) -> User {
        User {
            id: self.id.id.to_string(),
            names: self.names,
            last_name1: self.last_name1,
            last_name2: self.last_name2,
            gender: self.gender,
            role: self.role,
        }
    }
}

pub async fn course_professors(course_id: String) -> Result<impl Serialize, (u16, String)> {
    let query = r#"
SELECT
in.id AS id,
in.names AS names,
in.last_name1 AS last_name1,
in.last_name2 AS last_name2,
in.gender AS gender,
role
FROM type::thing("course", $id)<-teaches
"#;

    let mut resp = match DB.query(query).bind(("id", course_id)).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    let professors = match resp.take::<Vec<UserDB>>(0) {
        Ok(p) => p,
        Err(e) => return Err((500, format!("DB parse error: {}", e.to_string()))),
    };

    Ok(professors
        .into_iter()
        .map(|p| p.map())
        .collect::<Vec<User>>())
}

pub async fn desasign(course_id: &String, teacher_id: &String) -> Result<String, (u16, String)> {
    let query = format!(
        r#"
DELETE teaches where in=professor:{} && out=course:{} return before
"#,
        teacher_id, course_id
    );

    match DB.query(query).await {
        Ok(_) => Ok(format!(
            "Teacher {} desasigned from course {}",
            teacher_id, course_id
        )),
        Err(e) => Err((500, format!("DB Error: {}", e.to_string()))),
    }
}

pub async fn asign(
    course_id: String,
    professor_id: String,
    role: String,
) -> Result<String, (u16, String)> {
    let query = r#"
IF (SELECT in AS in FROM type::thing("course", $course_id)<-teaches)[0].in != type::thing("professor", <int>$professor_id)
	{{
		RELATE (type::thing("professor", <int>$professor_id))->teaches->(type::thing("course", $course_id))
    SET role=$role;
	}}
ELSE
	{{
		THROW 'Professor aleredy assigned to course';
	}}

"#;

    let mut resp = match DB
        .query(query)
        .bind(("course_id", course_id))
        .bind(("professor_id", professor_id))
        .bind(("role", role))
        .await
    {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Option<String>>(0) {
        Ok(_) => Ok("Teacher assigned".to_string()),
        Err(e) => Err((500, format!("DB parse error: {}", e.to_string()))),
    }
}

#[derive(Deserialize)]
pub struct SchoolDB {
    id: Thing,
    name: String,
    school: String,
}

#[derive(Serialize)]
pub struct School {
    id: String,
    name: String,
    school: String,
}

impl SchoolDB {
    fn map(self) -> School {
        School {
            id: self.id.id.to_string(),
            name: self.name,
            school: self.school,
        }
    }
}

pub async fn courses(professor_id: String) -> Result<impl Serialize, (u16, String)> {
    let query = r#"
SELECT (->course<-offers<-school.name)[0] as school,
out.name AS name,
out.id AS id 
FROM type::thing("professor", <int>$professor_id)->teaches;
    "#;

    let mut resp = match DB.query(query).bind(("professor_id", professor_id)).await {
        Ok(r) => r,
        Err(e) => return Err((500, format!("DB Error: {}", e.to_string()))),
    };

    match resp.take::<Vec<SchoolDB>>(0) {
        Ok(schools) => Ok(schools
            .into_iter()
            .map(|s| s.map())
            .collect::<Vec<School>>()),

        Err(e) => Err((500, format!("DB error to parse: {}", e.to_string()))),
    }
}

-- ------------------------------
-- Export generated by Surrealist on 2024-09-16T00:56:41.603Z
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- TABLE: course
-- ------------------------------

DEFINE TABLE course TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD name ON course TYPE string PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD places ON course TYPE int PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD tests ON course FLEXIBLE TYPE array<object> PERMISSIONS FULL;
DEFINE FIELD tests[*] ON course FLEXIBLE TYPE object PERMISSIONS FULL;

-- ------------------------------
-- TABLE: divided
-- ------------------------------

DEFINE TABLE divided TYPE RELATION IN semester OUT week SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON divided TYPE record<semester> PERMISSIONS FULL;
DEFINE FIELD out ON divided TYPE record<week> PERMISSIONS FULL;

-- ------------------------------
-- TABLE: enrolled
-- ------------------------------

DEFINE TABLE enrolled TYPE ANY SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON enrolled TYPE record<student> PERMISSIONS FULL;
DEFINE FIELD out ON enrolled TYPE record<course> PERMISSIONS FULL;

DEFINE INDEX course_index ON enrolled FIELDS in UNIQUE;

-- ------------------------------
-- TABLE: evaluated
-- ------------------------------

DEFINE TABLE evaluated TYPE RELATION IN course OUT student SCHEMALESS PERMISSIONS NONE;

DEFINE FIELD ev_type ON evaluated TYPE string PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD in ON evaluated TYPE record<course> PERMISSIONS FULL;
DEFINE FIELD number ON evaluated TYPE int PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD out ON evaluated TYPE record<student> PERMISSIONS FULL;
DEFINE FIELD score ON evaluated TYPE float ASSERT 0 <= score AND score <= 20 PERMISSIONS FOR select, create, update, delete WHERE FULL;

DEFINE INDEX index_id ON evaluated FIELDS id UNIQUE;

-- ------------------------------
-- TABLE: faculty
-- ------------------------------

DEFINE TABLE faculty TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD name ON faculty TYPE string VALUE string::uppercase(name) PERMISSIONS FOR select, create, update, delete WHERE FULL;

DEFINE INDEX index_name ON faculty FIELDS name UNIQUE;

-- ------------------------------
-- TABLE: has
-- ------------------------------

DEFINE TABLE has TYPE RELATION IN school OUT student SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON has TYPE record<school> PERMISSIONS FULL;
DEFINE FIELD out ON has TYPE record<student> PERMISSIONS FULL;

DEFINE INDEX index_student ON has FIELDS out UNIQUE;

-- ------------------------------
-- TABLE: includes
-- ------------------------------

DEFINE TABLE includes TYPE RELATION IN faculty OUT school SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON includes TYPE record<faculty> PERMISSIONS FULL;
DEFINE FIELD out ON includes TYPE record<school> PERMISSIONS FULL;

DEFINE INDEX index_school ON includes FIELDS out UNIQUE;

-- ------------------------------
-- TABLE: offers
-- ------------------------------

DEFINE TABLE offers TYPE RELATION IN school OUT course SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON offers TYPE record<school> PERMISSIONS FULL;
DEFINE FIELD out ON offers TYPE record<course> PERMISSIONS FULL;

-- ------------------------------
-- TABLE: professor
-- ------------------------------

DEFINE TABLE professor TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD dni ON professor TYPE string ASSERT string::len(dni) == 8 AND string::is::numeric(dni) PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD gender ON professor TYPE bool PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD last_name1 ON professor TYPE string PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD last_name2 ON professor TYPE string PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD names ON professor TYPE string VALUE string::trim(names) PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD password ON professor TYPE string PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD role ON professor TYPE string DEFAULT 'professor' PERMISSIONS FOR select, create, update, delete WHERE FULL;

DEFINE INDEX index_dni ON professor FIELDS dni UNIQUE;

-- ------------------------------
-- TABLE: programmed
-- ------------------------------

DEFINE TABLE programmed TYPE RELATION IN course OUT semester SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON programmed TYPE record<course> PERMISSIONS FULL;
DEFINE FIELD out ON programmed TYPE record<semester> PERMISSIONS FULL;

-- ------------------------------
-- TABLE: register_time
-- ------------------------------

DEFINE TABLE register_time TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD end ON register_time TYPE datetime PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD start ON register_time TYPE datetime DEFAULT time::now() PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD todo ON register_time TYPE string PERMISSIONS FOR select, create, update, delete WHERE FULL;

-- ------------------------------
-- TABLE: school
-- ------------------------------

DEFINE TABLE school TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD name ON school TYPE string VALUE string::uppercase(name) PERMISSIONS FOR select, create, update, delete WHERE FULL;

DEFINE INDEX index_name ON school FIELDS name UNIQUE;

-- ------------------------------
-- TABLE: semester
-- ------------------------------

DEFINE TABLE semester TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

-- ------------------------------
-- TABLE: student
-- ------------------------------

DEFINE TABLE student TYPE NORMAL SCHEMALESS PERMISSIONS NONE;

DEFINE FIELD code ON student TYPE string ASSERT string::is::numeric(code) AND string::len(code) == 10 PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD gender ON student TYPE bool PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD last_name1 ON student TYPE string ASSERT string::is::alpha(last_name1) PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD last_name2 ON student TYPE string ASSERT string::is::alpha(last_name2) PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD names ON student TYPE string PERMISSIONS FOR select, create, update, delete WHERE FULL;
DEFINE FIELD password ON student TYPE string PERMISSIONS FOR select, create, update, delete WHERE FULL;

DEFINE INDEX index_code ON student FIELDS code UNIQUE;

-- ------------------------------
-- TABLE: teaches
-- ------------------------------

DEFINE TABLE teaches TYPE RELATION IN professor OUT course SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON teaches TYPE record<professor> PERMISSIONS FULL;
DEFINE FIELD out ON teaches TYPE record<course> PERMISSIONS FULL;
DEFINE FIELD role ON teaches TYPE string PERMISSIONS FOR select, create, update, delete WHERE FULL;

-- ------------------------------
-- TABLE: test_type
-- ------------------------------

DEFINE TABLE test_type TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD name ON test_type TYPE string PERMISSIONS FOR select, create, update, delete WHERE FULL;

DEFINE INDEX index_name ON test_type FIELDS name UNIQUE;

-- ------------------------------
-- TABLE: week
-- ------------------------------

DEFINE TABLE week TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

-- ------------------------------
-- TRANSACTION
-- ------------------------------

BEGIN TRANSACTION;

-- ------------------------------
-- TABLE DATA: course
-- ------------------------------

UPDATE course:f04lf2tqlor5jg2qt4ej CONTENT { id: course:f04lf2tqlor5jg2qt4ej, name: 'Física II', places: 100, tests: [{ name: 'test', weight: 50 }, { name: 'práctice', weight: 50 }] };
UPDATE course:kp7n2n27oedkvphduv29 CONTENT { id: course:kp7n2n27oedkvphduv29, name: 'Física I', places: 100, tests: [{ name: 'test', weight: 50 }, { name: 'practice', weight: 50 }] };

-- ------------------------------
-- TABLE DATA: enrolled
-- ------------------------------

RELATE student:2222222222 -> enrolled:953padbremko0xybmc5a -> course:kp7n2n27oedkvphduv29 CONTENT { id: enrolled:953padbremko0xybmc5a, in: student:2222222222, out: course:kp7n2n27oedkvphduv29 };
RELATE student:4444444444 -> enrolled:ala4j9z8nkja7i51kun8 -> course:kp7n2n27oedkvphduv29 CONTENT { id: enrolled:ala4j9z8nkja7i51kun8, in: student:4444444444, out: course:kp7n2n27oedkvphduv29 };

-- ------------------------------
-- TABLE DATA: evaluated
-- ------------------------------

RELATE course:kp7n2n27oedkvphduv29 -> evaluated:5eyke0d5h26ah7q16f6t -> student:2222222222 CONTENT { ev_type: 'test', id: evaluated:5eyke0d5h26ah7q16f6t, in: course:kp7n2n27oedkvphduv29, number: 2, out: student:2222222222, score: 10 };
RELATE course:kp7n2n27oedkvphduv29 -> evaluated:ebz0n3zkdco0j9sjq5ir -> student:2222222222 CONTENT { ev_type: 'practice', id: evaluated:ebz0n3zkdco0j9sjq5ir, in: course:kp7n2n27oedkvphduv29, number: 1, out: student:2222222222, score: 2 };
RELATE course:kp7n2n27oedkvphduv29 -> evaluated:ivn83fil77vt1a2ir159 -> student:2222222222 CONTENT { ev_type: 'practice', id: evaluated:ivn83fil77vt1a2ir159, in: course:kp7n2n27oedkvphduv29, number: 2, out: student:2222222222, score: 13 };
RELATE course:kp7n2n27oedkvphduv29 -> evaluated:l8upun5g6z45swst90n7 -> student:4444444444 CONTENT { ev_type: 'test', id: evaluated:l8upun5g6z45swst90n7, in: course:kp7n2n27oedkvphduv29, number: 3, out: student:4444444444, score: 12 };
RELATE course:kp7n2n27oedkvphduv29 -> evaluated:lgwj6duub1rl4j8h00yg -> student:4444444444 CONTENT { ev_type: 'test', id: evaluated:lgwj6duub1rl4j8h00yg, in: course:kp7n2n27oedkvphduv29, number: 2, out: student:4444444444, score: 4 };
RELATE course:kp7n2n27oedkvphduv29 -> evaluated:lnzrb5bmdyb2gre5otut -> student:2222222222 CONTENT { ev_type: 'test', id: evaluated:lnzrb5bmdyb2gre5otut, in: course:kp7n2n27oedkvphduv29, number: 2, out: student:2222222222, score: 20 };
RELATE course:kp7n2n27oedkvphduv29 -> evaluated:mr074vy6vuyoz7htvahi -> student:4444444444 CONTENT { ev_type: 'practice', id: evaluated:mr074vy6vuyoz7htvahi, in: course:kp7n2n27oedkvphduv29, number: 1, out: student:4444444444, score: 9 };
RELATE course:kp7n2n27oedkvphduv29 -> evaluated:pjnnbt9f264tvqvjfwi0 -> student:2222222222 CONTENT { ev_type: 'practice', id: evaluated:pjnnbt9f264tvqvjfwi0, in: course:kp7n2n27oedkvphduv29, number: 3, out: student:2222222222, score: 12 };
RELATE course:kp7n2n27oedkvphduv29 -> evaluated:qjsfrjk64bzyw67p850a -> student:2222222222 CONTENT { ev_type: 'test', id: evaluated:qjsfrjk64bzyw67p850a, in: course:kp7n2n27oedkvphduv29, number: 1, out: student:2222222222, score: 20 };
RELATE course:kp7n2n27oedkvphduv29 -> evaluated:yxf6ew3h9q9hwbez6rve -> student:4444444444 CONTENT { ev_type: 'test', id: evaluated:yxf6ew3h9q9hwbez6rve, in: course:kp7n2n27oedkvphduv29, number: 1, out: student:4444444444, score: 13 };

-- ------------------------------
-- TABLE DATA: faculty
-- ------------------------------

UPDATE faculty:vlf3cz01hjbfbulrld3e CONTENT { id: faculty:vlf3cz01hjbfbulrld3e, name: 'HOLA' };
UPDATE faculty:zkifk1lew7goyi1bn0lm CONTENT { id: faculty:zkifk1lew7goyi1bn0lm, name: 'CIENCIAS' };

-- ------------------------------
-- TABLE DATA: has
-- ------------------------------

RELATE school:qr4i045651hejltikx9t -> has:44a6tfk79o2xs522vzdu -> student:2222222222 CONTENT { id: has:44a6tfk79o2xs522vzdu, in: school:qr4i045651hejltikx9t, out: student:2222222222 };
RELATE school:qr4i045651hejltikx9t -> has:bxspnzgn5n00907rekoe -> student:3333333333 CONTENT { id: has:bxspnzgn5n00907rekoe, in: school:qr4i045651hejltikx9t, out: student:3333333333 };
RELATE school:qr4i045651hejltikx9t -> has:dixhsc5dz9j9m9gfk7yi -> student:0 CONTENT { id: has:dixhsc5dz9j9m9gfk7yi, in: school:qr4i045651hejltikx9t, out: student:0 };
RELATE school:qr4i045651hejltikx9t -> has:fqoxvh22145udeve63xh -> student:4444444444 CONTENT { id: has:fqoxvh22145udeve63xh, in: school:qr4i045651hejltikx9t, out: student:4444444444 };
RELATE school:qr4i045651hejltikx9t -> has:nuj9g7m8xo3pey9sngfy -> student:987654321 CONTENT { id: has:nuj9g7m8xo3pey9sngfy, in: school:qr4i045651hejltikx9t, out: student:987654321 };
RELATE school:qr4i045651hejltikx9t -> has:qkqyzw0qw452f31m989e -> student:1234567890 CONTENT { id: has:qkqyzw0qw452f31m989e, in: school:qr4i045651hejltikx9t, out: student:1234567890 };

-- ------------------------------
-- TABLE DATA: includes
-- ------------------------------

RELATE faculty:zkifk1lew7goyi1bn0lm -> includes:accv7r0adjfvi9axnn8s -> school:qr4i045651hejltikx9t CONTENT { id: includes:accv7r0adjfvi9axnn8s, in: faculty:zkifk1lew7goyi1bn0lm, out: school:qr4i045651hejltikx9t };

-- ------------------------------
-- TABLE DATA: offers
-- ------------------------------

RELATE school:qr4i045651hejltikx9t -> offers:27ixpxzcp8mx7sry1oci -> course:kp7n2n27oedkvphduv29 CONTENT { id: offers:27ixpxzcp8mx7sry1oci, in: school:qr4i045651hejltikx9t, out: course:kp7n2n27oedkvphduv29 };
RELATE school:qr4i045651hejltikx9t -> offers:te2use9pmcobevoctjow -> course:f04lf2tqlor5jg2qt4ej CONTENT { id: offers:te2use9pmcobevoctjow, in: school:qr4i045651hejltikx9t, out: course:f04lf2tqlor5jg2qt4ej };

-- ------------------------------
-- TABLE DATA: professor
-- ------------------------------

UPDATE professor:0 CONTENT { dni: '00000000', gender: true, id: professor:0, last_name1: 'Mundo', last_name2: 'Mundo', names: 'Mundo', password: '$2b$12$kFgsrW/5EKMQIQyRklLQGOOcQqQT0nZd5NRbm/4iKq/npLRF2cFuO', role: 'professor' };
UPDATE professor:12345670 CONTENT { dni: '12345670', gender: true, id: professor:12345670, last_name1: 'Torrest', last_name2: 'Tarrillo', names: 'Rober Esbl', password: '$2b$12$4sN592wOQXjFjHHPkDhfJ.u/7fFEhhr.KnanebRnBw2qG0vrw4gG2', role: 'practice' };
UPDATE professor:12345671 CONTENT { dni: '12345671', gender: true, id: professor:12345671, last_name1: 'Torrest', last_name2: 'Tarrillo', names: 'Rober Esbl', password: '$2b$12$5IeP/bdngAuxvTybDvDD.OJtALN4WoOLUYanuHzD53OYvoRCI0zqe', role: 'practice' };
UPDATE professor:12345676 CONTENT { dni: '12345676', gender: true, id: professor:12345676, last_name1: 'Torrest', last_name2: 'Tarrillo', names: 'Rober Esbl', password: '$2b$12$KukGy1g9L4A.ebc.SqtkWuboYnpQNDOtC9MgSZZSZPu.DiHK26Ue.', role: 'practice' };
UPDATE professor:12345678 CONTENT { dni: '12345678', gender: true, id: professor:12345678, last_name1: 'Torrest', last_name2: 'Tarrillo', names: 'Admin', password: '$2b$12$5MJCJWYe/82HRPjVTAf4Qe.Y9j6IVLARQXsLiToySW7vxes0.3MFG', role: 'admin' };
UPDATE professor:12345679 CONTENT { dni: '12345679', gender: true, id: professor:12345679, last_name1: 'Torrest', last_name2: 'Tarrillo', names: 'Rober Esbl', password: '$2b$12$uep5RlLfssWtxfhJEVVibO.hFsYhwVWQg.u6eW4AjGCxwzI5aOP.m', role: 'practice' };
UPDATE professor:87654321 CONTENT { dni: '87654321', gender: true, id: professor:87654321, last_name1: 'Práctica', last_name2: 'Practica', names: 'Profesor', password: '$2b$12$XZr5EkIiSPkzbElYUI1nLu6r.YKytGhIKAdAEjW9UD2rUfC4J0BFa', role: 'professor' };

-- ------------------------------
-- TABLE DATA: register_time
-- ------------------------------

UPDATE register_time:1op4nqee1q8inr51z6x3 CONTENT { end: '2025-12-15T00:00:00Z', id: register_time:1op4nqee1q8inr51z6x3, start: '2024-09-16T00:43:24.735Z', todo: 'professor' };
UPDATE register_time:5te8i9be9ctx74cir6pr CONTENT { end: '2025-12-15T00:00:00Z', id: register_time:5te8i9be9ctx74cir6pr, start: '2024-09-16T00:34:56.831Z', todo: 'student' };
UPDATE register_time:8gs49yroxq7wig4x2t13 CONTENT { end: '2025-12-15T00:00:00Z', id: register_time:8gs49yroxq7wig4x2t13, start: '2024-09-16T00:39:13.774Z', todo: 'student' };
UPDATE register_time:9z1ygtfhe08zs4t7q9by CONTENT { end: '2025-12-15T00:00:00Z', id: register_time:9z1ygtfhe08zs4t7q9by, start: '2024-09-16T00:36:49.142Z', todo: 'student' };
UPDATE register_time:a3p57ctuh8p0pjw8vjiu CONTENT { end: '2025-12-15T00:00:00Z', id: register_time:a3p57ctuh8p0pjw8vjiu, start: '2024-09-16T00:36:52.167Z', todo: 'student' };
UPDATE register_time:ct87t4bdse4shsggdw83 CONTENT { end: '2025-12-15T00:00:00Z', id: register_time:ct87t4bdse4shsggdw83, start: '2024-09-16T00:51:05.724Z', todo: '1' };
UPDATE register_time:fvjw1yoqm2gp7l7ndvrv CONTENT { end: '2025-12-15T00:00:00Z', id: register_time:fvjw1yoqm2gp7l7ndvrv, start: '2024-09-16T00:36:48.259Z', todo: 'student' };
UPDATE register_time:j4fm951r1i8pc653zabu CONTENT { end: '2025-12-15T00:00:00Z', id: register_time:j4fm951r1i8pc653zabu, start: '2024-09-16T00:36:47.256Z', todo: 'student' };
UPDATE register_time:vvgbtbxrbqgj76qgpy3m CONTENT { end: '2025-12-15T00:00:00Z', id: register_time:vvgbtbxrbqgj76qgpy3m, start: '2024-09-16T00:39:11.658Z', todo: 'student' };

-- ------------------------------
-- TABLE DATA: school
-- ------------------------------

UPDATE school:qr4i045651hejltikx9t CONTENT { id: school:qr4i045651hejltikx9t, name: 'FÍSICA' };

-- ------------------------------
-- TABLE DATA: student
-- ------------------------------

UPDATE student:0 CONTENT { code: '0000000000', gender: true, id: student:0, last_name1: 'Mundo', last_name2: 'Mundo', names: 'Mundo', password: '$2b$12$W7bSdTCgOOUBJTLoOZzqHuH1RrVK4GyhPECkHYSOKtAF2sgolHS5q' };
UPDATE student:987654321 CONTENT { code: '0987654321', gender: true, id: student:987654321, last_name1: 'Vallejos', last_name2: 'Fernandez', names: 'Daniela', password: '$2b$12$r8Ysf8guZ.aA4VMDr5ob2uY6mM5ppdxrIMSIVTTwd5nkW.y4fHxs.' };
UPDATE student:1234567890 CONTENT { code: '1234567890', gender: true, id: student:1234567890, last_name1: 'Torres', last_name2: 'Tarrillo', names: 'Rober', password: '$2b$12$rbFu6uIp7T32dAG5zo3STeF3.dhx.GfveV.fs0LJn9VFRUGWzU7hS' };
UPDATE student:2222222222 CONTENT { code: '2222222222', gender: true, id: student:2222222222, last_name1: 'Hawking', last_name2: 'Hawking', names: 'Stephen', password: '$2b$12$NWA/vhHe.0TvYo4hG9qXc.MQyoghx9vqJ4R/9fbQ093aOh1obHAr2' };
UPDATE student:3333333333 CONTENT { code: '3333333333', gender: true, id: student:3333333333, last_name1: 'Estudiante', last_name2: 'Suscrito', names: 'Otro', password: '$2b$12$OCjr39zbb4WihlH5dZLp7eGuzvPOU4O9ap3HLFLdWfvkm3o/3apwm' };
UPDATE student:4444444444 CONTENT { code: '4444444444', gender: false, id: student:4444444444, last_name1: 'Vallejos', last_name2: 'Fernandez', names: 'Daniela Elizabeth', password: '$2b$12$3ZTcIOg/TukPn/cQfuXImuDtAnVyhLjX5CRdtvFIHMhki8EPU2ga.' };

-- ------------------------------
-- TABLE DATA: teaches
-- ------------------------------

RELATE professor:12345676 -> teaches:a8s5aejs05th2tx72s00 -> course:kp7n2n27oedkvphduv29 CONTENT { id: teaches:a8s5aejs05th2tx72s00, in: professor:12345676, out: course:kp7n2n27oedkvphduv29, role: 'theory' };
RELATE professor:87654321 -> teaches:bcaxgpgsih9qsi9oawd1 -> course:kp7n2n27oedkvphduv29 CONTENT { id: teaches:bcaxgpgsih9qsi9oawd1, in: professor:87654321, out: course:kp7n2n27oedkvphduv29, role: 'practice' };
RELATE professor:87654321 -> teaches:d719ubr0d521vjipp1k8 -> course:f04lf2tqlor5jg2qt4ej CONTENT { id: teaches:d719ubr0d521vjipp1k8, in: professor:87654321, out: course:f04lf2tqlor5jg2qt4ej, role: 'practice' };

-- ------------------------------
-- TABLE DATA: test_type
-- ------------------------------

UPDATE test_type:2mdrak6vl42xdjswav6a CONTENT { id: test_type:2mdrak6vl42xdjswav6a, name: 'Exámen' };

-- ------------------------------
-- TRANSACTION
-- ------------------------------

COMMIT TRANSACTION;
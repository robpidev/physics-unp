-- ------------------------------
-- OPTION
-- ------------------------------

OPTION IMPORT;

-- ------------------------------
-- TABLE: course
-- ------------------------------

DEFINE TABLE course TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD name ON course TYPE string PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD places ON course TYPE int PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD tests ON course FLEXIBLE TYPE array<object> PERMISSIONS FULL;
DEFINE FIELD tests[*] ON course FLEXIBLE TYPE object PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: course
-- ------------------------------

BEGIN;
INSERT { id: course:1d0gavs1xe39bw96r4fn, name: 'Lab. Física Iii (C)', places: 20, tests: [{ name: 'test', weight: 50 }, { name: 'práctice', weight: 50 }] } VERSION d'2025-11-26T09:35:12.719928244Z';
INSERT { id: course:1x1yegm6onvblnxb7swj, name: 'Lab. Física Iii (C)', places: 20, tests: [{ name: 'test', weight: 50 }, { name: 'práctice', weight: 50 }] } VERSION d'2025-11-26T09:35:15.000227512Z';
INSERT { id: course:b5v78c9rd7gp3yxme1em, name: 'Lab. Física Iii (C)', places: 20, tests: [{ name: 'test', weight: 50 }, { name: 'práctice', weight: 50 }] } VERSION d'2025-11-26T09:35:15.711177456Z';
INSERT { id: course:e7xkrl0vyybovg9lynr6, name: 'Lab. Física Iii (C)', places: 20, tests: [{ name: 'test', weight: 50 }, { name: 'práctice', weight: 50 }] } VERSION d'2025-11-26T09:35:16.259540260Z';
INSERT { id: course:eo3i4v37b39mzixwon87, name: 'Lab. Física Iii (C)', places: 20, tests: [{ name: 'test', weight: 50 }, { name: 'práctice', weight: 50 }] } VERSION d'2025-11-26T09:35:36.687529383Z';
INSERT { id: course:f04lf2tqlor5jg2qt4ej, name: 'Física II', places: 100, tests: [{ name: 'test', weight: 50 }, { name: 'práctice', weight: 50 }] } VERSION d'2025-11-26T09:28:35.033996984Z';
INSERT { id: course:gx6gqqbm31gyumrcutk9, name: 'Lab. Física Iii (C)', places: 20, tests: [{ name: 'test', weight: 50 }, { name: 'práctice', weight: 50 }] } VERSION d'2025-11-26T09:35:37.013518630Z';
INSERT { id: course:k0gac4kdjbl99swqhpf2, name: 'Lab. Física Iii (C)', places: 20, tests: [{ name: 'test', weight: 50 }, { name: 'práctice', weight: 50 }] } VERSION d'2025-11-26T09:35:30.628099673Z';
INSERT { id: course:kp7n2n27oedkvphduv29, name: 'Física I', places: 100, tests: [{ name: 'test', weight: 50 }, { name: 'practice', weight: 50 }] } VERSION d'2025-11-26T09:28:35.033996984Z';
INSERT { id: course:or8qolp16fnrf546bia9, name: 'Lab. Física Iii (C)', places: 20, tests: [{ name: 'test', weight: 50 }, { name: 'práctice', weight: 50 }] } VERSION d'2025-11-26T09:35:37.306135467Z';
INSERT { id: course:pw1u2tskvh0ui3dbourh, name: 'Lab. Física Iii (C)', places: 20, tests: [{ name: 'test', weight: 50 }, { name: 'práctice', weight: 50 }] } VERSION d'2025-11-26T09:35:13.715750869Z';
INSERT { id: course:r204o46b2er81ft1t9ws, name: 'Lab. Física Iii (C)', places: 20, tests: [{ name: 'test', weight: 50 }, { name: 'práctice', weight: 50 }] } VERSION d'2025-11-26T09:35:17.061202709Z';
INSERT { id: course:swc1h9j0x0hhe1pznfd5, name: 'Lab. Física Iii (C)', places: 20, tests: [{ name: 'test', weight: 50 }, { name: 'práctice', weight: 50 }] } VERSION d'2025-11-26T09:35:35.981522044Z';
COMMIT;

-- ------------------------------
-- TABLE: divided
-- ------------------------------

DEFINE TABLE divided TYPE RELATION IN semester OUT week SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON divided TYPE record<semester> PERMISSIONS FULL;
DEFINE FIELD out ON divided TYPE record<week> PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: divided
-- ------------------------------


-- ------------------------------
-- TABLE: enrolled
-- ------------------------------

DEFINE TABLE enrolled TYPE RELATION IN student OUT course SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON enrolled TYPE record<student> PERMISSIONS FULL;
DEFINE FIELD out ON enrolled TYPE record<course> PERMISSIONS FULL;

DEFINE INDEX course_index ON enrolled FIELDS in UNIQUE;


-- ------------------------------
-- TABLE DATA: enrolled
-- ------------------------------

BEGIN;
COMMIT;
BEGIN;
INSERT RELATION { __: true, id: enrolled:953padbremko0xybmc5a, in: student:2222222222, out: course:kp7n2n27oedkvphduv29 } VERSION d'2025-11-26T09:28:35.035637560Z';
INSERT RELATION { __: true, id: enrolled:ala4j9z8nkja7i51kun8, in: student:4444444444, out: course:kp7n2n27oedkvphduv29 } VERSION d'2025-11-26T09:28:35.035637560Z';
INSERT RELATION { __: true, id: enrolled:ym3lpw1pnespfhs4xhoz, in: student:1234567890, out: course:f04lf2tqlor5jg2qt4ej } VERSION d'2025-11-26T09:28:35.035637560Z';
COMMIT;

-- ------------------------------
-- TABLE: evaluated
-- ------------------------------

DEFINE TABLE evaluated TYPE RELATION IN course OUT student SCHEMALESS PERMISSIONS NONE;

DEFINE FIELD ev_type ON evaluated TYPE string PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD in ON evaluated TYPE record<course> PERMISSIONS FULL;
DEFINE FIELD number ON evaluated TYPE int PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD out ON evaluated TYPE record<student> PERMISSIONS FULL;
DEFINE FIELD score ON evaluated TYPE float ASSERT 0 <= score AND score <= 20 PERMISSIONS FOR select, create, update WHERE FULL;

DEFINE INDEX index_id ON evaluated FIELDS id UNIQUE;


-- ------------------------------
-- TABLE DATA: evaluated
-- ------------------------------

BEGIN;
COMMIT;
BEGIN;
INSERT RELATION { __: true, ev_type: 'test', id: evaluated:5eyke0d5h26ah7q16f6t, in: course:kp7n2n27oedkvphduv29, number: 2, out: student:2222222222, score: 10 } VERSION d'2025-11-26T09:28:35.036483826Z';
INSERT RELATION { __: true, ev_type: 'practice', id: evaluated:bh7y9tryubfce8kytal5, in: course:f04lf2tqlor5jg2qt4ej, number: 2, out: student:1234567890, score: 3f } VERSION d'2025-11-26T09:28:35.036483826Z';
INSERT RELATION { __: true, ev_type: 'test', id: evaluated:ced61p95awurfljefnzz, in: course:f04lf2tqlor5jg2qt4ej, number: 2, out: student:1234567890, score: 2f } VERSION d'2025-11-26T09:28:35.036483826Z';
INSERT RELATION { __: true, ev_type: 'practice', id: evaluated:ebz0n3zkdco0j9sjq5ir, in: course:kp7n2n27oedkvphduv29, number: 1, out: student:2222222222, score: 2 } VERSION d'2025-11-26T09:28:35.036483826Z';
INSERT RELATION { __: true, ev_type: 'practice', id: evaluated:ivn83fil77vt1a2ir159, in: course:kp7n2n27oedkvphduv29, number: 2, out: student:2222222222, score: 13 } VERSION d'2025-11-26T09:28:35.036483826Z';
INSERT RELATION { __: true, ev_type: 'test', id: evaluated:l8upun5g6z45swst90n7, in: course:kp7n2n27oedkvphduv29, number: 3, out: student:4444444444, score: 12 } VERSION d'2025-11-26T09:28:35.036483826Z';
INSERT RELATION { __: true, ev_type: 'test', id: evaluated:lgwj6duub1rl4j8h00yg, in: course:kp7n2n27oedkvphduv29, number: 2, out: student:4444444444, score: 4 } VERSION d'2025-11-26T09:28:35.036483826Z';
INSERT RELATION { __: true, ev_type: 'test', id: evaluated:lnzrb5bmdyb2gre5otut, in: course:kp7n2n27oedkvphduv29, number: 2, out: student:2222222222, score: 12f } VERSION d'2025-11-26T09:28:35.036483826Z';
INSERT RELATION { __: true, ev_type: 'practice', id: evaluated:mr074vy6vuyoz7htvahi, in: course:kp7n2n27oedkvphduv29, number: 1, out: student:4444444444, score: 9 } VERSION d'2025-11-26T09:28:35.036483826Z';
INSERT RELATION { __: true, ev_type: 'practice', id: evaluated:pjnnbt9f264tvqvjfwi0, in: course:kp7n2n27oedkvphduv29, number: 3, out: student:2222222222, score: 12 } VERSION d'2025-11-26T09:28:35.036483826Z';
INSERT RELATION { __: true, ev_type: 'test', id: evaluated:qjsfrjk64bzyw67p850a, in: course:kp7n2n27oedkvphduv29, number: 1, out: student:2222222222, score: 20 } VERSION d'2025-11-26T09:28:35.036483826Z';
INSERT RELATION { __: true, ev_type: 'test', id: evaluated:yxf6ew3h9q9hwbez6rve, in: course:kp7n2n27oedkvphduv29, number: 1, out: student:4444444444, score: 13 } VERSION d'2025-11-26T09:28:35.036483826Z';
COMMIT;

-- ------------------------------
-- TABLE: faculty
-- ------------------------------

DEFINE TABLE faculty TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD name ON faculty TYPE string VALUE string::uppercase(name) PERMISSIONS FOR select, create, update WHERE FULL;

DEFINE INDEX index_name ON faculty FIELDS name UNIQUE;


-- ------------------------------
-- TABLE DATA: faculty
-- ------------------------------

BEGIN;
INSERT { id: faculty:vlf3cz01hjbfbulrld3e, name: 'HOLA' } VERSION d'2025-11-26T09:28:35.036869217Z';
INSERT { id: faculty:zkifk1lew7goyi1bn0lm, name: 'CIENCIAS' } VERSION d'2025-11-26T09:28:35.036869217Z';
COMMIT;

-- ------------------------------
-- TABLE: has
-- ------------------------------

DEFINE TABLE has TYPE RELATION IN school OUT student SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON has TYPE record<school> PERMISSIONS FULL;
DEFINE FIELD out ON has TYPE record<student> PERMISSIONS FULL;

DEFINE INDEX index_student ON has FIELDS out UNIQUE;


-- ------------------------------
-- TABLE DATA: has
-- ------------------------------

BEGIN;
COMMIT;
BEGIN;
INSERT RELATION { __: true, id: has:44a6tfk79o2xs522vzdu, in: school:qr4i045651hejltikx9t, out: student:2222222222 } VERSION d'2025-11-26T09:28:35.037221815Z';
INSERT RELATION { __: true, id: has:bxspnzgn5n00907rekoe, in: school:qr4i045651hejltikx9t, out: student:3333333333 } VERSION d'2025-11-26T09:28:35.037221815Z';
INSERT RELATION { __: true, id: has:dixhsc5dz9j9m9gfk7yi, in: school:qr4i045651hejltikx9t, out: student:0 } VERSION d'2025-11-26T09:28:35.037221815Z';
INSERT RELATION { __: true, id: has:fqoxvh22145udeve63xh, in: school:qr4i045651hejltikx9t, out: student:4444444444 } VERSION d'2025-11-26T09:28:35.037221815Z';
INSERT RELATION { __: true, id: has:nuj9g7m8xo3pey9sngfy, in: school:qr4i045651hejltikx9t, out: student:987654321 } VERSION d'2025-11-26T09:28:35.037221815Z';
INSERT RELATION { __: true, id: has:qkqyzw0qw452f31m989e, in: school:qr4i045651hejltikx9t, out: student:1234567890 } VERSION d'2025-11-26T09:28:35.037221815Z';
COMMIT;

-- ------------------------------
-- TABLE: includes
-- ------------------------------

DEFINE TABLE includes TYPE RELATION IN faculty OUT school SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON includes TYPE record<faculty> PERMISSIONS FULL;
DEFINE FIELD out ON includes TYPE record<school> PERMISSIONS FULL;

DEFINE INDEX index_school ON includes FIELDS out UNIQUE;


-- ------------------------------
-- TABLE DATA: includes
-- ------------------------------

BEGIN;
COMMIT;
BEGIN;
INSERT RELATION { __: true, id: includes:accv7r0adjfvi9axnn8s, in: faculty:zkifk1lew7goyi1bn0lm, out: school:qr4i045651hejltikx9t } VERSION d'2025-11-26T09:28:35.037424403Z';
COMMIT;

-- ------------------------------
-- TABLE: notice
-- ------------------------------

DEFINE TABLE notice TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD datetime ON notice DEFAULT time::now() PERMISSIONS FULL;
DEFINE FIELD note ON notice TYPE string PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: notice
-- ------------------------------

BEGIN;
INSERT { datetime: d'2025-11-26T09:45:21.306145728Z', id: notice:0nyftlyfeh4vl44uxo25, note: '"probado"' } VERSION d'2025-11-26T09:45:21.306965733Z';
INSERT { datetime: d'2025-11-26T10:02:36.656293968Z', id: notice:1mjeswpwyyp7tyzl0hzj, note: 'Todo va bien' } VERSION d'2025-11-26T10:02:36.656373149Z';
INSERT { datetime: d'2025-11-26T10:02:34.206994407Z', id: notice:62otbdj4glwtnorwzux8, note: 'Todo va bien' } VERSION d'2025-11-26T10:02:34.207091156Z';
INSERT { datetime: d'2025-11-26T09:45:37.211559470Z', id: notice:iqxvk5vq0accqoaxa56i, note: 'Todo va bien' } VERSION d'2025-11-26T09:45:37.211632616Z';
INSERT { datetime: d'2025-11-26T10:03:53.143460959Z', id: notice:tt4osr7fu3csavfwtogf, note: 'Todo va bien' } VERSION d'2025-11-26T10:03:53.143554409Z';
COMMIT;

-- ------------------------------
-- TABLE: notices
-- ------------------------------

DEFINE TABLE notices TYPE ANY SCHEMALESS PERMISSIONS NONE;

DEFINE FIELD datetime ON notices DEFAULT time::now() READONLY PERMISSIONS FULL;
DEFINE FIELD note ON notices TYPE string PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: notices
-- ------------------------------

BEGIN;
INSERT { datetime: d'2025-11-26T09:31:38.260398800Z', id: notices:8r7fyvwlekcey7yjgm4h, note: 'test' } VERSION d'2025-11-26T09:31:38.260472418Z';
INSERT { datetime: d'2025-11-26T09:31:38.161069138Z', id: notices:bxqnqjbpieubjsb8iery, note: 'test' } VERSION d'2025-11-26T09:31:38.161186934Z';
INSERT { id: notices:grsqyb41ay99a1kn5ouk, note: 'test' } VERSION d'2025-11-26T09:30:42.795255646Z';
INSERT { datetime: d'2025-11-26T09:31:37.814618253Z', id: notices:ibxcr8aeyijzfuqbzmv6, note: 'test' } VERSION d'2025-11-26T09:31:37.814703791Z';
INSERT { id: notices:iqeuc66x97og2y3n4vwv, note: 'test' } VERSION d'2025-11-26T09:30:42.273293009Z';
INSERT { id: notices:jbxci3ec5umkx7vx5r49, note: 'test' } VERSION d'2025-11-26T09:30:37.527073827Z';
INSERT { id: notices:m71116bm0fevym7zh7og, note: 'test' } VERSION d'2025-11-26T09:30:41.761485036Z';
INSERT { datetime: d'2025-11-26T09:31:37.180781961Z', id: notices:po6aihdvzp1t5r7pbia8, note: 'test' } VERSION d'2025-11-26T09:31:37.180916913Z';
INSERT { id: notices:xm3tfwle6qpwfszntf06, note: 'test' } VERSION d'2025-11-26T09:30:41.087378744Z';
INSERT { datetime: d'2025-11-26T09:31:38.004340939Z', id: notices:y51qajf7u0ust8099u8n, note: 'test' } VERSION d'2025-11-26T09:31:38.004461241Z';
INSERT { id: notices:ze0m4jtr3c9hlfdvh2u7, note: 'test' } VERSION d'2025-11-26T09:30:41.550055497Z';
COMMIT;

-- ------------------------------
-- TABLE: offers
-- ------------------------------

DEFINE TABLE offers TYPE RELATION IN school OUT course SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON offers TYPE record<school> PERMISSIONS FULL;
DEFINE FIELD out ON offers TYPE record<course> PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: offers
-- ------------------------------

BEGIN;
COMMIT;
BEGIN;
INSERT RELATION { __: true, id: offers:27ixpxzcp8mx7sry1oci, in: school:qr4i045651hejltikx9t, out: course:kp7n2n27oedkvphduv29 } VERSION d'2025-11-26T09:28:35.037558947Z';
INSERT RELATION { __: true, id: offers:4m86j0fvy1cf8pn0j1tj, in: school:qr4i045651hejltikx9t, out: course:pw1u2tskvh0ui3dbourh } VERSION d'2025-11-26T09:35:13.715750869Z';
INSERT RELATION { __: true, id: offers:93gbh5e3m7kkd1d2b3wn, in: school:qr4i045651hejltikx9t, out: course:gx6gqqbm31gyumrcutk9 } VERSION d'2025-11-26T09:35:37.013518630Z';
INSERT RELATION { __: true, id: offers:9u1tdwfaag68x0n1d39i, in: school:qr4i045651hejltikx9t, out: course:1d0gavs1xe39bw96r4fn } VERSION d'2025-11-26T09:35:12.719928244Z';
INSERT RELATION { __: true, id: offers:cv791herun4awtkib069, in: school:qr4i045651hejltikx9t, out: course:r204o46b2er81ft1t9ws } VERSION d'2025-11-26T09:35:17.061202709Z';
INSERT RELATION { __: true, id: offers:enu1us7lrvo5s9zufmxt, in: school:qr4i045651hejltikx9t, out: course:k0gac4kdjbl99swqhpf2 } VERSION d'2025-11-26T09:35:30.628099673Z';
INSERT RELATION { __: true, id: offers:fz2s5ledu6or6fllntx1, in: school:qr4i045651hejltikx9t, out: course:b5v78c9rd7gp3yxme1em } VERSION d'2025-11-26T09:35:15.711177456Z';
INSERT RELATION { __: true, id: offers:lvnggxqhbaryd0v259xu, in: school:qr4i045651hejltikx9t, out: course:eo3i4v37b39mzixwon87 } VERSION d'2025-11-26T09:35:36.687529383Z';
INSERT RELATION { __: true, id: offers:nf07rivg3ji82n75pid4, in: school:qr4i045651hejltikx9t, out: course:or8qolp16fnrf546bia9 } VERSION d'2025-11-26T09:35:37.306135467Z';
INSERT RELATION { __: true, id: offers:p2d60pw83oh1ruxu9i1o, in: school:qr4i045651hejltikx9t, out: course:swc1h9j0x0hhe1pznfd5 } VERSION d'2025-11-26T09:35:35.981522044Z';
INSERT RELATION { __: true, id: offers:te2use9pmcobevoctjow, in: school:qr4i045651hejltikx9t, out: course:f04lf2tqlor5jg2qt4ej } VERSION d'2025-11-26T09:28:35.037558947Z';
INSERT RELATION { __: true, id: offers:z8rqxwyyoffwixaldhx9, in: school:qr4i045651hejltikx9t, out: course:1x1yegm6onvblnxb7swj } VERSION d'2025-11-26T09:35:15.000227512Z';
INSERT RELATION { __: true, id: offers:zpq3gbhvh5odo6687e7o, in: school:qr4i045651hejltikx9t, out: course:e7xkrl0vyybovg9lynr6 } VERSION d'2025-11-26T09:35:16.259540260Z';
COMMIT;

-- ------------------------------
-- TABLE: professor
-- ------------------------------

DEFINE TABLE professor TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD dni ON professor TYPE string ASSERT string::len(dni) == 8 AND string::is::numeric(dni) PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD gender ON professor TYPE bool PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD last_name1 ON professor TYPE string PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD last_name2 ON professor TYPE string PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD names ON professor TYPE string VALUE string::trim(names) PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD password ON professor TYPE string PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD role ON professor TYPE string DEFAULT 'professor' PERMISSIONS FOR select, create, update WHERE FULL;

DEFINE INDEX index_dni ON professor FIELDS dni UNIQUE;


-- ------------------------------
-- TABLE DATA: professor
-- ------------------------------

BEGIN;
INSERT { dni: '00000000', gender: true, id: professor:0, last_name1: 'Mundo', last_name2: 'Mundo', names: 'Mundo', password: '$2b$12$kFgsrW/5EKMQIQyRklLQGOOcQqQT0nZd5NRbm/4iKq/npLRF2cFuO', role: 'professor' } VERSION d'2025-11-26T09:28:35.037833273Z';
INSERT { dni: '12345670', gender: true, id: professor:12345670, last_name1: 'Torrest', last_name2: 'Tarrillo', names: 'Rober Esbl', password: '$2b$12$4sN592wOQXjFjHHPkDhfJ.u/7fFEhhr.KnanebRnBw2qG0vrw4gG2', role: 'practice' } VERSION d'2025-11-26T09:28:35.037833273Z';
INSERT { dni: '12345671', gender: true, id: professor:12345671, last_name1: 'Torrest', last_name2: 'Tarrillo', names: 'Rober Esbl', password: '$2b$12$5IeP/bdngAuxvTybDvDD.OJtALN4WoOLUYanuHzD53OYvoRCI0zqe', role: 'practice' } VERSION d'2025-11-26T09:28:35.037833273Z';
INSERT { dni: '12345676', gender: true, id: professor:12345676, last_name1: 'Torrest', last_name2: 'Tarrillo', names: 'Rober Esbl', password: '$2b$12$KukGy1g9L4A.ebc.SqtkWuboYnpQNDOtC9MgSZZSZPu.DiHK26Ue.', role: 'practice' } VERSION d'2025-11-26T09:28:35.037833273Z';
INSERT { dni: '12345678', gender: true, id: professor:12345678, last_name1: 'Torrest', last_name2: 'Tarrillo', names: 'Admin', password: '$2b$12$5MJCJWYe/82HRPjVTAf4Qe.Y9j6IVLARQXsLiToySW7vxes0.3MFG', role: 'admin' } VERSION d'2025-11-26T09:28:35.037833273Z';
INSERT { dni: '12345679', gender: true, id: professor:12345679, last_name1: 'Torrest', last_name2: 'Tarrillo', names: 'Rober Esbl', password: '$2b$12$uep5RlLfssWtxfhJEVVibO.hFsYhwVWQg.u6eW4AjGCxwzI5aOP.m', role: 'practice' } VERSION d'2025-11-26T09:28:35.037833273Z';
INSERT { dni: '87654321', gender: true, id: professor:87654321, last_name1: 'Práctica', last_name2: 'Practica', names: 'Profesor', password: '$2b$12$XZr5EkIiSPkzbElYUI1nLu6r.YKytGhIKAdAEjW9UD2rUfC4J0BFa', role: 'professor' } VERSION d'2025-11-26T09:28:35.037833273Z';
COMMIT;

-- ------------------------------
-- TABLE: programmed
-- ------------------------------

DEFINE TABLE programmed TYPE RELATION IN course OUT semester SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON programmed TYPE record<course> PERMISSIONS FULL;
DEFINE FIELD out ON programmed TYPE record<semester> PERMISSIONS FULL;



-- ------------------------------
-- TABLE DATA: programmed
-- ------------------------------


-- ------------------------------
-- TABLE: register_time
-- ------------------------------

DEFINE TABLE register_time TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD end ON register_time TYPE datetime PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD start ON register_time TYPE datetime DEFAULT time::now() PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD todo ON register_time TYPE string PERMISSIONS FOR select, create, update WHERE FULL;



-- ------------------------------
-- TABLE DATA: register_time
-- ------------------------------

BEGIN;
INSERT { end: '2025-12-15T00:00:00Z', id: register_time:1op4nqee1q8inr51z6x3, start: '2024-09-16T00:43:24.735Z', todo: 'professor' } VERSION d'2025-11-26T09:28:35.038088233Z';
INSERT { end: '2025-12-15T00:00:00Z', id: register_time:5te8i9be9ctx74cir6pr, start: '2024-09-16T00:34:56.831Z', todo: 'student' } VERSION d'2025-11-26T09:28:35.038088233Z';
INSERT { end: '2025-12-15T00:00:00Z', id: register_time:8gs49yroxq7wig4x2t13, start: '2024-09-16T00:39:13.774Z', todo: 'student' } VERSION d'2025-11-26T09:28:35.038088233Z';
INSERT { end: '2025-12-15T00:00:00Z', id: register_time:9z1ygtfhe08zs4t7q9by, start: '2024-09-16T00:36:49.142Z', todo: 'student' } VERSION d'2025-11-26T09:28:35.038088233Z';
INSERT { end: '2025-12-15T00:00:00Z', id: register_time:a3p57ctuh8p0pjw8vjiu, start: '2024-09-16T00:36:52.167Z', todo: 'student' } VERSION d'2025-11-26T09:28:35.038088233Z';
INSERT { end: '2025-12-15T00:00:00Z', id: register_time:ct87t4bdse4shsggdw83, start: '2024-09-16T00:51:05.724Z', todo: '1' } VERSION d'2025-11-26T09:28:35.038088233Z';
INSERT { end: d'2024-10-18T04:59:00Z', id: register_time:f7h141fiki8ofofqify9, start: d'2024-10-18T00:53:38.127378300Z', todo: '2' } VERSION d'2025-11-26T09:28:35.038088233Z';
INSERT { end: '2025-12-15T00:00:00Z', id: register_time:fvjw1yoqm2gp7l7ndvrv, start: '2024-09-16T00:36:48.259Z', todo: 'student' } VERSION d'2025-11-26T09:28:35.038088233Z';
INSERT { end: '2025-12-15T00:00:00Z', id: register_time:j4fm951r1i8pc653zabu, start: '2024-09-16T00:36:47.256Z', todo: 'student' } VERSION d'2025-11-26T09:28:35.038088233Z';
INSERT { end: '2025-12-15T00:00:00Z', id: register_time:vvgbtbxrbqgj76qgpy3m, start: '2024-09-16T00:39:11.658Z', todo: 'student' } VERSION d'2025-11-26T09:28:35.038088233Z';
COMMIT;

-- ------------------------------
-- TABLE: school
-- ------------------------------

DEFINE TABLE school TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD name ON school TYPE string VALUE string::uppercase(name) PERMISSIONS FOR select, create, update WHERE FULL;

DEFINE INDEX index_name ON school FIELDS name UNIQUE;


-- ------------------------------
-- TABLE DATA: school
-- ------------------------------

BEGIN;
INSERT { id: school:qr4i045651hejltikx9t, name: 'FÍSICA' } VERSION d'2025-11-26T09:28:35.038179623Z';
COMMIT;

-- ------------------------------
-- TABLE: semester
-- ------------------------------

DEFINE TABLE semester TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;




-- ------------------------------
-- TABLE DATA: semester
-- ------------------------------


-- ------------------------------
-- TABLE: student
-- ------------------------------

DEFINE TABLE student TYPE NORMAL SCHEMALESS PERMISSIONS NONE;

DEFINE FIELD code ON student TYPE string ASSERT string::is::numeric(code) AND string::len(code) == 10 PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD gender ON student TYPE bool PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD last_name1 ON student TYPE string ASSERT string::is::alpha(last_name1) PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD last_name2 ON student TYPE string ASSERT string::is::alpha(last_name2) PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD names ON student TYPE string PERMISSIONS FOR select, create, update WHERE FULL;
DEFINE FIELD password ON student TYPE string PERMISSIONS FOR select, create, update WHERE FULL;

DEFINE INDEX index_code ON student FIELDS code UNIQUE;


-- ------------------------------
-- TABLE DATA: student
-- ------------------------------

BEGIN;
INSERT { code: '0000000000', gender: true, id: student:0, last_name1: 'Mundo', last_name2: 'Mundo', names: 'Mundo', password: '$2b$12$W7bSdTCgOOUBJTLoOZzqHuH1RrVK4GyhPECkHYSOKtAF2sgolHS5q' } VERSION d'2025-11-26T09:28:35.038358131Z';
INSERT { code: '0987654321', gender: true, id: student:987654321, last_name1: 'Vallejos', last_name2: 'Fernandez', names: 'Daniela', password: '$2b$12$r8Ysf8guZ.aA4VMDr5ob2uY6mM5ppdxrIMSIVTTwd5nkW.y4fHxs.' } VERSION d'2025-11-26T09:28:35.038358131Z';
INSERT { code: '1234567890', gender: true, id: student:1234567890, last_name1: 'Torres', last_name2: 'Tarrillo', names: 'Rober', password: '$2b$12$rbFu6uIp7T32dAG5zo3STeF3.dhx.GfveV.fs0LJn9VFRUGWzU7hS' } VERSION d'2025-11-26T09:28:35.038358131Z';
INSERT { code: '2222222222', gender: true, id: student:2222222222, last_name1: 'Hawking', last_name2: 'Hawking', names: 'Stephen', password: '$2b$12$NWA/vhHe.0TvYo4hG9qXc.MQyoghx9vqJ4R/9fbQ093aOh1obHAr2' } VERSION d'2025-11-26T09:28:35.038358131Z';
INSERT { code: '3333333333', gender: true, id: student:3333333333, last_name1: 'Estudiante', last_name2: 'Suscrito', names: 'Otro', password: '$2b$12$OCjr39zbb4WihlH5dZLp7eGuzvPOU4O9ap3HLFLdWfvkm3o/3apwm' } VERSION d'2025-11-26T09:28:35.038358131Z';
INSERT { code: '4444444444', gender: false, id: student:4444444444, last_name1: 'Vallejos', last_name2: 'Fernandez', names: 'Daniela Elizabeth', password: '$2b$12$3ZTcIOg/TukPn/cQfuXImuDtAnVyhLjX5CRdtvFIHMhki8EPU2ga.' } VERSION d'2025-11-26T09:28:35.038358131Z';
COMMIT;

-- ------------------------------
-- TABLE: teaches
-- ------------------------------

DEFINE TABLE teaches TYPE RELATION IN professor OUT course SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD in ON teaches TYPE record<professor> PERMISSIONS FULL;
DEFINE FIELD out ON teaches TYPE record<course> PERMISSIONS FULL;
DEFINE FIELD role ON teaches TYPE string PERMISSIONS FOR select, create, update WHERE FULL;



-- ------------------------------
-- TABLE DATA: teaches
-- ------------------------------

BEGIN;
COMMIT;
BEGIN;
INSERT RELATION { __: true, id: teaches:5wwtfb65vclbbaj2yrqc, in: professor:12345678, out: course:kp7n2n27oedkvphduv29, role: 'practice' } VERSION d'2025-11-26T09:28:35.038760997Z';
INSERT RELATION { __: true, id: teaches:a8s5aejs05th2tx72s00, in: professor:12345676, out: course:kp7n2n27oedkvphduv29, role: 'theory' } VERSION d'2025-11-26T09:28:35.038760997Z';
INSERT RELATION { __: true, id: teaches:bcaxgpgsih9qsi9oawd1, in: professor:87654321, out: course:kp7n2n27oedkvphduv29, role: 'practice' } VERSION d'2025-11-26T09:28:35.038760997Z';
INSERT RELATION { __: true, id: teaches:d719ubr0d521vjipp1k8, in: professor:87654321, out: course:f04lf2tqlor5jg2qt4ej, role: 'practice' } VERSION d'2025-11-26T09:28:35.038760997Z';
INSERT RELATION { __: true, id: teaches:hk1vc0r2hv6zdhycoy1n, in: professor:12345678, out: course:f04lf2tqlor5jg2qt4ej, role: 'practice' } VERSION d'2025-11-26T09:28:35.038760997Z';
COMMIT;

-- ------------------------------
-- TABLE: week
-- ------------------------------

DEFINE TABLE week TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;




-- ------------------------------
-- TABLE DATA: week
-- ------------------------------



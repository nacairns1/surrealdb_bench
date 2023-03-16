use fake::{Dummy, Fake, Faker};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::{
    engine::local::{Db, File},
    sql::{Id, Thing},
    Surreal,
};

pub static DB: Surreal<Db> = Surreal::init();

pub async fn init_to_file_based() {
    let mut file_path = std::env::current_dir().unwrap();

    file_path.push("surreal_storage");
    DB.connect::<File>("surreal_storage").await.unwrap();
    DB.use_ns("bench").use_db("bench").await.unwrap();

    DB.query("REMOVE TABLE small; REMOVE TABLE medium; REMOVE TABLE large;")
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
pub struct Small {
    #[serde(skip)]
    pub id: Option<String>,
    int: u32,
    string: String,
}

impl Dummy<Faker> for Small {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(f: &Faker, _rng: &mut R) -> Self {
        let id = Some(gen_rand_thing("small").to_string());
        let int: u32 = f.fake();
        let string: String = f.fake();

        Self { id, int, string }
    }
}

fn gen_rand_thing(tb: &str) -> Thing {
    Thing {
        tb: tb.to_string(),
        id: Id::rand(),
    }
}

impl Small {
    fn define_statements() -> Vec<&'static str> {
        [
            "DEFINE TABLE small SCHEMAFULL",
            "DEFINE FIELD int ON TABLE small TYPE int",
            "DEFINE FIELD string ON TABLE small TYPE string",
        ]
        .to_vec()
    }

    pub async fn run_define_statements() {
        let statements = Self::define_statements();

        for statement in statements {
            DB.query(statement).await.unwrap();
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Medium {
    #[serde(skip)]
    pub id: Option<String>,
    nested_smalls: Vec<Small>,
    array: Vec<String>,
    opt_string: Option<String>,
    opt_int: Option<u32>,
    opt_vec: Option<Vec<String>>,
    linked_thing: String,
}
impl Dummy<Faker> for Medium {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(f: &Faker, _rng: &mut R) -> Self {
        let id = Some(gen_rand_thing("medium").to_string());
        let nested_smalls: Vec<Small> = fake::vec![Small;5];
        let array = fake::vec![String; 30];
        let opt_string: Option<String> = Some(f.fake());
        let opt_int: Option<u32> = Some(f.fake());
        let opt_vec: Vec<String> = fake::vec![String; 20];
        let opt_vec = Some(opt_vec);
        let linked_thing = gen_rand_thing("small").to_string();

        Self {
            id,
            nested_smalls,
            array,
            opt_string,
            opt_int,
            opt_vec,
            linked_thing,
        }
    }
}

impl Medium {
    pub fn define_statements() -> Vec<&'static str> {
        [
            "DEFINE TABLE medium SCHEMAFULL",
            "DEFINE FIELD nested_smalls ON TABLE medium TYPE array",
            "DEFINE FIELD array ON TABLE medium TYPE array",
            "DEFINE FIELD opt_string ON TABLE medium TYPE string",
            "DEFINE FIELD opt_int ON TABLE medium TYPE int",
            "DEFINE FIELD opt_vec ON TABLE medium TYPE array",
            "DEFINE FIELD linked_thing ON TABLE medium TYPE record (small)",
        ]
        .to_vec()
    }
    pub async fn run_define_statements() {
        let statements = Self::define_statements();

        for statement in statements {
            DB.query(statement).await.unwrap();
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Large {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    nested_mediums: Vec<Medium>,
    opt_object: Option<BTreeMap<String, String>>,
    nested_array_in_object: BTreeMap<String, Vec<String>>,
    int1: u8,
    int2: u16,
    int3: u32,
    int5: i8,
    int6: i16,
    int7: i32,
    opt_int: Option<u32>,
    string1: String,
    string2: String,
    string3: String,
    string4: String,
    string5: String,
    string6: String,
    string7: String,
    string8: String,
    string9: String,
    string10: String,
    string11: String,
    string12: String,
    string13: String,
    string14: String,
    string15: String,
    string16: String,
    string17: String,
    string18: String,
    string19: String,
    string20: String,
    opt_string: Option<String>,
}

impl Dummy<Faker> for Large {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(f: &Faker, _rng: &mut R) -> Self {
        let id = Some(gen_rand_thing("large").to_string());
        let nested_mediums = fake::vec![Medium; 20];
        let mut object: BTreeMap<String, String> = BTreeMap::new();

        let range: u8 = f.fake();

        for _ in 0..range {
            object.insert(f.fake(), f.fake());
        }
        let opt_object = Some(object);

        let mut nested_array_in_object: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let range: u8 = f.fake();
        for _ in 0..range {
            nested_array_in_object.insert(f.fake(), fake::vec![String;10]);
        }
        let int1: u8 = f.fake();
        let int2: u16 = f.fake();
        let int3: u32 = f.fake();
        let int5: i8 = f.fake();
        let int6: i16 = f.fake();
        let int7: i32 = f.fake();
        let opt_int: Option<u32> = Some(f.fake());
        let string1: String = f.fake();
        let string2: String = f.fake();
        let string3: String = f.fake();
        let string4: String = f.fake();
        let string5: String = f.fake();
        let string6: String = f.fake();
        let string7: String = f.fake();
        let string8: String = f.fake();
        let string9: String = f.fake();
        let string10: String = f.fake();
        let string11: String = f.fake();
        let string12: String = f.fake();
        let string13: String = f.fake();
        let string14: String = f.fake();
        let string15: String = f.fake();
        let string16: String = f.fake();
        let string17: String = f.fake();
        let string18: String = f.fake();
        let string19: String = f.fake();
        let string20: String = f.fake();
        let opt_string = Some(f.fake());
        Large {
            id,
            nested_mediums,
            opt_object,
            nested_array_in_object,
            int1,
            int2,
            int3,
            int5,
            int6,
            int7,
            opt_int,
            string1,
            string2,
            string3,
            string4,
            string5,
            string6,
            string7,
            string8,
            string9,
            string10,
            string11,
            string12,
            string13,
            string14,
            string15,
            string16,
            string17,
            string18,
            string19,
            string20,
            opt_string,
        }
    }
}

impl Large {
    pub fn define_statements() -> Vec<&'static str> {
        [
            "DEFINE TABLE large SCHEMAFULL",
            "DEFINE FIELD nested_mediums ON TABLE large TYPE array",
            "DEFINE FIELD opt_object ON TABLE large TYPE object",
            "DEFINE FIELD nested_array_in_object ON TABLE large TYPE object",
            "DEFINE FIELD int1 ON TABLE large TYPE int",
            "DEFINE FIELD int2 ON TABLE large TYPE int",
            "DEFINE FIELD int3 ON TABLE large TYPE int",
            "DEFINE FIELD int5 ON TABLE large TYPE int",
            "DEFINE FIELD int6 ON TABLE large TYPE int",
            "DEFINE FIELD int7 ON TABLE large TYPE int",
            "DEFINE FIELD opt_int ON TABLE large TYPE int",
            "DEFINE FIELD string1 ON TABLE large TYPE string",
            "DEFINE FIELD string2 ON TABLE large TYPE string",
            "DEFINE FIELD string3 ON TABLE large TYPE string",
            "DEFINE FIELD string4 ON TABLE large TYPE string",
            "DEFINE FIELD string5 ON TABLE large TYPE string",
            "DEFINE FIELD string6 ON TABLE large TYPE string",
            "DEFINE FIELD string7 ON TABLE large TYPE string",
            "DEFINE FIELD string8 ON TABLE large TYPE string",
            "DEFINE FIELD string9 ON TABLE large TYPE string",
            "DEFINE FIELD string10 ON TABLE large TYPE string",
            "DEFINE FIELD string11 ON TABLE large TYPE string",
            "DEFINE FIELD string12 ON TABLE large TYPE string",
            "DEFINE FIELD string13 ON TABLE large TYPE string",
            "DEFINE FIELD string14 ON TABLE large TYPE string",
            "DEFINE FIELD string15 ON TABLE large TYPE string",
            "DEFINE FIELD string16 ON TABLE large TYPE string",
            "DEFINE FIELD string17 ON TABLE large TYPE string",
            "DEFINE FIELD string18 ON TABLE large TYPE string",
            "DEFINE FIELD string19 ON TABLE large TYPE string",
            "DEFINE FIELD string20 ON TABLE large TYPE string",
            "DEFINE FIELD string21 ON TABLE large TYPE string",
            "DEFINE FIELD string22 ON TABLE large TYPE string",
            "DEFINE FIELD opt_string ON TABLE large TYPE string",
        ]
        .to_vec()
    }
    pub async fn run_define_statements() {
        let statements = Self::define_statements();

        for statement in statements {
            DB.query(statement).await.unwrap();
        }
    }
}

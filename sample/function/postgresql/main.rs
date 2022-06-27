/*
    postgres crate를 이용하여 postgresql-rust간의 db처리
    (해당 예졔는 docker에서 실행된 postgresql로 테스트하였습니다.)

    1.참조 사이트
        1)postgres : https://crates.io/crates/postgres
        2)docker 구성 관련 참조 :
        https://shanepark.tistory.com/237
        https://devinlife.com/postgresql/run-postgresql-on-docker/

    2. Cargo.toml파일 버전 설정 :
        [dependencies]
        postgres = "0.19.3"
        anyhow="1.0"

    3. 동작설명
     1)db 주소로 connect
     2)db table 생성
     3)db insert 수행

*/
use anyhow::{anyhow, Result};
use postgres::{Client, NoTls};

pub struct PostgresDb {
    pub client: Client,
}
pub struct Testvalues {
    pub value1: f32,
    pub value2: f32,
}

fn connect_database(url: &str) -> Result<PostgresDb> {
    if let Ok(ret) = Client::connect(url, NoTls) {
        return Ok(PostgresDb { client: ret });
    }
    Err(anyhow!("Fail to connect database"))
}

impl PostgresDb {
    pub fn create_table(&mut self) -> Result<()> {
        if self
            .client
            .batch_execute(
                "
             CREATE TABLE IF NOT EXISTS test_values (
               values1 REAL PRIMARY KEY,
               values2 REAL NOT NULL,
            )
          ",
            )
            .is_ok()
        {
            Ok(())
        } else {
            Err(anyhow!("Fail to Create test Table"))
        }
    }
    pub fn insert_table(&mut self, test_values: &Testvalues) -> Result<()> {
        if self
            .client
            .execute(
                "INSERT INTO test_values VALUES($1,$2)",
                &[&test_values.value1, &test_values.value2],
            )
            .is_ok()
        {
            Ok(())
        } else {
            Err(anyhow!("Fail to Execute Insert Qeury"))
        }
    }
}

fn main() {
    let db_url = "postgresql://postgres:123456@localhost:5432/postgres";
    let mut postgres_db = match connect_database(db_url) {
        Ok(ret) => ret,
        Err(e) => {
            panic!("{e:?}");
        }
    };

    if postgres_db.create_table().is_err() {
        panic!("Fail ti Create Table");
    }

    let vals = Testvalues {
        value1: 10.0,
        value2: 20.0,
    };
    if let Err(e) = postgres_db.insert_table(&vals) {
        panic!("{e:?}");
    }
}

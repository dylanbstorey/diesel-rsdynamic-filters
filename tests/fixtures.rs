use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use uuid::Uuid;

use pedal_pal::{
    dal::DataAccessLayer,
    models::{
        person::{NewPerson, Person},
        bike::{NewBike, Bike},
        color::{NewColor, Color},
        bike_trip::NewBikeTrip,
    },
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct TestFixture {
    pool: Pool,
}

impl TestFixture {
    pub fn new() -> Self {
        let database_url= "postgres://postgres:password@localhost:5432/postgres".to_string();

        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create DB connection pool");

        // Run migrations
        let mut conn = pool
        .get()
        .expect("Failed to get DB connection");

        conn.begin_test_transaction().expect("Failed to begin test transaction");
        conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");

        TestFixture { pool }

    }

    pub fn dal(&self) -> DataAccessLayer {
        DataAccessLayer::new(self.pool.clone())
    }

    pub fn setup_colors(&self) {
        let dal = self.dal();
        let colors = vec![
            "Red", "Blue", "Green", "Yellow", "Purple", "Orange", "Pink", "Brown",
            "Black", "White", "Gray", "Cyan", "Magenta", "Lime", "Teal", "Indigo"
        ];

        for c in colors {
            let new_color = NewColor::new(c);
            dal.color().create(&new_color).expect("Failed to create color");
        }
    }

    pub fn setup_bike_trips(&self) {
        let dal = self.dal();
        let bike = self.create_bike("Mountain Bike",None,None);


        for i in 1..=5 {
            let new_trip = NewBikeTrip {
                id: Uuid::new_v4().to_string(),
                name: format!("Trip {}", i),
                bike_id: Some(bike.id.clone()),
            };
            dal.bike_trip().create(&new_trip).unwrap();
        }
    }

    pub fn setup_bikes(&self) {
        let dal = self.dal();
        let alice = self.create_person("Alice");
        let bob = self.create_person("Bob");
        let red = self.create_color("Red");
        let blue = self.create_color("Blue");

        self.create_bike("Mountain Bike", Some(&alice.id), Some(&red.id));
        self.create_bike("Road Bike", Some(&bob.id), Some(&blue.id));
        self.create_bike("City Bike", Some(&alice.id), Some(&blue.id));
    }

    pub fn setup_people(&self) {
        self.create_person("Alice");
        self.create_person("Bob");
        self.create_person("Charlie");
    }

    pub fn create_bike(&self, name: &str, owner_id: Option<&str>, color_id: Option<&str>) -> Bike {
        let dal = self.dal();
        let new_bike = NewBike {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            owner_id: owner_id.map(|s| s.to_string()),
            color_id: color_id.map(|s| s.to_string()),
        };
        dal.bike().create(&new_bike).unwrap()
    }

    pub fn create_person(&self, name: &str) -> Person {
        let dal = self.dal();
        let new_person = NewPerson::new(name);
        dal.person().create(&new_person).unwrap()
    }

    pub fn create_color(&self, name: &str) -> Color {
        let dal = self.dal();
        let new_color = NewColor::new(name);
        dal.color().create(&new_color).unwrap()
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {}
}

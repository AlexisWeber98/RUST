// uso de mods

/*

mod math {
    type Complex = (f64, f64);
    pub fn sin(f: f64) -> f64 { /* ---- */
    }
    pub fn cos(f: f64) -> f64 { /* ---- */
    }
    pub fn tan(f: f64) -> f64 { /* ---- */
    }
}
// println!("{}", math::cos(45.0));

// uso de codigo público / privado

// declare a private struct

struct Foo;

// declare a public enu, with private field

pub struct Bar {
    field: i32,
}

// declare  a public enum with two public variants

pub enum State {
    PubliclyAccessibleVariant,
    PubliclyAccessibileVariant2,
}

// Además de ayudarle a organizar mejor el código, los módulos también garantizan la privacidad de sus valores, tipos y métodos.

pub struct User {
    username: String,
    password_hash: u64,
}

impl User {
    pub fn new(username: &str, password: &str) -> User {
        User {
            username: username.to_string(),
            password_hash: hash_password(&password.to_owned()),
        }
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn set_password(&mut self, new_password: &str) {
        self.password_hash = hash_password(&new_password.to_owned())
    }
}

fn hash_password<T: Hash>(t: &T) -> u64 {/* ... */}

 */
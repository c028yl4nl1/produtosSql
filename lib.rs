use std::io::SeekFrom;
use colored::*;
use sqlite::Connection;

pub trait Sql {
    fn view_users(&self) -> Vec<namespace_users>;
    fn id_is_valido(&self, id: i32) -> bool;
    fn delete_user(&self, id: i32) -> bool;
    fn add_user(&self,id: i32 , username: String, data: i32 , info_sobre_user: String) -> bool; 
    fn total_de_users(&self) -> i32;
    fn editar(&self, username: String, data: i32, info_sobre_user: String , id: i32) -> bool;
    fn mostra_user(&self , id: i32);

    

}
#[derive(Debug)]
pub struct namespace_users {
    pub id: String,
    pub username: String,
}

impl Sql for Connection {
    fn view_users(&self) -> Vec<namespace_users> {
        let mut vector: Vec<namespace_users> = Vec::new();
        {
            self.iterate("SELECT * FROM users", |pairs| {
                let mut username: String = String::new();
                let mut id: String = String::new();
                let mut contador = 0;
                for &(name, value) in pairs.iter() {
                    if name == "id" || name == "username" {
                        match value {
                            Some(id_or_name) => match contador {
                                0 => {
                                    id.push_str(id_or_name);
                                    contador += 1;
                                }

                                1 => {
                                    username.push_str(id_or_name);

                                    vector.push(namespace_users {
                                        id: id.to_owned(),
                                        username: username.to_owned(),
                                    });

                                    username.clear();
                                    id.clear();
                                    contador = 0;
                                }

                                _ => {}
                            },
                            None => {
                                eprintln!("Erro ao lista o user");
                            }
                        }
                    }
                }
                true
            })
            .unwrap();
        }

        vector
    }

    fn id_is_valido(&self, id: i32) -> bool {
        let mut bool: bool = false;
        let yc = format!("SELECT id FROM users WHERE id ={}", id);
        self.iterate(yc, |pairs| {
            if pairs.len() == 0 {
                bool = false;
            } else {
                bool = true
            }
            true
        })
        .unwrap();
        bool
    }

    fn delete_user(&self, id: i32) -> bool {
        let a = self.execute(format!("DELETE FROM users WHERE id='{}'", id));

        if let Ok(_) = a {
            return true;
        } else {
            return false;
        }
    }
    fn add_user(&self,id: i32 , username: String ,data: i32 , info_sobre_user: String) -> bool {
        use chrono::*;

        let data =  Local::now().date().naive_local().checked_add_days(Days::new(data as u64)).unwrap().to_string();
        
        let query = format!("INSERT INTO users VALUES ({},'{}', '{}', '{}')", id,username, data,info_sobre_user);

        if self.execute(query).is_err(){
            return false;
        }

        true
    }

    fn total_de_users(&self) -> i32 {
        let query = "SELECT COUNT(id) FROM users";
        let mut valor = 0;
        let execute =  self.iterate(query, |pairs| {

            for &(c,o) in pairs{
                valor = o.unwrap().parse::<i32>().unwrap();
            }
            true
        });
       
       
    
        valor
    }
    fn editar(&self, username: String, datas: i32, info_sobre_user: String , id: i32) -> bool {
        let mut valor = String::new();
        let execute =  self.iterate(format!("SELECT data FROM users WHERE id ={}", id), |pairs| {

            for &(c,o) in pairs{
                valor = o.unwrap().to_string();
            }
            true
        });

        use chrono::*;
    
        let date = NaiveDate::parse_and_remainder(valor.as_str(), "%Y-%m-%d").unwrap().0;

        let data = date.checked_add_days(Days::new(datas as u64)).unwrap().to_string();




        let query = format!("UPDATE users SET username='{}' , data='{}', info='{}' WHERE id = {}", username, data, info_sobre_user , id);
        let execute = self.execute(query);

        if execute.is_err(){
            return  false;

        }
        true
    }

    fn mostra_user(&self, id: i32) {
        self.iterate(format!("SELECT * FROM users WHERE id = {} ", id), |pairs| {

            for &(c,v) in pairs{
                println!("{} | {} ", c , v.unwrap());
            }
            true
        });

    }
}

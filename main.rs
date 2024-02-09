// Ferramenta para gereciar usuarios
const nome_db: &str = "jiraya.db";
use jiraya::Sql;
use sqlite::*;
use std::{f32::consts::SQRT_2, ffi::NulError, io::Write, num::ParseIntError, os::{fd::AsFd, unix::process}};
use colored::*;
use rand::*; 
#[derive(Debug)]
struct AddInfoUSer {
    username: String,
    data_de_saida: i32,
    infSobre_user: String,
}

impl AddInfoUSer {
    fn new(username: String, data_de_saida: i32, info_sobre_user: String) -> AddInfoUSer {
        AddInfoUSer {
            username: username,
            data_de_saida: data_de_saida,
            infSobre_user: info_sobre_user,
        }
    }
}

fn main() {
        Sqlite_functions(); 
        help_or_menu();
}

/**
 * Sqlite learning
 */

fn Sqlite_functions(){
    

    let connection = open(nome_db);

    match connection {
        Ok(conect) => {
            let mut query = r#"
            
            
            CREATE TABLE IF NOT EXISTS users ( id INT primary key, username TEXT , data TEXT , info TEXT);
            
            "#;

            if let Ok(v) = conect.execute(query) {
                //println!("{}", "Tabela criada com sucesso".bright_green());
            }
        }
        Err(error) => {
            println!("{}", "Erro ao cria a tabela".bright_red());

            std::process::exit(1);
            dbg!(error);
        }
    }
}

fn help_or_menu() {
    loop {
        limpar_tela();
        println!("
        {} -> {}
        {} -> {}
        {} -> {}
        {} -> {}

        ", "1".bright_white() , "Todos Usuarios".bright_green(), "2".bright_purple(), "Quantos Usuarios Tem".bright_yellow(), "3".bright_magenta(), "Adcionar novo Usuario".bright_green(), "4".bright_cyan(), "Encerra o programa".bright_red());

        {
            let option = input("Digite Sua Opção: ");

            match string_to_number(option) {
                Some(Valor) => match Valor {
                    1 => loop {
                        limpar_tela();

                        for userLists in  conect_op().view_users(){
                            print!("Username: {} | id: {}", userLists.username , userLists.id);
                            
                            println!("");
                        }
                        anim("
                        1 - editar user 
                        2 - deletar user 
                        3 - Ver user
                        4 - back or volta 
                        "); 
                        let ux = input("\rDigite a opção:");

                        if let Some(number) = string_to_number(ux) {
                            match number {
                                1 => loop {
                                    limpar_tela();
                                    println!("------- Editar ----- 0 para voltar ");
                                    let uv = input("Digite o id: ");

                                    if let Some(valor) = string_to_number(uv) {
                                        if valor == 0 {
                                            break;
                                        }

                                        if conect_op().id_is_valido(valor) {
                                            // Valido

                                           // username = 
                                           // data_user = 
                                           // info sobre o user
                                            loop {
                                                limpar_tela();

                                                println!("Editando ..");
                                                let user = input("Digite o username: ");    
                                                let data = input("Digite a nova data: ");


                                                if let Some(data) = string_to_number(data){
                                                    let info = input("Digite as info sobre o user: ");

                                                    // EDITAR      
                                                    if conect_op().editar(user, data, info, valor){
                                                        println!("Editei com sucesso");
                                                        std::thread::sleep(std::time::Duration::from_secs(1));
                                                        break;
                                                    }
                                                    else  {
                                                        println!("Ouver um error não conseguir editar");
                                                        std::thread::sleep(std::time::Duration::from_secs(1));
                                                    }

                                                }
                                                else {
                                                    println!("Na data digite apenas number");
                                                    std::thread::sleep(std::time::Duration::from_secs(1));
                                                }
                                                
                                               
                                                
                                            }
                                         
                                        } else {
                                            // Não é valido

                                            println!("Digite o id valido: ");

                                            std::thread::sleep(std::time::Duration::from_secs(1));
                                        }
                                    } else {
                                        println!("Digite apenas numero ok pois o id é numero");
                                        std::thread::sleep(std::time::Duration::from_secs(1));
                                    }
                                },

                                2 => {
                                    // Deletar

                                    loop {
                                        limpar_tela();

                                        let yv = input("0 -back  id: ");

                                        if let Some(valor) = string_to_number(yv) {
                                            match valor {
                                                0 => {
                                                    break;
                                                }
                                                _ => {
                                                    if conect_op().id_is_valido(valor) {
                                                        if conect_op().delete_user(valor) {
                                                            println!("ID deletado com sucesso");
                                                            std::thread::sleep(
                                                                std::time::Duration::from_secs(1),
                                                            );
                                                        } else {
                                                            println!("Houver um problema ao deletar o user ");
                                                            std::thread::sleep(
                                                                std::time::Duration::from_secs(1),
                                                            );
                                                        }
                                                    } else {
                                                        println!("Não é valido");
                                                        std::thread::sleep(
                                                            std::time::Duration::from_secs(1),
                                                        );
                                                    }
                                                }
                                            }
                                        } else {
                                            println!("Digite apenas numero");
                                            std::thread::sleep(std::time::Duration::from_secs(1));
                                        }
                                    }
                                },
                                3 => {
                                    loop {
                                        limpar_tela();
                                        let ui = input("0 - back Digite o id do usuario: ");
                                        if let Some(valor) = string_to_number(ui){
                                            if valor == 0{
                                                break;
                                            }
                                            if conect_op().id_is_valido(valor){
                                                conect_op().mostra_user(valor); 

                                                input("enter back"); 
                                                break;
                                            }
                                            else {
                                                println!("Digite o id correto");
                                                std::thread::sleep(std::time::Duration::from_secs(1));
                                            }

                                        }
                                        else {
                                            println!("Digite Apenas o id não string");
                                            std::thread::sleep(std::time::Duration::from_secs(1));
                                        }
                                     }
                                }
                                4 => {
                                    break;
                                }
                                _ => {}
                            }
                        } else {
                            println!("Apenas numero please");
                            std::thread::sleep(std::time::Duration::from_secs(1));
                        }
                    },
                    2 => {
                        println!("Total De usuarios cadastrados: {} ", conect_op().total_de_users());
                        std::thread::sleep(std::time::Duration::from_secs(1));
                        
                    }, 
                    3 => {
                        'add_user: loop {
                             limpar_tela();

                             // username dataDeSaida info a mais 

                             let username = input("DIgite o username: ");
                             if username.len() < 1 {
                                println!("O username não pode ser menor que 1");
                                std::thread::sleep(std::time::Duration::from_secs(1));
                                continue;
                             }
                             // Numero de dias que o cliente vai ficar 
                              loop {
                                let data = input("Digite o numero de dias: ");

                                if let Some(valor) = string_to_number(data) {
                                    let input_string = input("Digite mais informaçoes sobre o cliente: ");
                                    use rand::*;


                                 



                                    let gen = thread_rng().gen_range(0..100000);
                                    if conect_op().add_user(gen,username.clone() , valor, input_string){
                                        println!("Usuario adcionado com sucesso no banco de dados");
                                        
                                        std::thread::sleep(std::time::Duration::from_secs(1));
                                        break 'add_user;
                                    }
                                    else  {
                                        println!("Erro ao adcionar o user no banco de dados");
                                        std::thread::sleep(std::time::Duration::from_secs(1));
                                        break;
                                    }

                                }
                                else {
                                    println!("Digite apenas number");
                                    std::thread::sleep(std::time::Duration::from_secs(1));
                                }
                             }
                             
                             


                        }
                    }
                    4 => {
                        println!("Bye");
                        std::process::exit(0);
                    }
                    _ => {
                        println!("Escolha a opção certa");
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                },

                None => {
                    println!("Number please");
                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            }
        }
    }
}

fn input(pergunta: &str) -> String {
    let mut user_question: String = String::new();
    //print!("{}", );
    anim(pergunta);
    std::io::stdout().flush();
    std::io::stdin().read_line(&mut user_question);

    if user_question.len() > 4{
        return  user_question.trim().to_string();
    }
    
    user_question.trim().replace(" ", "").to_string()
}

fn limpar_tela() {
    #[cfg(target_os = "windows")]
    std::process::Command::new("cls").status();
    #[cfg(target_os = "linux")]
    std::process::Command::new("clear").status();
}

fn string_to_number(strings: String) -> Option<i32> {
    let a = strings.parse::<i32>();

    if let Ok(number) = a {
        Some(number)
    } else {
        None
    }
}

fn conect_op() -> Connection {
    let connect = Connection::open(nome_db);
    match connect {
        Ok(conect) => {
            return conect;
        }
        Err(_) => {
        
            println!("Ouver algum Erro provavelmente não foi meu ");
            std::process::exit(1);
        }
    }
}


fn anim<T>(ascii: T)
    
    where 
    T: std::string::ToString,
    {
    for a in ascii.to_string().chars() {
        
        let number = rand::thread_rng().gen_range(0..13);
        match number {
            1 => {
                print!("{}", a.to_string());
            }
            2 => {
                print!("{}", a.to_string().bold().green());
            }
            3 => {
                print!("{}", a.to_string().bold().black());
            }
            4 => {
                print!("{}", a.to_string().bright_black());
            }
            5 => {
                print!("{}", a.to_string().bright_blue());
            }
            6 => {
                print!("{}", a.to_string().bright_cyan());
            }
            7 => {
                print!("{}", a.to_string().bright_green());
            }
            8 => {
                print!("{}", a.to_string().bright_magenta());
            }
            9 => {
                print!("{}", a.to_string().bright_purple());
            }
            10 => {
                print!("{}", a.to_string().bright_red());
            }
            11 => {
                print!("{}", a.to_string().bright_yellow());
            }
            _ => {
                print!("{}", a.to_string().bright_white());
            }
        }
    }
}
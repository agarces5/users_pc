use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    process::{Child, Command, Stdio},
    time::Duration,
};

use regex::RegexSet;
use wait_timeout::ChildExt;

pub struct Users;

impl Users {
    /// Mostramos por pantalla los usuarios validos
    pub fn print_users(ip: String) {
        let mut buf = String::new();
        let mut child = Self::get_all_users(ip);
        child.stdout.take().unwrap().read_to_string(&mut buf).ok();
        let result = Self::get_valid_users(buf);
        println!("{}", result.join("\t\n"))
    }
    /// Dado una red y un rango, escribe todos los usuarios relevantes de los PC activos
    pub fn users_to_file(red: String, rango: String) -> Result<(), std::io::Error> {
        let rango: Vec<u8> = rango.split('-').map(|i| i.parse().unwrap()).collect();
        let mut file_txt = File::create("usuarios.txt")?;
        let mut file_csv = File::create("usuarios.csv")?;

        println!("Comenzando la revisión de USUARIOS!\n");
        for i in rango[0]..=rango[1] {
            let ip = format!("{}.{}", red, i); // Construimos la ip
            let mut child = Self::get_all_users(ip.clone());

            // Si tarda más de un segundo, paramos el comando.
            if child.wait_timeout(Duration::from_secs(1))?.is_none() {
                // child hasn't exited yet
                child.kill()?;
                child.wait()?;
            };

            let mut users = String::new();

            child.stdout.take().unwrap().read_to_string(&mut users).ok();

            // Construimos un vector con los usuarios que nos importan.
            let users = Self::get_valid_users(users);

            // Escribimos los usuarios en los archivos
            if !users.is_empty() {
                let result = format!("Usuarios en {}:\n\t{}\n\n", &ip, users.join("\n\t"));
                let result_csv = format!("{};{}\n", &ip, users.join(";"));
                file_txt.write_all(result.as_bytes())?;
                file_csv.write_all(result_csv.as_bytes())?;
            }
        }

        println!("Se ha terminado de revisar los usuarios!");
        println!("Resultados en usuarios.txt y usuarios.csv\n");
        Ok(())
    }
    fn get_all_users(ip: String) -> Child {
        let mut path = PathBuf::new();
        path.push(r"\\");
        path.push(ip);
        path.push(r"c$\users");

        Command::new("cmd")
            .arg("/C")
            .arg("dir")
            .arg("/b")
            .arg(path)
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute process")
    }
    fn get_valid_users(users: String) -> Vec<String> {
        let rex = RegexSet::new([
            r"administrador.hplaya",
            r"administrador",
            r"apppool",
            r"defaultuser0",
            r"^dell$",
            r"v2",
            r"pc-1",
            r"^pc$",
            r"public$",
            r"user",
            r"usuario",
        ])
        .unwrap();

        users
            .lines()
            .filter_map(|line| {
                // Con filter_map, nos quedamos solo con el valor de Some(x)
                if !rex.is_match(line.to_lowercase().as_str()) {
                    return Some(line.to_owned());
                }
                None
            })
            .collect::<Vec<String>>()
    }
}

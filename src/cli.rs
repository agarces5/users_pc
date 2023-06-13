use clap::Parser;

use crate::users::Users;

/// Programa para ver los usuarios de los equipos
#[derive(Parser, Debug)]
#[command(name = "Users PC")]
#[command(author = "Antonio Garcés")]
#[command(
    help_template = "{about-section} \n {usage-heading} {usage} \n {all-args} {tab} \n\n Autor: {author-with-newline}"
)]
#[command(about, long_about)]
pub struct Args {
    /// Hotel a escanear ("central" para la Central, 101, 102, etc.)
    /// Los genera en archivos
    #[arg(short, long)]
    centro: Option<String>,
    /// Rango a escanear
    #[arg(short, long)]
    rango: Option<String>,
    /// Equipo a inspeccionar (Nombre de equipo o IP)
    name: Option<String>,
}
impl Args {
    pub fn execute(&self) -> Result<(), std::io::Error> {
        if let Some(name) = self.name.clone() {
            Users::print_users(name)
        } else if let Some(centro) = self.centro.clone() {
            let rango = self.rango();
            match centro.as_str() {
                "central" => Users::users_to_file("172.17.3".to_string(), rango)?,
                hotel => Users::users_to_file(format!("192.168.{}", hotel), rango)?,
            }
        }
        Ok(())
    }
    fn rango(&self) -> String {
        match self.rango.clone() {
            Some(rango) => rango,
            None => String::from("1-255"),
        }
    }
}

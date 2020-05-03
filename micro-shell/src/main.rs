use std::io::{self, Write};
use std::process::{Command};
use std::vec::Vec;


#[derive(Debug, Clone)]
/// création de la structure avec les valeurs red, green et blue
pub struct Cmd{
    name : String,
    args : Vec<String>
}

impl Cmd{
    /// constructeur pour notre Cmd
    pub fn new(name : String, args: Vec<String>) -> Self{
        Cmd{
            name: name,
            args: args
        }
    }

    /// getters
    pub fn name(self) -> String{
        self.name
    }

    pub fn args(self) -> Vec<String>{
        self.args
    }
}

fn main() -> std::io::Result<()>{
    let stdout = io::stdout();

    let stdin = io::stdin();

    // Pour voir hello world avec strace
    println!("Hello World");

    loop{

        // ECRITURE sur la STDOUT
        {
            let mut handle = stdout.lock(); // STDOUT locked
            handle.write_all(b"> ")?; // locked
            handle.flush()? // locked
        } // STDOUT unlocked
        
        
        let mut user_input = String::with_capacity(256);
        // On prends un référence mutable
        stdin.read_line(&mut user_input)?;
        let user_input = user_input.trim_end();

        // Récupération des éléments entrés dans un vecteur
        let mut i = 0;
        let mut vec = Vec::new();

        for token in user_input.split_whitespace(){
            vec.push(token);
            println!("vec {} {}",i,vec[i]);
            i+=1;
         }

        //-----------------------//
        // Executer une commande //
        //-----------------------//
        let output = Command::new(vec[0]).args(&vec[1..]).output()?;
        //afficher statut commande
        println!("status : {}", output.status);
        let stdout = io::stdout().write_all(&output.stdout);
        
        //println!("stdout : {:?}", stdout);
        println!("stderr : {:?}", output.stderr);

        //? sert à "propager l'erreur"
        // c'est mieux que crash avec un unwrap ou expect ;)
    }
        
    Ok(())
}
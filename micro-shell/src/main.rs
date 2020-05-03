use std::io::{self, Write};
use std::process::{Command};


fn main() -> std::io::Result<()>{
    let stdout = io::stdout();

    let stdin = io::stdin();

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

        //-----------------------//
        // Executer une commande //
        //-----------------------//
        let status = Command::new(user_input).status()?;
        //afficher statut commande
        println!("{}", status)

        //? sert à "propager l'erreur"
        // c'est mieux que crash avec un unwrap ou expect ;)
    }
        
    Ok(())
}
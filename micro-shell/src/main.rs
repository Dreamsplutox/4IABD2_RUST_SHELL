use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::vec::Vec;
use std::error::Error;
use std::io::prelude::*;



#[derive(Debug, Clone)]
/// création de la structure pour stocker nos commandes
pub struct Cmd{
    name : usize,
    begin_args : usize,
    end_args : usize

}

impl Cmd{
    /// constructeur pour notre Cmd
    pub fn new(name : usize, begin_args: usize, end_args: usize) -> Self{
        Cmd{
            name: name,
            begin_args: begin_args,
            end_args: end_args
        }
    }

    /// getters
    pub fn name(self) -> usize{
        self.name
    }

    pub fn begin_args(self) -> usize{
        self.begin_args
    }

    pub fn end_args(self) -> usize{
        self.end_args
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
        
        
        /// Récupération de l'entrée utilisateur
        /// 
        let mut user_input = String::with_capacity(256);
        // On prends un référence mutable
        stdin.read_line(&mut user_input)?;
        let user_input = user_input.trim_end();
        

        // Récupération des éléments entrés dans un vecteur
        let mut i = 0;
        let mut words = Vec::new();
        let mut cmds = Vec::<Cmd>::new();
        let mut is_name = true;
        let mut name = 0;
        let mut begin_args = 0;
        let mut end_args = 0;

        /// Remplissage de notre structure
        /// 
        for token in user_input.split_whitespace(){
            words.push(token);
            //println!("vec {} {}",i,token);
            if token == "|"{
                //println!("pipe");
                cmds.push(Cmd::new(name ,begin_args, end_args+1));
                is_name = true;
                //println!("args ==> {}, {}", begin_args, end_args);
                begin_args = 0;
                end_args = 0;
            }else{
                if is_name{
                    //println!("is name");
                    name = i;
                    is_name = false;
                }else{
                    //println!("args");
                    if begin_args == 0{
                        begin_args = i;
                    }
                    end_args = i;
                }
            }
            i+=1;
        }
        
        if begin_args == 0 {
            cmds.push(Cmd::new(name ,begin_args, end_args));    
        }else{
            cmds.push(Cmd::new(name ,begin_args, end_args+1));  
        }
        
        
        //println!("words : {:?}", words);

        /// Après avoir créé les différentes structures, on veut les exécuter

        let mut begin = true;

        let command = Command::new(words[cmds[0].name])
                .args(&words[cmds[0].begin_args..cmds[0].end_args])
                .stdout(Stdio::piped())
                .spawn()
                .expect("I was pancaked while trying to launch ls.");

        
        /// STDOUT de notre commande n°1 ==> itération à travers les commandes suivantes        

        for cmd in &cmds[1..]{
            // Check des variables
            //println!("cmd : {}  {:?}", words[cmd.name], &words[cmd.begin_args..cmd.end_args]);
            //println!("cmd : {}, {}, {}", cmd.name, cmd.begin_args, cmd.end_args);
             
            // Stdio::piped() -> type pour representer une entrée/sortie standard qui sera un tube.
            let cmd_stdout = Stdio::from(command.stdout.expect("Something wrong with ls stdin"));

            /// ERROR : ;-( , nous voulons récupérer la stdout de notre première commande pour l'utiliser
            ///  en tant que stdin des suivantes mais nous avons une erreur (moved value ==> borrow)
            
            let command = Command::new(words[cmd.name])
            .args(&words[cmd.begin_args..cmd.end_args])
            .stdin(cmd_stdout)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Whopsie! wc failled to launch");   
            
        }
      
        //-----------------------//
        // Résultat de la commande //
        //-----------------------//
        
        let mut s = String::new();
        match command.stdout.unwrap().read_to_string(&mut s) {
            Err(why) => println!("couldn't read wc stdout: {}",
                            why.description()),
            Ok(_) => print!("wc responded with:\n{}", s),
        }
        

        //? sert à "propager l'erreur"
        // c'est mieux que crash avec un unwrap ou expect ;)
    }
        
    Ok(())
}
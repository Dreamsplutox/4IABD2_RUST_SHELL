# 4IABD2_RUST_SHELL

# QUESTIONS DU TP

## 1 Révision de Rust et Processus

Nathanaël NEZONDET-RENAUD, Arnaud SIMON
# 1.2 Révision de Rust et Processus
1) En Rust, les références servent à accéder à des ressources via leur adresse dans la mémoire, un peu comme les pointeurs en C
2) Pour former ses propres types en Rust, il faut utiliser des structures ou des enums.
3) Rust est compilé nativement, il n'y a pas de JVM
4) Pour trouver la valeur maximale adressable, on doit calculer l'espace d'adressage avec la formule 2ⁿ - 1, ce qui nous donne ici pour 8 bits =>  255, en hexa FF. l'adresse maximale est donc 0x0000_0000_0000_00FF
5) Un processus est un thread un peu particulier, responsable d'un espace d'adressage. Il peut contenir plusieurs threads / processus (c'est une sorte de conteneur). (source : ce qu'on a compris du cours)

### 2 Pratique - micro-shell
# 2.1 Questions : Deployement du projet et entrées sorties
1) Créer un projet binaire avec cargo
* Pour compiler et exécuter son programme, il faut utiliser la commande "cargo build" puis la commande "cargo run"  
* Pour exécuter les tests, on doit utiliser cargo test
* En mode debug, les binaires sont rangés dans le dossier target/debug, si on utilise l'option --release dans un cargo build, les binaires seront rangés dans le dossier target/release


### 3 Execution d’un Processus
# 3.1 Questions : Executer une commande
4) Rust nous force à récupérer le status pour que l'on puisse vérifier que le programme ne plante pas / que tout s'est bien passé
5) Par défaut, le programme vit sa vie. Mais ici, "Status" force notre programme à attendre la fin de l'exécution de son enfant pour reprendre le travail

### 4 Redirections - pipe my programs
# 4.1 Questions : Redirections
7) C'est un outil qui permet de rediriger la sortie STDOUT d'un processus vers la STDIN d'un autre processus, ils peuvent ainsi communiquer ! (source : ce qu'on a compris du cours)

### 5 Executions en concurence : Gérer des commandes en fond
# 5.1 Questions
10) Un processus ID ou PID est un identifiant (code unique) associé à un processus. Il est très utile pour effectuer des commandes s'appliquant à un processus en particulier, comme la commande kill par exemple.
(source wikipédia / cours de deuxième année sur les processus)
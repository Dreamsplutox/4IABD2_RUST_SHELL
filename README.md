# 4IABD2_RUST_SHELL
Nathanaël NEZONDET-RENAUD, Arnaud SIMON
# 1 Révision de Rust et Processus
1) En Rust, les références servent à accéder à des ressources via leur adresse dans la mémoire, un peu comme les pointeurs en C
2) Rust est compilé nativement, il n'y a pas de JVM
3) Pour trouver la valeur maximale adressable, on doit calculer l'espace d'adressage avec la formule 2ⁿ - 1, ce qui nous donne ici pour 8 bits =>  255, en hexa FF. l'adresse maximale est donc 0x0000_0000_0000_00FF
## 1.1 Pratique - micro-shell
### 1 Créer un projet binaire avec cargo
* Pour compiler et exécuter son programme, il faut utiliser la commande "cargo build" puis la commande "cargo run"  
* Pour exécuter les tests, on doit utiliser cargo test
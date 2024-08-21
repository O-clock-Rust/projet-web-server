# Projet 2 : un serveur web en Rust

Tu vas créer une API REST en Rust, ce qui te permettra de mettre un pied dans le web à la sauce Rust. Rust est particulièrement bon en web, les performances sont proches de C, avec les bénéfices qu'apporte Rust.

Pas besoin de créer un serveur _from scratch_, on va utiliser un des frameworks les plus populaires : Actix ! Je te propose donc de plonger dans la [doc d'Actix](https://actix.rs/) si tu es curieux. Dans tous les cas, tu vas devoir finir par y aller 😁

## Instructions

**Développer une API REST pour gérer une liste de tâches (To-Do List)**

L'objectif est de créer une API REST simple qui permet de gérer une liste de tâches avec les fonctionnalités suivantes :

⚠️ **Pas de base de données pour l'instant !** On renvoie donc des données fictives

- Créer une nouvelle tâche
- Récupérer la liste des tâches
- Récupérer une tâche spécifique par son ID
- Mettre à jour une tâche existante
- Supprimer une tâche

**Bonus**

- Utiliser une base de données (SQLite par exemple) pour au moins une des routes
- Pour cela, utiliser la librairie `sqlx` : https://github.com/launchbadge/sqlx

➡️ Pour comprendre comment fonctionne `sqlx`, lis bien le readme et prends connaissance des exemples !

### Spécifications de l'API

|   | Méthode HTTP  | Endpoint | Body  | Réponse  |
|---|---|---|---|---|
| Créer une tâche  | POST  | /tasks  | JSON avec les données de la tâche | JSON de la tâche créée  |
| Récupérer toutes les tâches  | GET  | /tasks  | -  | JSON contenant une liste de toutes les tâches  |
| Récupérer une tâche par ID  | GET  | /tasks/:id  | -  | JSON contenant les détails de la tâche correspondante  |
| Mettre à jour une tâche  | PUT  | /tasks/:id  | JSON contenant les nouveaux détails de la tâche  | JSON de la tâche mise à jour  |
| Supprimer une tâche  | DELETE  | /tasks/:id  | -  | Statut de succès  |

### Modèle de données

- id
- title
- description
- status

## Help !

Ce projet va probablement être un peu déstabilisant car tu dois découvrir une librairie que tu ne connais pas. Ceci dit, la [documentation d'Actix](https://actix.rs/) comporte tout ce qu'il faut pour compléter ce projet ! En fouillant, tu devrais t'en sortir.

Il y a aussi [plein d'exemples ici](https://github.com/actix/examples).
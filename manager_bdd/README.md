# manager BDD

Effectue differentes actions sur la base de donnée:  
- Remove les sessions périmées (utilisateur et utilisateur vm)
- Remove les mot de passe inchangés depuis trop longtemps
- Ajout des sessions temporaires ainsi que l'envoi d'email
- Lors du lancement, push des utilisateurs par default dans la base de donnée

## Variable d'environnement

DATABASE_URL: l'url complète de la database (avec mdp, user et base name)  
EMAIL_PASS: le mot de passe du compte mail  
EMAIL: l'email   
ROOT_CERT: le certificat root  
SMTP_SERV: le serveur smtp de l'email  
SERVER_URL: L'url du serveur  
USERS: Les utilisateurs par default [{'first_name':'','last_name':'','email':'','level':1,'password':'9caractere'},...]  
TIME_SESSION: la durée d'une session utilisateur (en secondes)  
TIME_TEMPORAIRE: la duree d'une session temporaire (en secondes)  
TIME_VEMMION: la duree maximum d'une connexion d'une session vnc (en secondes)  
TIME_VEMMION_MAX: la duree d'une session vnc (en secondes)  
CHANGE_PASSWORD: la durée maximal avant la supression automatique du mot de passe (en secondes)  
ACTUALISATION: le temps entre chaque actualisation de la base de donnée (en secondes)  
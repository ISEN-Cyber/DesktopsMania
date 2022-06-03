# The redirect vnc

Le programme permet de rediriger un flux vnc vers la bonne machine, selon les informations contenues dans la base de donnée.  
Le client envoie la premiere requête avec un path de session (valable 1 seule et unique fois).  
Ne pas oublier de set la variable d'environnement DATABASE_URL, afin d'indiquer l'url, l'utilisateur, le mot de password et le nom de la base de donnée au programme
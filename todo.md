axum:
 - clean up
   - add logging for errors
   - add static routing for images/css/etc

jwt:
 - when there is an error on the token, redirect to /login
 - read username from database
    - split from password to make it easier
    - abstract this so we can connect to postgres, aws, etc.
 - authenticate against password
    - start with just a basic auth
    - using argon2
 - read password from database
 - add group
    - use binary system
    - i.e. user = 1, parent = 2, user and parent = 3, support = 4, admin = 256
 - add refreshing to tokens
    - why don't we try refreshing if its within 8 days of expiry
    - we keep the refresh token in the database if we need to revoke the token
    - but the normal token doesnt ever check the db for requests
    - can change the above to an hour?
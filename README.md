# ZSChallenge
Zusammenstromen coding challenge

>Please build a docker-compose setup that includes a DB and an application image that can handle the following API endpoints in JSON format:
> - A createAccount (The account should consist of details like email, car details and bank details)
> - login
> - getDetails
>
>We would like to see how you solve this challenge using Rust, but you can also use any other typed language.

Challenge was solved using Rust as requested.

## Run
 - copy `api/.env.docker` to `api/.env`
 - run `docker-compose up`
After startup the API should be reachable under `localhost:8080`.

Improments for the future:
 - Use hashing for passwords instead of plaintext
 - implement proper authentication

# ![RealWorld Example App](logo.png)

> ### Rust/Tide codebase containing real world examples (CRUD, auth, advanced patterns, etc) that adheres to the [RealWorld](https://github.com/gothinkster/realworld) spec and API.


### [Demo](https://github.com/gothinkster/realworld)&nbsp;&nbsp;&nbsp;&nbsp;[RealWorld](https://github.com/gothinkster/realworld)

### WIP - this repo is as yet incomplete and still being implemented

The app will evolve as I experiment with nice ways to structure things. It's very minimal so far, but I intend to grow it to be a good reference for implementing an app in Tide.

For now, the collection of Postman tests serve as "acceptance tests" (see [https://github.com/gothinkster/realworld/tree/master/api]). The testing story for tide is still evolving, but rust tests will be included when they can be.

This codebase was created to demonstrate a fully fledged fullstack application built with **Rust/Tide** including CRUD operations, authentication, routing, pagination, and more.

We've gone to great lengths to adhere to the **Rust/Tide** community styleguides & best practices.

For more information on how to this works with other frontends/backends, head over to the [RealWorld](https://github.com/gothinkster/realworld) repo.


# How it works

> Describe the general architecture of your app here

# Getting started

Ensure postgres is installed and running.
Ensure user 'realworld-tide' exists and can create databases.
```
sudo -u postgres psql -c "CREATE USER \"realworld-tide\" WITH ENCRYPTED PASSWORD 'password';"
sudo -u postgres psql -c "ALTER USER \"realworld-tide\" CREATEDB;"
```
Ensure diesel cli is installed, see [http://diesel.rs/guides/getting-started/]

## Run tests
Run tests, including DB integration tests
```
cargo make test
```

## Run app and realworld test suite
Setup database using diesel cli
```
diesel database setup
```
Run the app
```
cargo run
```
Run the "realworld" Postman tests
```
git clone https://github.com/gothinkster/realworld
cd realworld/api
APIURL=http://localhost:8181/api ./run-api-tests.sh
```

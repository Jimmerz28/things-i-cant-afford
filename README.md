# Things I Can't Afford

I like window shopping online, especially for future purchases or for inspiration (Pinterest, Tumblr, etc.)

However, I like to have prices attached so I can have realistic goals for my future purchases which I currently use Coda.io for.

Personally I can't stand Pinterest, Tumblr only suits half the role I wanted and Coda.io is closed source.

Plus this is something good to try out new shit in my spare time.

## Technical Motivations

### Frontend

I wanted to stick with Facebooks's ecosystem i.e. `React`, `Flux`/`Redux`, `Flow`, `GraphQL` since I haven't yet worked too extensivly with `GraphQL`.

### Backend

The goal here was to use a compiled langugage which meets my personal syntax taste and doesn't look like whomever developed the syntax threw up on the keyboard.

Also wanted to see the current state and ease of GraphQL integration.

Initially, I wanted to see if Server-Side Swift already had GraphQL implementations readily available either via [Vapor](https://vapor.codes/) or [Kitura](https://www.kitura.io/), but I wasn't impressed with the precursory research.

Although, I am happy that IBM seems to be pushing or supporting Server-Side Swift.

Had my eye on `Rust` for awhile and found that there were several GraphQL frameworks in the wild and integration didn't seem overly difficult.

So for now I'm going with `Rust` to firstly see how the integration with `GraphQL` goes and then `MongoDB`.

## Getting Started

@TODO: Flesh this out once the setup is complete

### Prerequisites

* Node.js
* yarn
* docker
* docker-compose

### Installing

Right now there's a React based website and a Rust based GraphQL server (and eventually a Mongo database).

So first install our Node.js dependencies

```
yarn install
```

And start the server with

```
docker-compose -f docker/dev/docker-compose.yaml up --build
```

@TODO: Add server startup as a script to `package.json`

## Running the tests

@TODO: Write tests

## Deployment

@TODO: Figure out a good deployment process

## Built With

* [Rust](https://www.rust-lang.org/) - Server side language
  * [Rocket](https://rocket.rs/) - Web framework for Rust
  * [Juniper](https://github.com/graphql-rust/juniper) - GraphQL server lib for Rust
* [Create React App](https://github.com/facebook/create-react-app) - Frontend project setup tool
* [docker](https://www.docker.com/) - Containerizer
* [docker-compose](https://docs.docker.com/compose/install/) - Containerizer orchestrator

## Contributing

Contributing is disabled for this project.

## Versioning

@TODO: Integrate automatic versioning

## Authors

* **James Daniel Holby**

## License

This project is licensed under the WTFPL - see the [LICENSE.md](LICENSE.md) file for details

## Acknowledgments

* [PurpleBooth](https://gist.github.com/PurpleBooth/109311bb0361f32d87a2) - README Template
* [Coda.io](https://coda.io/welcome) - inspiration

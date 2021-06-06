# potku-console

"Minimalistic server auto start and configuration platform."



## Table of content
* [Status of the project](#status-of-the-project)
* [What is potku-console](#what-is-potku-console)
  * [Pitch for the potku-console](#pitch-for-the-potku-console)
  * [Main (upcoming) features](#main-(upcoming)-features)
* [Installation instructions](#installation-instructions)
* [Technologies and architecture](#technologies-and-architecture)
* [Getting involved and contributions](#getting-involved-and-contributions)



## Status of the project

This is currently very much a WIP-project. Core team is currently working on basic architecture and documentation of the project.



## What is potku-console

Potku-console is a service control platform that is used for starting services automatically on demand. Potku-console consists of different kinds of modules: the core, frontend, service modules and cloud modules that together form the potku-console. This repository concentrates especially on core and frontend parts, because they form the spine of the potku-console, but most important first party service and cloud modules are developed in this repository as well.



### Pitch for the potku-console

You would like to own an (insert game name here) server where you and your group of Discord friends could play together. You would like to have a server with multiple cores and a decent amount of RAM from a cloud provider, because the server software is resource hungry and your 10 year old WWW-server computer isn't quite powerful enough. You look for a good hosting solution, but can't find anything that would fulfill your needs due to a tight budget. You ponder that you could save a ton of money in the cloud expenses, if you turn off the server machine when there are no active players on the server. At first the idea felt good, but after a minute you figure out that manually controlling the server's power would be impractical, since some of your friends live in different time zones and they like to play on the server even after you have gone to sleep. If only there was an easy way to automate this power cycling process...

Yes there is a solution, it is called potku-console. Potku-console automatically monitors player counts on the server and shuts it down after the last player has left the game. After the server has been shutdown potku-console sets a guard for the server's address that waits for new players and automatically starts up the server when the first player arrives. This arrangement saves a ton of money and natural resources.  



### Main (upcoming) features

* Automatic startup and shutdown for servers
* Automatic installation and configuration of the servers
* Low CPU and RAM overhead
* Support for multiple cloud providers
* Modular design that enables great extensibility



## Installation instructions

*TODO installation instructions*



## Technologies and architecture

Potku-console uses the following technologies:
* Rust as a main programming language for core and modules
* Frontend language and technologies yet to be decided
* [OpenAPI](https://swagger.io/specification/) for API design and REST principles
* [SQLite](https://www.sqlite.org/index.html) as a database
* [Docker](https://www.docker.com/) for deployment

More information about [potku-console architecture](https://github.com/softgitron/potku-console/wiki/Components-overview) can be found from the project's wiki.



## Getting involved and contributions

Awesome to see that you are interested in making this project even better! We are open for all improvements and contributions. See [Contributing wiki-page](https://github.com/softgitron/potku-console/wiki/Contributing) for basic instructions.
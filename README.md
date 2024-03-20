# Fishhead Labs Submission Wrangler

Manage submissions for a veterinary pathologist.

## Code Organization

### Client

The client is the web interface. It is a React style single page app using Yew and compiled into
WebAssembly.

- Assets - Static files that are copied over into production without modification (raw JS, CSS,
  images, etc)
- Components - Reusable widgets that may be exported as an external project at some point.
- Pages -

### Server

This is a backend that handles creating aggregate data, viewing the local file system, and other
tools that are not available in a web browser sandbox.

### Common

Items that are shared between the client and server, such as the data model.

### Src-Tauri

This contains the glue code for packaging the Server and UI into an installable package.

### Libraries

Rust libraries/macros that are being incubated for becoming stand-alone projects. Each should use
[submodules|https://github.blog/2016-02-01-working-with-submodules/] once they are spun off to their own repo. As Rust is a young language, having local forks of Trunk for some of the projects used
can be useful to drive PRs for items not owned by the Process Foundry.

- Grapht: An in-memory graph database that can be queried using the OpenCypher/GQL syntax
- AllWhat: An enhanced Result monad toolkit with macros

## Windows Setup

Install docker desktop and run [neo4j](https://neo4j.com/docs/operations-manual/current/docker/introduction/)

```shell
docker run \
 --restart always \
 --publish=7474:7474 --publish=7687:7687 \
 --env NEO4J_AUTH=neo4j/your_password \
 --volume=/path/to/your/data:/data \
 --volume=/path/to/your/logs:/logs \
 neo4j:5.9.0
```

## TODO

Now:

- Create base app
  - Tauri: Hello world app
  - common: make organization struct
  - yew: render basic organization from common
- Install Twind with Tailwind syntax
- Make Button
  - Send Organization Object to Tauri Server
  - Tauri runs create Organization
  - The client receives ack/nack
- Add Sub-Organization, adding both parent and child edges
- Add an organization config object
  - Bill to Parent|Self
  - Terms
- Add a billing address to the organization
- Make a table listing all root organizations
- Update the table to show the children in a collapsible window

Later:

- Possible Traits
  - Renderable: Has an associated view associated with it
  - Tabular: Built-in table view for browsing the item
  - Editable: Built-in edit view
- Edit component - popup window for editing embedded objects. It should be able to use breadcrumbs
  when editing complex objects (eg. add a new address/contact from the edit page)

- NEO4J instance Password: BPcl3-UgbrW3FM9-Ovn7pCe0XcNwe1u4YqZ4cTIcBXs

# Fishhead Labs Submission Wrangler

Manage submissions for a veterinary pathologist.

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
- Run neo4j database in docker
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
  in order to edit complex objects (eg. add a new address/contact from the edit page)

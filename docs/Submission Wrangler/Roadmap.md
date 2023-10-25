# Roadmap

This is the ToDo list for getting to being able to minimally replace the existing invoicing system.

- [ ] Walking Skeleton
	- [ ] Manual setup of Docker Neo4j DB
		- Ephemeral for tests
		- Persistent for work
		- Ensure that can connect to prod
	- [x] Add submodules  to integrate existing code
	- [ ] Export Submissions and Organizations into local files
	- [ ] Make end to end test: Client -> Server -> DB -> Server -> Client
		- [ ] Create a simple server workspace singleton with a db connection pool
		- [ ] Make some simple API endpoints to receive calls from a client
			- [ ] Get Orgs (action: `ListAll<Orgs>`)
			- [ ] Import Orgs (action: `Import<Orgs>`)
			- Make a test which assumes the a local db is running
			- Should not involve Tauri yet, only the server
		- [ ] Add the glue to involve Tauri and expose the endpoints (single vs API?)
		- [ ] Make a page which queries all the orgs in the db and prints them
		- [ ] Add a button to call Import function which reads the Org data and inserts them into the database
			- Call API with hard-coded path to local orgs, no selector yet
			- Unique GUID, pretty name and full name only
			- Nodes only, no relationships
			- Return final stats of the insert
			- Update the rendered list of orgs
		- [ ] 
	- [ ] CI/CD Pipeline
		- [ ] Find out how to make a deployable executable
		- [ ] Write instructions for setting up executable in windows pointing to an existing DB
		- [ ] Look at GitLab CI/CD to figure out if I can store some basic artifacts for free
			- [x] Install gitlab-runner locally
			- [ ] Make simple pipeline that builds tests and optionally packages the tauri executable
	- [ ] Make clean Windows 11 VM to test installation process against prod DB
	- [ ] Acceptance Critera
		- End to end test works from compiled Tauri app
- [ ] Get allwhat working in Common::Errors
- [ ] Decide on logging/intrumentation package and add a location for it
	- [ ] Make a test buffer for testing
	- [ ] Add a Logger to the workspace singleton
- [ ] Add testing code coverage: https://blog.rng0.io/how-to-do-code-coverage-in-rust

## Notes

- Some ideas for how to build out the project metadata: https://github.com/rusty-ferris-club/rust-starter/

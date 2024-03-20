#! /usr/bin/env zsh

# Get the root folder of the repo
export WORKDIR=${0:a:h}

export RUST_BACKTRACE=1;
export RUST_LOG=info,test=debug;
export DOCKER_COMPOSE=/usr/bin/local/docker-compose

# Start a new Neo4j container
function reset_db {
   $DOCKER_COMPOSE -f $WORKDIR/scripts/docker/dev.docker-compose.yml down graph_db
   $DOCKER_COMPOSE -f $WORKDIR/scripts/docker/dev.docker-compose.yml up graph_db
}

function get_tailwind {
  cd $WORKDIR/bin
  curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
  mv tailwindcss-linux-x64 tailwindcss
  chmod +x tailwindcss
}


function init {
  echo "Running initialization"
  # echo "Running docker compose initialization"
  # make dockerDev
  $DOCKER_COMPOSE -f $WORKDIR/scripts/docker/dev.docker-compose.yml up -d --remove-orphans graph_db

  # This can vary based on Docker's configuration. I find the proper IP from:
  # > docker inspect docker_postgres_1 | grep 'IPAddress": "172'
  DB_IP=$(\
    docker inspect docker-graph_db-1 \
    | grep 'IPAddress"' \
    | perl -ane '/"IPAddress": "([\d\.]+)",/ ? print "$1" : print ""' \
  )

  echo "Database is listening on: ${DB_IP}"
}


# Remove all the docker containers before exiting
function tearDown {
  echo "All done, tearing down"
  #/usr/bin/docker-compose -f scripts/docker/dev.docker-compose.yml down
}


# Initialize items like docker compose
init
space=" "
modify="${space}MODIFY${space}"

while true; do
  # Execute the test every time
  cargo test -p wrangler-server --test happy_path

  command -v inotifywait > /dev/null 2>&1 || $(echo -e "InotifyWait not installed" && exit 1)
  EVENT=$(inotifywait -r -e modify ./watcher.sh ./Cargo.toml ./server ./common)

  FILE_PATH=${EVENT/${modify}/}
  # echo -e "\nReceived event on file: '${FILE_PATH}'"

  # Root cases
  if [[ $FILE_PATH =~ "watcher.sh" ]]; then
    echo "Matched Watcher.sh. Exiting so we can restart"
    tearDown
    sleep 1
    exit 0

 elif [[ $FILE_PATH =~ "^./Cargo.toml$" ]]; then
    rebuild_invoicer

  elif [[ $FILE_PATH =~ ".+.rs$" ]]; then
    rebuild_invoicer

  else
    echo -en "No Match on '${FILE_PATH}'': Continuing"

  fi
done

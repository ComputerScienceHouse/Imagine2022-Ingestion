## Using the Dockerfile

Build the container with `podman build . --tag=imagine2022-ingestion`

Then, spin up a MongoDB. Could be a container, or some other deployment (I tested with a container). Use `run_mongo.sh`.

To add credentials, do the following:
```
use develop;
db.createUser(
  {
    user: "imaginerit",
    pwd:  "ligma123",
    roles: [ { role: "dbOwner", db: "develop" } ]
  }
);
```

Write a `MONGO_URL` into a `.env` file.

```
echo 'MONGO_URL=mongodb://some_mongo' > .env
```

Then, run the container with this command:
```
podman run --rm -d --name=imagine2022-ingestion -p 8080:8080 --env-file=.env imagine2022-ingestion
```

The container should now be running.

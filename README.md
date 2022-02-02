## Using the Dockerfile

Build the container with `podman build . --tag=imagine2022-ingestion`

Then, spin up a MongoDB. Could be a container, or some other deployment (I tested with a container).

Write a `MONGO_URL` into a `.env` file.

```
echo 'MONGO_URL=mongodb://some_mongo' > .env
```

Then, run the container with this command:
```
podman run --rm -d --name=imagine2022-ingestion --network some-network --env-file=.env imagine2022-ingestion
```

The container should now be running.

# qpp-agent

## How to run
We provide a Dockerfile to run the server. To build the image, please use the following command:
```bash
docker build -t qpp-backend:20240308 -f Dockerfile .
```

To run the qpp-backend, please use the following command:
```bash
docker run -d --network=host --name=qpp-backend-rust --restart=always qpp-backend:20240308
```

Then, you can use `emulate-client` to submit jobs to the server.
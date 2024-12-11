# async-runtime-sgx
Test and demonstrate async runtime in a SGX Enclave

## Build
```
cp default.env .env
docker compose up -d --build
```

Use your container name to attach to it.
```
docker attach async-runtime-container
make
```

## Test
Run the app in the container to test the tokio runtime in the enclave.
```
cd bin
./app
```
You will expect to see the following output:
1. Time taken: 100-300ms for 1 request
2. Time taken: 100-300ms for 8 requests

The result is not 100% accurate, but it should give you a good idea of the performance.
The time taken is the time it takes to spawn 8 tasks and wait for them to complete.
This result suggests that the tokio runtime is able to handle 8 or more I/O trusted tasks concurrently in the enclave.
# RustWebServer
Run on your local:
```
cargo run
```

For optimal performance:

1) Build the executable
```
cargo build --release
```
2) Run the executable
```
./target/release/web_server_is_prime 
```

Some examples of using the executable:

Python `requests`
```
In [1]: import requests

In [2]: temp = requests.get("http://localhost:8088/is_prime", params="100")

In [3]: temp.content
Out[3]: 'false'

```

Linux shell: 
```
curl -X GET http://localhost:8088/is_prime?101
```

In the browser:
```
http://localhost:8088/is_prime?101
```



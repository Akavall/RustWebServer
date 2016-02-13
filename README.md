# RustWebServer
Run on your local:
```
cargo run
```

For optimal performace:
1) Build the executable
```
cargo build --release
```
2) Run the executable
```
./target/release/web_server_is_prime 
```

Some examples of using the executabe:

Python `requests`
```
In [11]: import requests

In [12]: temp = requests.post("http://localhost:8088/is_prime", "101")

In [13]: temp.content
Out[13]: 'true'
```

Linux shell: 
```
curl -X POST -d "101" http://localhost:8088/is_prime
```



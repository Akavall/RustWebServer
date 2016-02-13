# RustWebServer
Run on your local:
```
cargo run
```

For optimul performace:
1) Build the executable
```
cargo build --release
```
2) Run the executable
```
./target/release/web_server_is_prime 
```

Once the exetable is running you can use it with:
```
In [11]: import requests

In [12]: temp = requests.post("http://localhost:8088/is_prime", "101")

In [13]: temp.content
Out[13]: 'true'
```

or 
```
curl -X POST -d "101" http://localhost:8088/is_prime
```



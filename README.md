# Example Automatic swagger documentation using utoipa [utoipa](https://github.com/juhaku/utoipa)
example of:
- json request body
- params
- response schemas

## run using docker
1. build image ```docker build -t rust-api .```
2. run container ```docker run --name rust-api -d -p 5009:8080 rust-api```
3. open your browser ```http://localhost:5009/swagger-ui/```
4. using curl ```
curl -X 'POST' \
  'http://localhost:5009/api/api2/hello/mark?phone_numbers=+62891092,+629059043,+6293050' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
  "birth_date": "1999-01-02"
}'
```

## run using cargo
1. run ```cargo build --release```
2. execute app ```./target/release/ner```
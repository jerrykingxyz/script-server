// POST https://localhost:8080/saveBookDetails
// BODY: {"id": "123", "name": "book1", "year": "2020"}

struct Request {
    token: String,
    function: String,
}

struct Response {}

# url2md  

![GitHub repo size](https://img.shields.io/github/repo-size/0yik/url2md)  
![GitHub stars](https://img.shields.io/github/stars/0yik/url2md?style=social)  
![GitHub forks](https://img.shields.io/github/forks/0yik/url2md?style=social)  

`url2md` is a Rust-based application that fetches HTML content from a given URL and converts it into Markdown format. This project leverages the `axum` framework for building web applications and `reqwest` for making HTTP requests.

## Features  
- Fetch HTML content from any URL.  
- Convert fetched HTML into Markdown format.  
- Handle URL encoding and decoding.  
- Simple API for conversion via HTTP requests.

## Getting Started  

### Prerequisites  
- Rust (1.56 or later)  
- Cargo

### Installation  
Clone the repository:

```bash  
git clone https://github.com/0yik/url2md.git  
cd url2md  
```

Install dependencies:

```bash  
cargo build  
```

### Usage  
To run the server, execute:

```bash  
cargo run  
```

You can convert a URL to Markdown by sending a GET request to the server:

```bash  
curl http://localhost:3000/https://example.com  
```

The server will respond with the Markdown representation of the HTML content fetched from the specified URL.

### Testing  
To run the tests, use:

```bash  
cargo test  
```

This will execute all unit tests defined in the project.

## API Endpoints  

### Convert URL  
- **Endpoint**: `GET /{url}`  
- **Description**: Converts the HTML content of the specified URL to Markdown.  
- **Response**: Returns the Markdown content with a `200 OK` status. If the URL is invalid, it returns a `400 Bad Request`.

### Example  

```bash  
curl http://localhost:3000/https://example.com  
```

## Contributing  
Contributions are welcome! Please open an issue or submit a pull request for any enhancements or bug fixes.

## License  
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgements  
- [axum](https://docs.rs/axum/latest/axum/)  
- [reqwest](https://docs.rs/reqwest/latest/reqwest/)  
- [serde](https://serde.rs/)  
- [url](https://docs.rs/url/latest/url/)  

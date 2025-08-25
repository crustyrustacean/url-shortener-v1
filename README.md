# URL Shortener

A high-performance URL shortener service built with modern Rust technologies. This service provides a simple API for creating shortened URLs and redirecting users to their original destinations.

## 🚀 Features

- **Fast URL shortening**: Generate short, unique identifiers for long URLs
- **Reliable redirects**: Permanent redirects to original URLs with proper HTTP status codes
- **PostgreSQL storage**: Persistent storage with database migrations
- **Comprehensive logging**: Structured logging with tracing and request IDs
- **Health monitoring**: Built-in health check endpoint
- **Production ready**: Deployed on Shuttle with HTTPS support

## 🛠 Technology Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum) - Modern async web framework
- **Database**: PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx) for type-safe queries
- **Deployment**: [Shuttle](https://shuttle.dev) - Serverless Rust deployment platform
- **Logging**: Structured logging with `tracing` and Bunyan formatting
- **Testing**: Comprehensive integration tests with testcontainers

## 📡 API Endpoints

### Shorten a URL
```bash
POST /
Content-Type: text/plain

# Example
curl -d 'https://shuttle.dev/' https://your-domain.com/
```

**Response**: Returns the shortened URL
```
https://your-domain.com/AbC123
```

### Redirect to Original URL
```bash
GET /{id}

# Example
curl -L https://your-domain.com/AbC123
```

**Response**: HTTP 308 Permanent Redirect to the original URL

### Health Check
```bash
GET /health_check

# Example
curl https://your-domain.com/health_check
```

**Response**: HTTP 200 OK with empty body

## 🚀 Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- [Shuttle CLI](https://docs.shuttle.dev/getting-started/installation)
- PostgreSQL (for local development)

### Local Development

1. **Clone the repository**
   ```bash
   git clone https://github.com/sentinel1909/url-shortener-v1.git
   cd url-shortener-v1
   ```

2. **Install dependencies**
   ```bash
   cargo build
   ```

3. **Set up the database**
   ```bash
   # The migrations will run automatically when starting the application
   # Database schema is located in /migrations
   ```

4. **Run locally with Shuttle**
   ```bash
   shuttle run
   ```

5. **Test the service**
   ```bash
   # Shorten a URL
   curl -d 'https://example.com' http://localhost:8000/
   
   # Visit the shortened URL
   curl -L http://localhost:8000/AbC123
   ```

### Deployment

1. **Login to Shuttle**
   ```bash
   shuttle login
   ```

2. **Deploy**
   ```bash
   shuttle deploy
   ```

## 🧪 Testing

The project includes comprehensive integration tests using testcontainers for database testing.

```bash
# Run all tests
cargo test

# Run tests with logging output
TEST_LOG=1 cargo test

# Run specific test module
cargo test health_check
```

### Test Coverage
- ✅ Health check endpoint
- ✅ URL shortening functionality
- ✅ URL redirection
- ✅ Database integration
- ✅ Error handling

## 🏗 Project Structure

```
src/
├── bin/
│   └── main.rs              # Application entry point
└── lib/
    ├── lib.rs               # Library crate root
    ├── startup.rs           # Application configuration
    ├── telemetry.rs         # Logging and tracing setup
    └── routes/
        ├── mod.rs           # Route module exports
        ├── health_check.rs  # Health check handler
        ├── shorten.rs       # URL shortening handler
        └── redirect.rs      # URL redirect handler

tests/
└── api/
    ├── main.rs              # Integration test entry
    ├── helpers.rs           # Test utilities and setup
    ├── health_check.rs      # Health check tests
    ├── shorten.rs           # URL shortening tests
    └── redirect.rs          # URL redirect tests

migrations/
├── 20250106035925_url-shortener.up.sql    # Database schema
└── 20250106035925_url-shortener.down.sql  # Rollback migration
```

## 🔧 Configuration

The application uses environment-based configuration:

- **Database**: Automatically provisioned by Shuttle
- **Logging**: Configurable via `RUST_LOG` environment variable
- **Port**: Automatically assigned by Shuttle

## 📊 Database Schema

```sql
CREATE TABLE urls (
  id VARCHAR(6) PRIMARY KEY,    -- Short identifier (nanoid)
  url VARCHAR NOT NULL          -- Original URL
);
```

## 🔍 Monitoring & Observability

- **Structured Logging**: JSON-formatted logs with request correlation IDs
- **Request Tracing**: Full request lifecycle tracing
- **Health Checks**: `/health_check` endpoint for uptime monitoring
- **Error Handling**: Comprehensive error responses with appropriate HTTP status codes

## 🚧 Roadmap

- [ ] Web UI with Tera templates
- [ ] User authentication and URL management
- [ ] Analytics and usage statistics
- [ ] Custom short URL aliases
- [ ] URL expiration and cleanup
- [ ] Rate limiting
- [ ] Status page with BetterStack integration

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [License.txt](License.txt) file for details.

## 👤 Author

**Jeffery D. Mitchell**
- Email: crusty.rustacean@gmail.com
- GitHub: [@crustyrustacean](https://github.com/crustyrustacean)

## 🙏 Acknowledgments

- Built with [Shuttle](https://shuttle.dev) - Amazing Rust deployment platform
- Inspired by the [Shuttle URL Shortener Tutorial](https://docs.shuttle.dev/templates/tutorials/url-shortener)

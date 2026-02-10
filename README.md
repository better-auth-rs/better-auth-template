# better-auth-template

A [cargo-generate](https://github.com/cargo-generate/cargo-generate) template for [better-auth-rs](https://github.com/better-auth-rs/better-auth-rs) projects.

## Usage

```bash
cargo generate better-auth-rs/better-auth-template
```

You will be prompted to choose a database adapter:

| Adapter | Description |
|---------|-------------|
| `memory` | In-memory (default), for development and prototyping |
| `postgres` | PostgreSQL via SQLx, for production |

## Generated Endpoints

| Method | Path | Description |
|--------|------|-------------|
| POST | `/auth/sign-up/email` | Sign up with email/password |
| POST | `/auth/sign-in/email` | Sign in with email/password |
| GET | `/auth/get-session` | Get current session |
| POST | `/auth/sign-out` | Sign out |
| GET | `/auth/list-sessions` | List all sessions |
| POST | `/auth/forget-password` | Request password reset |
| POST | `/auth/reset-password` | Reset password with token |
| POST | `/auth/change-password` | Change password (auth required) |

# {{project-name}}

Built with [better-auth-rs](https://github.com/better-auth-rs/better-auth-rs).

## Getting Started
{% if database == "postgres" %}
```bash
cp .env.example .env
# edit .env with your database URL
cargo run
```
{% else %}
```bash
cargo run
```
{% endif %}
The server starts at `http://localhost:3000`.

## Auth Endpoints

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

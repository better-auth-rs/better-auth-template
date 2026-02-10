# better-auth-template

A [cargo-generate](https://github.com/cargo-generate/cargo-generate) template for [better-auth-rs](https://github.com/better-auth-rs/better-auth-rs) projects.

## Usage

```bash
cargo generate better-auth-rs/better-auth-template
```

You will be prompted to choose:

- **Database adapter**: `memory` (in-memory, for dev/prototyping) or `postgres` (PostgreSQL via SQLx)

### PostgreSQL

If you chose `postgres`, create a `.env` file from the example and run migrations before starting:

```bash
cp .env.example .env
# edit .env with your database URL
cargo run
```

### Memory

If you chose `memory`, just run:

```bash
cargo run
```

## What's Included

- Axum web server on port 3000
- Email/password authentication (`/auth/sign-up/email`, `/auth/sign-in/email`)
- Session management (`/auth/get-session`, `/auth/sign-out`, `/auth/list-sessions`)
- Password management (`/auth/forget-password`, `/auth/reset-password`, `/auth/change-password`)

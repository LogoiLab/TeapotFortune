## Build Stage
# Pull base image and update
FROM rust:latest AS backend-build

USER root

RUN update-ca-certificates

ENV TZ=America/New_York
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

# Create app user
ARG USER=backend
ARG UID=10001

ENV USER=$USER
ENV UID=$UID

RUN adduser \
	--disabled-password \
	--gecos "" \
	--home "/nonexistent" \
	--shell "/sbin/nologin" \
	--no-create-home \
	--uid "${UID}" \
	"${USER}"

WORKDIR /app

COPY ./src ./src
COPY ./data/copypastas.sqlite ./data/copypastas.sqlite
COPY ./Cargo.lock .
COPY ./Cargo.toml .

RUN chown -R "${USER}":"${USER}" /app

# Build app

ENV DATABASE_URL="sqlite://./data/copypastas.sqlite"

RUN cargo build --release

FROM debian:bookworm AS final

ARG USER=backend
ARG UID=10001

ENV USER=$USER
ENV UID=$UID

ENV DEBIAN_FRONTEND=noninteractive

RUN rm -rf /var/lib/apt/lists/*

# Import from backend-build.
COPY --from=backend-build /etc/passwd /etc/passwd
COPY --from=backend-build /etc/group /etc/group
COPY --from=backend-build /app/data/copypastas.sqlite /app/data/copypastas.sqlite

WORKDIR /app

# Copy our build
COPY --from=backend-build /app/target/release/teapot_fortune /app/teapot_fortune

RUN chown -R "${USER}":"${USER}" /app

USER $USER:$USER

# Expose web http port
EXPOSE 6757

ENTRYPOINT ["/app/teapot_fortune"]

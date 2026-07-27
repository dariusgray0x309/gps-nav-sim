FROM rust:1.85 AS builder

WORKDIR /app
COPY . .

RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Copy from the build image to the runtime image
COPY --from=builder /app/target/release/gps_node /usr/local/bin/gps_node
COPY --from=builder /app/target/release/vehicle_node /usr/local/bin/vehicle_node
COPY --from=builder /app/target/release/sat_node /usr/local/bin/sat_node

CMD [ "/usr/local/bin/gps_node" ]
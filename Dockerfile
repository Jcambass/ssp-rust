FROM busybox:1.35

LABEL org.opencontainers.image.source=https://github.com/jcambass/ssp-rust
LABEL org.opencontainers.image.description="SSP in Rust"

# Create a non-root user to own the files and run our server
RUN adduser -D static
USER static
WORKDIR /home/static

# Copy the static website
# Use the .dockerignore file to control what ends up inside the image!
COPY dist/ .

# Copy httpd conf
COPY httpd.conf .

# Run BusyBox httpd
CMD ["busybox", "httpd", "-f", "-v", "-p", "5000"]
FROM rust:1 
SHELL ["/bin/bash", "-c"]

# ENV PATH=/root/.nvm/versions/node/v22.16.0/bin/:$PATH
ENV TAILWINDCSS_VERSION=3.4.17


### Install tailwindcss CLI
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/download/v$TAILWINDCSS_VERSION/tailwindcss-linux-x64\
  && chmod +x tailwindcss-linux-x64\
  && mv tailwindcss-linux-x64 /bin/tailwindcss

###
RUN cargo install cargo-chef dioxus-cli sea-orm-cli && rustup component add rustfmt
WORKDIR /usr/src/app

COPY . .
RUN cargo chef prepare --recipe-path recipe.json
ENTRYPOINT [ "/bin/bash", "./.docker/entrypoint.sh" ]

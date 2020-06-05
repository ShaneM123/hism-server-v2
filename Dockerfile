FROM rust:latest
MAINTAINER shane moloney
WORKDIR /var/stuff/app
COPY . var/stuff/app
RUN apt-get update
RUN apt-get install -y curl apt-utils
RUN apt-get update
RUN apt-get install build-essential
RUN apt-get install -y tcl
RUN wget http://download.redis.io/releases/redis-stable.tar.gz 
RUN tar xzf redis-stable.tar.gz
RUN cd redis-stable && make
RUN cd redis-stable && make test
RUN cd redis-stable && make install 
RUN apt-get install sqlite3
RUN cargo install diesel_cli --no-default-features --features sqlite
RUN cd var/stuff/app/ && diesel migration run
RUN cd var/stuff/app/ && cargo build
RUN cd var/stuff/app/ && chmod +x startserver.sh
EXPOSE 8443
CMD cd var/stuff/app/ && ./startserver.sh

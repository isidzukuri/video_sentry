# TODO: fix dlib compilation

FROM rust:1.67

ENV APP_HOME /home/video_sentry


RUN apt-get update -qq && apt-get install -y build-essential cmake pkg-config libx11-dev libatlas-base-dev libgtk-3-dev libboost-python-dev
RUN apt-get install -y python-dev python3-dev python3-pip

RUN pip install dlib

WORKDIR $APP_HOME

COPY . .

# SHELL ["/bin/bash", "--login" , "-c"]
RUN cargo build
# SHELL ["/bin/sh", "-c"]

FROM ubuntu:18.04

LABEL maintainer="wshinohara@gmail.com"
LABEL description="running environment for Overlay Token on Substrate."

ARG PROFILE=release

RUN apt-get update && \
	apt-get upgrade -y && \
	apt-get install -y libssl1.0.0 libssl-dev

EXPOSE 30333 9933 9944
ENTRYPOINT ["bash"]
FROM ubuntu

RUN apt update && apt upgrade -y

RUN apt install -y \
	wget \
	make \
	gcc \
	libglib2.0-dev \
	python3-dev

RUN wget https://ftp.gnu.org/gnu/gnubg/gnubg-release-1.07.001-sources.tar.gz

RUN tar -xvf gnubg-release-1.07.001-sources.tar.gz

WORKDIR /gnubg-1.07.001 

RUN ./configure && \
	make && \
	make install

ENTRYPOINT gnubg


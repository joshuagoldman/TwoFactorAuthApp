FROM ubuntu/nginx
COPY ./dist /usr/share/nginx/html
#RUN rm /etc/nginx/conf.d/default.conf 
COPY ./default.conf /etc/nginx/conf.d/default.conf
RUN apt update && apt upgrade
RUN apt install bash
RUN apt install -y curl
RUN apt install -y vim


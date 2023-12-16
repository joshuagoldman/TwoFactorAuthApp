FROM nginx
COPY ./dist /usr/share/nginx/html
RUN rm /etc/nginx/conf.d/default.conf 
COPY ./default.conf /etc/nginx/conf.d/default.conf
RUN apt-get install bash
RUN apt-get install curl
RUN apt-get update
RUN apt-get install -y vim


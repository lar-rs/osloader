FROM alpine:3.9

RUN apk update && apk add --no-cache nginx

COPY nginx.conf /etc/nginx/nginx.conf

RUN mkdir -p /usr/share/www/ /run/nginx/ && \
  rm /etc/nginx/conf.d/default.conf

EXPOSE 80

STOPSIGNAL SIGTERM

ENTRYPOINT ["/usr/sbin/nginx", "-g", "daemon off;"]

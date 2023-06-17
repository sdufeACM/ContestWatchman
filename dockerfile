FROM archlinux
LABEL maintainer="mslxl <i@mslxl.com>"
ENV ContestWatchmanVersion 0.0.0
ENV WATCHMAN_DATABASE /cw/contest.db

# RUN yes | pacman -Sy gmp ca-certificates

COPY target/release/ContestWatchman /usr/bin/cw

RUN mkdir -p /cw

EXPOSE 3000
EXPOSE 3000/udp

CMD /usr/bin/cw serve 1>/cw/verbose.log 2>/cw/error.log
FROM archlinux
LABEL maintainer="mslxl <i@mslxl.com>"
ENV ContestWatchmanVersion 0.0.0

# RUN yes | pacman -Sy gmp ca-certificates

COPY bin/ContestWatchman /usr/bin/cw

RUN mkdir -p /cw

EXPOSE 3000
EXPOSE 3000/udp

CMD /usr/bin/cw 1>/cw/verbose.log 2>/cw/error.log
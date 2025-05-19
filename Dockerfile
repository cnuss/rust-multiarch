FROM scratch
COPY ./dist/rust-multiarch /bin
ENTRYPOINT [ "rust-multiarch" ]

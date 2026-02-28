cd /root/rust_playground
rm -rf docs/book
mdbook build ./docs
python3 -m http.server 8080 -d /root/rust_playground/docs/book
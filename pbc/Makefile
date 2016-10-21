# Makefile
# just a reminder of the various commands
# inside vim use `!make target` to get colorized output

# variables {{{1
VERBOSE = --verbose
CARGO = cargo
RUSTC = rustc

OPTS = -s -v --exe
OPTS_DEP = $(OPTS) --with-coverage --cover-html


# rustc {{{1
# cargo {{{1
# FIXME: how to obtain colored output using vim `make` or `! make`
# build {{{2
build:
	$(CARGO) build
build-v:
	$(CARGO) build $(VERBOSE)
build-r:
	$(CARGO) build --release
build-rv:
	$(CARGO) build --release $(VERBOSE)
# check {{{2
# NOTE: requires cargo install cargo-check
# NOTE: runs as build but without generating any code
check:
	$(CARGO) check
# clean {{{2
clean:
	$(CARGO) clean
clean-v:
	$(CARGO) clean $(VERBOSE)
# test {{{2
test:
	$(CARGO) test
test-v:
	$(CARGO) test $(VERBOSE)
test-nc:
	$(CARGO) test -- --nocapture
# doc {{{2
doc:
	$(CARGO) doc
doc-o:
	$(CARGO) doc --open
# run {{{2
run:
	$(CARGO) run
run-bls:
	$(CARGO) run --example bls
# lint {{{2
clippy:
	# NOTE: requires nightly and cargo install clippy
	$(CARGO) clippy
fmt:
	# NOTE: requires rustfmt, overwrites
	$(CARGO) fmt -- --write-mode=overwrite
fmt-d:
	# NOTE: requires rustfmt, diff
	$(CARGO) fmt -- --write-mode=diff


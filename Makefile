.PHONY: help
help:
	@echo -e "List of make targets:\n"
	@grep "^\.PHONY: " Makefile | cut -d" " -f2- | sed -e "s/ /\n/g"

.PHONY: run
run:
	cargo run

.PHONY: report
report:
	make -C report
